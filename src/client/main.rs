use std::env;
use std::io::{self, Read};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::SystemTime;

use atty::Stream;
use socket2::{Domain, Protocol, SockAddr, Socket, Type};

fn main() -> io::Result<()> {
    let payload = if let Some(arg) = env::args().nth(1) {
        arg.into_bytes()
    } else {
        if atty::is(Stream::Stdin) {
            eprintln!("Usage: client [data] OR pipe data to standard input");
            std::process::exit(1);
        }
        let mut buf = Vec::new();
        io::stdin().read_to_end(&mut buf)?;
        buf
    };

    let packet = build_packet(&payload);

    let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::from(253)))?;
    socket.set_header_included_v4(false)?;

    let dest = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0);
    let dest_sockaddr = SockAddr::from(dest);

    socket.send_to(&packet, &dest_sockaddr)?;

    println!("Packet sent successfully!");
    Ok(())
}

/// Header layout (12 bytes total):
///   - **Source Port (16 bits):** 2 bytes, big-endian.
///   - **Destination Port (16 bits):** 2 bytes, big-endian.
///   - **Timestamp (64 bits):** 8 bytes, high precision (nanoseconds since UNIX epoch), big-endian.
fn build_packet(payload: &[u8]) -> Vec<u8> {
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