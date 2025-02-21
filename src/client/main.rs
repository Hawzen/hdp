use std::env;
use std::io::{self, Read};
use std::net::{IpAddr, SocketAddr};
use std::time::{SystemTime, Duration};

use atty::Stream;
use socket2::{Domain, Protocol, SockAddr, Socket, Type};

// Create an enum of l4 protocols
enum L4Protocol {
    HDP,
    UDP
}

fn main() -> io::Result<()> {
    // IO
    let usage_message = "Usage: client <num-packets> <ip-address> <ip-protocol:if -1 uses all protocols> <l4:hdp/udp>. Reads payload from stdin";
    if atty::is(Stream::Stdin) {
        eprintln!("{}", usage_message);
        std::process::exit(1);
    }
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf)?;
    let num_packets = env::args().nth(1).expect(usage_message).parse::<u32>().expect("The number of packets is invalid or non-existent");
    let dst_ip = env::args().nth(2).expect(usage_message).parse::<IpAddr>().expect("The IP address is invalid or non-existent");
    let ip_protocol = env::args().nth(3).expect(usage_message).parse::<i32>().expect("The IP protocol is invalid or non-existent");
    let l4_protocol = env::args().nth(4).expect(usage_message).parse::<String>().expect("The L4 protocol is invalid or non-existent");

    // // Ready up a socket and send the packet
    // Ok so my idea is to loop through all the protocol numbers from -1 to 256, and on each iteration
    //  i output a markdown row with the protocol number, and the time of sending the packet.
    println!("| Protocol Number | Succeeded (Client) | Time (μs) (Client) | Byte sum (Client) | Failure reason (Client) |");

    let cycle_between_all_protocols = ip_protocol == -1;

    let mut buf2 = Vec::new();
    let word = [1u8];
    for i in 0..=num_packets as i32 {
        let protocol_number = if cycle_between_all_protocols { 
            i % 256 
        } else { 
            ip_protocol 
        };
        
        // If we don't sleep here, the packets will be sent too fast and the server will not be able to keep up
        std::thread::sleep(std::time::Duration::from_millis(200 + (i % 150) as u64));

        // Add one bit to the payload. This allows me to track the server report to the client's payload
        buf2.extend_from_slice(&word);
        let payload = buf2.as_slice();

        let l4_protocol_enum = match l4_protocol.as_str() {
            "hdp" => L4Protocol::HDP,
            "udp" => L4Protocol::UDP,
            _ => unreachable!("The L4 protocol is invalid or non-existent!! OR I just don't support it")
        };    

        match send_packet(
            protocol_number, 
            dst_ip, 
            l4_protocol_enum,
            payload
        ) {
            Ok((time_right_before_sending_packet, byte_sum)) => {
                println!("| {} | 🫡 | {} | {} | - |", protocol_number, time_right_before_sending_packet.as_micros(), byte_sum);
            },
            Err(e) => {
                println!("| {} | 🤯 | - | - | {} |", protocol_number, e);
            }
        }
    }
    Ok(())
}


fn send_packet(protocol_number: i32, dst_ip: IpAddr, l4_protocol: L4Protocol, payload: &[u8]) -> Result<(Duration, u64), Box<dyn std::error::Error>> {
    let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::from(protocol_number)))?;
    socket.set_header_included_v4(false)?;

    let dest = SocketAddr::new(dst_ip, 0);
    let dest_sockaddr = SockAddr::from(dest);

    let packet = match l4_protocol {
        L4Protocol::HDP => build_hdp_packet(&payload),
        L4Protocol::UDP => build_udp_packet(&payload),
    };

    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    socket.send_to(&packet, &dest_sockaddr)?;

    // Return the time and the sum of the bytes in the packet including ip header
    let byte_sum = match l4_protocol {
        L4Protocol::HDP => 20 /* (IP Header) */ + 12 /* (HDP Header) */ + payload.len() as u64,
        L4Protocol::UDP => 20 /* (IP Header) */ + 8 /* (UDP Header) */ + payload.len() as u64,
    };
    Ok((time?, byte_sum))
}
/// Header layout (12 bytes total):
///   - **Source Port (16 bits):** 2 bytes, big-endian.
///   - **Destination Port (16 bits):** 2 bytes, big-endian.
///   - **Timestamp (64 bits):** 8 bytes, high precision (nanoseconds since UNIX epoch), big-endian.
#[allow(dead_code)]
fn build_hdp_packet(payload: &[u8]) -> Vec<u8> {
    let mut packet = Vec::with_capacity(12 + payload.len());

    let src_port: u16 = 420;
    packet.extend_from_slice(&src_port.to_be_bytes());

    let dest_port: u16 = 420;
    packet.extend_from_slice(&dest_port.to_be_bytes());

    let now = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(now) => now,
        Err(err) => {
            eprintln!("Ah shit, imma die: {}", err);
            std::process::exit(1);
        }
    };
    let timestamp = now.as_nanos() as u64;
    packet.extend_from_slice(&timestamp.to_be_bytes());

    packet.extend_from_slice(payload);

    packet
}

#[allow(dead_code)]
fn build_udp_packet(payload: &[u8]) -> Vec<u8> {
    let mut packet = Vec::with_capacity(8 + payload.len());

    let src_port: u16 = 420;
    packet.extend_from_slice(&src_port.to_be_bytes());

    let dest_port: u16 = 420;
    packet.extend_from_slice(&dest_port.to_be_bytes());

    let length = (8 + payload.len()) as u16;
    packet.extend_from_slice(&length.to_be_bytes());

    let checksum: u16 = 0; // Placeholder for checksum
    packet.extend_from_slice(&checksum.to_be_bytes());

    packet.extend_from_slice(payload);

    packet
}