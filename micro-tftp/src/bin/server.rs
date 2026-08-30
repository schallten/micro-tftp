use std::io::{self, Write};
use std::net::UdpSocket;

fn main() -> io::Result<()> {
    // The server listens on port 8080, same as the C version.
    let socket = UdpSocket::bind("0.0.0.0:8080")?;
    println!("Server ready, type 'exit' to quit");

    // A buffer to hold whatever the client sends us.
    let mut buf = [0u8; 100];

    loop {
        // Wait for a message from the client. recv_from gives us the
        // number of bytes and the address the client is sending from.
        let (size, client_addr) = socket.recv_from(&mut buf)?;

        // Turn the raw bytes into a Rust String so we can print it.
        // from_utf8_lossy won't crash if the bytes aren't valid UTF-8.
        let message = String::from_utf8_lossy(&buf[..size]);
        println!("Client ({}): {}", client_addr.ip(), message.trim());

        // If the client said "exit", stop the server.
        if message.trim() == "exit" {
            break;
        }

        // Ask the user (the server person) for a reply.
        print!("Reply: ");
        io::stdout().flush()?; // make "Reply: " show up before we wait for input

        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let reply = line.trim().to_string();

        // Empty reply or "exit" ends the conversation.
        if reply.is_empty() || reply == "exit" {
            break;
        }

        // Send our reply back to the client that just talked to us.
        socket.send_to(reply.as_bytes(), client_addr)?;
    }

    Ok(())
}
