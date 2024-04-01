use std::net::TcpStream;
use std::io::{Read, Write};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to server");
    println!("Connected to server!");

    let message = "Hello, server!";
    stream.write_all(message.as_bytes()).expect("Failed to write to server");

    let mut response = String::new();
    stream.read_to_string(&mut response).expect("Failed to read response from server");
    println!("Server responded: {}", response);
}

