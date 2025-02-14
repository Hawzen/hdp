use std::env;
use std::io::{self, Read};
use std::net::{IpAddr, SocketAddr};
use std::time::{SystemTime, Duration};
use sys_info;
use whoami;

use atty::Stream;
use socket2::{Domain, Protocol, SockAddr, Socket, Type};

fn main() -> io::Result<()> {
    // IO
    let usage_message = "Usage: client <ip-address>. Reads payload from stdin";
    if atty::is(Stream::Stdin) {
        eprintln!("{}", usage_message);
        std::process::exit(1);
    }
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf)?;
    let payload = buf.as_slice();
    let dst_ip = env::args().nth(1).expect(usage_message).parse::<IpAddr>().expect("The IP address is invalid or non-existent");

    // // Ready up a socket and send the packet

    // First, identify the machine that we are running on
    // Print Information
    println!("--- System Info ---");
    println!("OS: {} {}", 
        sys_info::os_type().unwrap_or_else(|_| "Unknown".to_string()), 
        sys_info::os_release().unwrap_or_else(|_| "Unknown".to_string())
    );
    println!("Architecture: {}", whoami::arch());
    println!("-------------------");


    // Ok so my idea is to loop through all the protocol numbers from -1 to 256, and on each iteration
    //  i output a markdown row with the protocol number, and the time of sending the packet.
    println!("| Protocol Number | Succeeded | Time before sending packet (ns) | Failure reason |");
    for protocol_number in -1..=256 {
        match send_packet(protocol_number, dst_ip, payload) {
            Ok(time_right_before_sending_packet) => {
                println!("| {} | 🫡🫡🫡 | {} |", protocol_number, time_right_before_sending_packet.as_nanos());
            },
            Err(e) => {
                // Explode
                println!("| {} | 🤯🤯🤯 | - | {} |", protocol_number, e);
            }
        }
    }
    Ok(())
}


fn send_packet(protocol_number: i32, dst_ip: IpAddr, payload: &[u8]) -> Result<Duration, Box<dyn std::error::Error>> {
    let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::from(protocol_number)))?;
    socket.set_header_included_v4(false)?;

    let dest = SocketAddr::new(dst_ip, 0);
    let dest_sockaddr = SockAddr::from(dest);

    let packet = build_hdp_packet(&payload);
    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    socket.send_to(&packet, &dest_sockaddr)?;

    Ok(time?)
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