use socket2::{Domain, Protocol, Socket, Type};
use std::io;
use std::net::SocketAddr;
use std::mem::MaybeUninit;

fn main() -> io::Result<()> {
    let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::from(253)))?;

    let bind_addr = SocketAddr::from(([0, 0, 0, 0], 0));
    socket.bind(&bind_addr.into())?;

    let mut buffer: [MaybeUninit<u8>; 65535] = unsafe { MaybeUninit::uninit().assume_init() };

    println!("Server listening on IP protocol 253 (raw)...");

    loop {
        // Receive raw IP packets into `buffer`.
        let (size, _) = socket.recv_from(&mut buffer)?;

        // Dear sir, this might appear unsafe, but we are only reading the buffer up to `size`
        // So we're not reading uninitialized memory.
        // Let's not bother the uninitialized memories, shall we?
        let buffer = unsafe { &*(buffer.as_ptr() as *const [u8; 65535]) };
        let received_data = &buffer[..size];

        // // Parse IP packet
        // TODO: Why is this unsafe?
        let ip_header = &received_data[..20];
        let version = ip_header[0] >> 4;
        let ihl = ip_header[0] & 0b00001111;
        let dscp = ip_header[1] >> 2;
        let ecn = ip_header[1] & 0b00000011;
        let total_length = u16::from_be_bytes([ip_header[2], ip_header[3]]);
        let identification = u16::from_be_bytes([ip_header[4], ip_header[5]]);
        let flags = ip_header[6] >> 5;
        let fragment_offset = u16::from_be_bytes([ip_header[6] & 0b00011111, ip_header[7]]);
        let ttl = ip_header[8];
        let protocol = ip_header[9];
        let header_checksum = u16::from_be_bytes([ip_header[10], ip_header[11]]);
        let src_ip = &ip_header[12..16];
        let dst_ip = &ip_header[16..20];

        // Print the IP header
        println!("~~~ IP Header ~~~");
        println!("Version: {}", version);
        println!("IHL: {}", ihl);
        println!("DSCP: {}", dscp);
        println!("ECN: {}", ecn);
        println!("Total Length: {}", total_length);
        println!("Identification: {}", identification);
        println!("Flags: {}", flags);
        println!("Fragment Offset: {}", fragment_offset);
        println!("TTL: {}", ttl);
        println!("Protocol: {}", protocol);
        println!("Header Checksum: {}", header_checksum);
        println!("Source IP: {:?}", src_ip);
        println!("Destination IP: {:?}", dst_ip);
        println!("\n");
        
        // Now let's do some HDP..!!!
        let payload = &received_data[20..];
        let src_port = u16::from_be_bytes([payload[0], payload[1]]);
        let dest_port = u16::from_be_bytes([payload[2], payload[3]]);
        let unix_timestamp = u64::from_be_bytes([
            payload[4], payload[5], payload[6], payload[7],
            payload[8], payload[9], payload[10], payload[11],
        ]);
        let data = &payload[12..];

        println!("~~~ HDP Header & Data ~~~");
        println!("Source Port: {}", src_port);
        println!("Destination Port: {}", dest_port);
        println!("Timestamp: {}", unix_timestamp);
        println!("Data: {}", String::from_utf8_lossy(data));
        println!("\n\n");
    }
}
