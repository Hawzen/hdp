use std::env;
use std::io::{self, Read};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

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

    let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::from(253)))?;
    socket.set_header_included_v4(false)?;

    let dest = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 78);
    let dest_sockaddr = SockAddr::from(dest);

    socket.send_to(&payload, &dest_sockaddr)?;

    println!("Packet sent successfully!");
    Ok(())
}