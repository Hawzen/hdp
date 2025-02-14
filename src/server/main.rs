#![allow(non_snake_case)]
use socket2::{Domain, Protocol, Socket, Type};
use std::io::{self, Write};
use std::fs::OpenOptions;
use std::net::SocketAddr;
use std::mem::MaybeUninit;
use std::time::SystemTime;
use std::fs::create_dir_all;

fn main() -> io::Result<()> {
    // // Init
    let protocol_number = 0;
    let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::from(protocol_number)))?;

    let bind_addr = SocketAddr::from(([0, 0, 0, 0], 0));
    socket.bind(&bind_addr.into())?;

    let mut buffer: [MaybeUninit<u8>; 65535] = unsafe { MaybeUninit::uninit().assume_init() };

    // This will be relevant later, trust me
    let dir_path = "/tmp/hdp";
    create_dir_all(dir_path)?;


    // // Loop time
    println!("| Protocol Number | Time (μs) (Server) | Source IP (Server) | Byte Sum (Server) |");
    println!("| --- | --- | --- |");

    loop {
        // Receive raw IP packets into `buffer`.
        let (size, _) = socket.recv_from(&mut buffer)?;

        // What's the time?
        let time_right_after_receiving_the_packet = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_micros();

        // Dear sir, this might appear unsafe, but we are only reading the buffer up to `size`
        // So we're not reading uninitialized memory.
        // Let's not bother the uninitialized memories, shall we?
        let buffer = unsafe { &*(buffer.as_ptr() as *const [u8; 65535]) };
        let received_data = &buffer[..size];

        // Let's write it in /tmp/hdp/ folder
        // (Yes, I'm a boogyman who uses unwrap like this)
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(format!("/tmp/hdp/hdp_{time_right_after_receiving_the_packet}.bin"))?;
        file.write_all(received_data)?;

        // // PAAAAARSE
        // IP
        let ip_header = &received_data[..20];
        // let ip__version = ip_header[0] >> 4;
        // let ip__ihl = ip_header[0] & 0b00001111;
        // let ip__dscp = ip_header[1] >> 2;
        // let ip__ecn = ip_header[1] & 0b00000011;
        // let ip__total_length = u16::from_be_bytes([ip_header[2], ip_header[3]]);
        // let ip__identification = u16::from_be_bytes([ip_header[4], ip_header[5]]);
        // let ip__flags = ip_header[6] >> 5;
        // let ip__fragment_offset = u16::from_be_bytes([ip_header[6] & 0b00011111, ip_header[7]]);
        // let ip__ttl = ip_header[8];
        let ip__protocol = ip_header[9];
        // let ip__header_checksum = u16::from_be_bytes([ip_header[10], ip_header[11]]);
        let ip__src_ip = &ip_header[12..16];
        // let ip__dst_ip = &ip_header[16..20];
        // HDP..!!!
        // let hdp__payload = &received_data[20..];
        // let hdp__src_port = u16::from_be_bytes([hdp__payload[0], hdp__payload[1]]);
        // let hdp__dest_port = u16::from_be_bytes([hdp__payload[2], hdp__payload[3]]);
        // let hdp__unix_timestamp = u64::from_be_bytes([
        //     hdp__payload[4], hdp__payload[5], hdp__payload[6], hdp__payload[7],
        //     hdp__payload[8], hdp__payload[9], hdp__payload[10], hdp__payload[11],
        // ]);
        // let hdp__data = &hdp__payload[12..];
        // Let's parse it as UDP packet
        // let udp__header = &received_data[20..];
        // let udp__src_port = u16::from_be_bytes([udp__header[0], udp__header[1]]);
        // let udp__dst_port = u16::from_be_bytes([udp__header[2], udp__header[3]]);
        // let udp__length = u16::from_be_bytes([udp__header[4], udp__header[5]]);
        // let udp__checksum = u16::from_be_bytes([udp__header[6], udp__header[7]]);
        // let udp__data = &udp__header[8..];

        // // Print the received data as a markdown table. Each row is a packet.
        // I need to print the time, source ip, byte sum of ip header + payload
        let formatted_src_ip = format!("{}.{}.{}.{}", ip__src_ip[0], ip__src_ip[1], ip__src_ip[2], ip__src_ip[3]);
        println!(
            "| {} | {} | {} | {} |",
            ip__protocol,
            time_right_after_receiving_the_packet,
            formatted_src_ip,
            size,
        );
    }
}
