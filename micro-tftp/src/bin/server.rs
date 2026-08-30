use std::fs::File;
use std::io::{self, Read, Write};
use std::net::UdpSocket;

#[repr(C, packed)]
struct PacketHeader {
    seq_num: u32,
    size: u16,
    is_last: u8,
}

const CHUNK_SIZE: usize = 1024;
const HEADER_SIZE: usize = std::mem::size_of::<PacketHeader>();

fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:8080")?;
    println!("File Sharing Server listening on port 8080");

    loop {
        let mut buf = [0u8; 1024];

        // Wait for filename (first message from client)
        let (size, client_addr) = socket.recv_from(&mut buf)?;
        let filename = String::from_utf8_lossy(&buf[..size]);
        println!("\nIncoming file: {} from {}", filename, client_addr.ip());

        // Send ACK to start file transfer
        socket.send_to(&[1], client_addr)?;

        // Receive and save the file
        match receive_file(&socket, client_addr, &filename) {
            Ok(_) => println!("File saved successfully"),
            Err(e) => println!("Error receiving file: {}", e),
        }
    }
}

fn receive_file(
    socket: &UdpSocket,
    client_addr: std::net::SocketAddr,
    filename: &str,
) -> io::Result<()> {
    let mut output = File::create(filename)?;
    let mut expected_seq = 0u32;
    let mut buf = vec![0u8; HEADER_SIZE + CHUNK_SIZE];

    loop {
        let (size, addr) = socket.recv_from(&mut buf)?;

        // Only accept from the expected client
        if addr != client_addr {
            continue;
        }

        if size < HEADER_SIZE {
            continue;
        }

        // Parse header (little-endian)
        let seq_num = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
        let chunk_size = u16::from_le_bytes([buf[4], buf[5]]) as usize;
        let is_last = buf[6];

        // Only process in-order packets
        if seq_num != expected_seq {
            continue;
        }

        // Write chunk to file
        if chunk_size > 0 {
            output.write_all(&buf[HEADER_SIZE..HEADER_SIZE + chunk_size])?;
        }

        // Send ACK
        socket.send_to(&[1], client_addr)?;

        if seq_num % 10 == 0 {
            println!("  Received chunk {}", seq_num);
        }

        if is_last == 1 {
            break;
        }

        expected_seq += 1;
    }

    Ok(())
}
