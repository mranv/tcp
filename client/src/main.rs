use std::net::TcpStream;
use std::io::{Write, BufRead, BufReader};

const SHARED_SECRET: &str = "secret_token";

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8088").expect("Failed to connect to server");
    println!("Connected to server!");

    stream.write_all(SHARED_SECRET.as_bytes()).expect("Failed to write to server");

    let mut response = String::new();
    let mut reader = BufReader::new(&stream);
    reader.read_line(&mut response).expect("Failed to read response from server");
    
    if response.trim() == "Authenticated" {
        println!("Server authenticated successfully");

        loop {
            let mut buffer = String::new();
            match reader.read_line(&mut buffer) {
                Ok(0) => {
                    println!("Server disconnected");
                    break;
                }
                Ok(_) => {
                    println!("Server: {}", buffer.trim());
                }
                Err(e) => {
                    println!("Error reading from server: {}", e);
                    break;
                }
            }
        }
    } else {
        println!("Server refused authentication");
        return;
    }
}
