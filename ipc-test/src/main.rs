use std::net::UdpSocket;
use std::str;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:8001").expect("Failed to bind socket");
    socket.connect("127.0.0.1:8000").expect("Failed to connect to editor");

    // Build the JSON message and append the form feed character to it.
    let mut message: String = r#"{
        "type": "message",
        "data": {
            "from": "127.0.0.1:8001",
            "message": "Hello from Rust!"
        }
    }"#.to_string();
    message.push_str("\u{C}");

    // Send the JSON message.
    socket.send(message.as_bytes());

    let mut buffer = [0; 2048];
    loop {
        let bytes_read = socket.recv(&mut buffer).expect("Failed to recieve bytes");
        let message_bytes = &buffer[..bytes_read];
        let message = str::from_utf8(message_bytes).expect("Message was invalid UTF-8");
        println!("{}", message.trim());
    }
}
