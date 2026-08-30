use std::io::{self, Write};
use std::net::UdpSocket;

fn main() -> io::Result<()> {
    // The client talks to the server on localhost port 8080.
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("127.0.0.1:8080")?;

    println!("Type 'exit' to quit");

    // A buffer to hold whatever the server sends us.
    let mut buf = [0u8; 100];

    loop {
        // Ask the user for something to say.
        print!("Say: ");
        io::stdout().flush()?;

        let mut line = String::new();
        let bytes = io::stdin().read_line(&mut line)?;

        // If stdin is closed (Ctrl-D), quit.
        if bytes == 0 {
            break;
        }

        let message = line.trim().to_string();

        // Empty input or "exit" ends the conversation.
        if message.is_empty() || message == "exit" {
            break;
        }

        // Send our message to the server (we already connected, so send).
        socket.send(message.as_bytes())?;

        // Wait for the server's reply and print it.
        let size = socket.recv(&mut buf)?;
        let reply = String::from_utf8_lossy(&buf[..size]);
        println!("Server: {}", reply.trim());
    }

    Ok(())
}
