use std::fs::File;
use std::io::{self, Read, Write};
use std::net::UdpSocket;

// packet header : seq_num + size + is_last
#[repr(C, packed)] // C means keeping it c style save as set , and packed means no empty things i
// guess
struct PacketHeader {
    seq_num: u32,
    size: u16,
    is_last: u8,
}
const CHUNK_SIZE: usize = 1024;
const HEADER_SIZE: usize = std::mem::size_of::<PacketHeader>();
const MAX_PACKET: usize = HEADER_SIZE + CHUNK_SIZE;

fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0")?;
    socket.connect("127.0.0.1:8080")?;

    println!("File sharing client");
    println!("Enter filename to send (or 'exit' to quit ) : ");
    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let filename = input.trim();

        if filename == "exit" {
            break;
        }

        if filename.is_empty() {
            continue;
        }

        match send_file(&socket, filename) {
            Ok(_) => println!("File sent successfully"),
            Err(e) => println!("Error sending file: {}", e),
        }
        println!("\nEnter next filename : ");
    }
    Ok(())
}

fn send_file(socket: &UdpSocket, filename: &str) -> io::Result<()> {
    // open the file
    let mut file = File::open(filename);

    // send filename first
    let name_bytes = filename.as_bytes();
    socket.send(name_bytes)?;

    // wait for ACK before startinf fiel transfer
    let mut ack_buf = [0u8; 1];
    socket.recv(&mut ack_buf)?;

    // read and senf files in chunks
    let mut seq_num = 0u32;
    let mut chunk_buf = vec![0u8; CHUNK_SIZE];
    loop {
        // read next chunk from file
        let bytes_read = file.read(&mut chunk_buf)?;

        if bytes_read == 0 {
            break; // eof
        }
        let is_last = if bytes_read < CHUNK_SIZE { 1 } else { 0 };

        // builc packet : header + chunk data;
        let mut packet = vec![0u8; HEADER_SIZE + bytes_read];

        packet[0..4].copy_from_slice(&seq_num.to_le_bytes()); // seq numer
        packet[4..6].copy_from_slice(&(bytes_read as u16).to_le_bytes()); // this is size of file i believe
        packet[6] = is_last; // flag telling if last or not
        packet[HEADER_SIZE..].copy_from_slice(&chunk_buf[..bytes_read]);

        // send
        // and
        // wait
        // for
        // ack

        socket.send(&packet)?;
        let mut ack_buf = [0u8; 1];
        socket.recv(&mut ack_buf)?;

        if seq_num % 10 == 0 {
            println!(" sent chunk {}", seq_num);
        }
        seq_num += 1;
        if is_last == 1 {
            break;
        }
    }
    Ok(())
}
