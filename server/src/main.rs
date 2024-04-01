use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use rustls::{NoClientAuth, ServerConfig, ServerSession, Stream};
use rustls::internal::pemfile::{certs, pkcs8_private_keys};

const CERTIFICATE: &[u8] = include_bytes!("server_cert.pem");
const PRIVATE_KEY: &[u8] = include_bytes!("server_key.pem");

fn handle_client(mut stream: Stream<TcpStream, ServerSession>) {
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
    let config = load_tls_config();
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind");
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                let config = config.clone();
                std::thread::spawn(move || {
                    let _ = config;
                    let _ = handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn load_tls_config() -> ServerConfig {
    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert = certs(&mut CERTIFICATE.as_ref()).unwrap();
    let key = pkcs8_private_keys(&mut PRIVATE_KEY.as_ref()).unwrap().remove(0);
    config.set_single_cert(cert, key).unwrap();
    config
}
