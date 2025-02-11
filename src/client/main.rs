use std::env;
use std::io::{self, Read};
use std::net::{IpAddr, SocketAddr};
use std::time::SystemTime;

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
    let protocol_number = 253;
    let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::from(protocol_number)))?;
    socket.set_header_included_v4(false)?;

    let dest = SocketAddr::new(dst_ip, 0);
    let dest_sockaddr = SockAddr::from(dest);

    // Time 
    let packet = build_hdp_packet(&payload);
    let before_packet_sent = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    socket.send_to(&packet, &dest_sockaddr)?;
    let after_packet_sent = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);

    println!("Packet sent successfully!");
    println!("Time taken to send packet: {:?}", after_packet_sent.unwrap().as_nanos() - before_packet_sent.unwrap().as_nanos());
    Ok(())
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