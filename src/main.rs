use socket2::{Domain, Protocol, Socket, Type};
use std::io;
use std::net::SocketAddr;
use std::mem::MaybeUninit;

fn main() -> io::Result<()> {
    let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::from(253)))?;

    let bind_addr = SocketAddr::from(([0, 0, 0, 0], 0));
    socket.bind(&bind_addr.into())?;

    let mut buffer = [MaybeUninit::uninit(); 65535];

    println!("Server listening on IP protocol 253 (raw)...");

    loop {
        // Receive raw IP packets into `buffer`.
        let (size, src_addr) = socket.recv_from(&mut buffer)?;
        println!("Received {} bytes from {:?}", size, src_addr);

        // // Parse IP packet
        // TODO: Why is this unsafe?
        let ip_header: [u8; 20] = unsafe {
            std::ptr::read(buffer.as_ptr() as *const [u8; 20])
        };
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
        let payload = &buffer[20..size];

        // Print everything
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
        println!("Payload: ");

        // TODO: STOP ADDING THESE UNSAFE BLOCKS AND LEARN HOW TO DO THIS SAFELY :D
        for byte in payload {
            let byte = unsafe { byte.assume_init() };
            print!("{:02x}", byte);
        }
        println!("\n");
        

        
    }
}
