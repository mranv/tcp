use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    while match stream.read(&mut buf) {
        Ok(size) => {
            if size == 0 {
                false
            } else {
                let message = String::from_utf8_lossy(&buf[..size]);
                println!("Received message: {}", message);
                stream.write_all(&buf[..size]).unwrap();
                true
            }
        },
        Err(_) => {
            println!("Error occurred, terminating connection with client");
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind");
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                std::thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
