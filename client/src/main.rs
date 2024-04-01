use std::io::{Read, Write};
use std::net::TcpStream;
use rustls::{ClientConfig, ClientSession, Stream};
use rustls::internal::pemfile::certs;

const SERVER_ADDR: &str = "127.0.0.1:8080";
const CA_CERTIFICATE: &[u8] = include_bytes!("ca_cert.pem");

fn main() {
    let config = load_tls_config();
    let stream = TcpStream::connect(SERVER_ADDR).expect("Failed to connect to server");
    let hostname = webpki::DNSNameRef::try_from_ascii_str(SERVER_ADDR.split(':').next().unwrap()).unwrap();
    let stream = rustls::ClientSession::new(&config, hostname);
    let mut stream = Stream::new(stream, stream);

    stream.write_all(b"Hello, server!").expect("Failed to write to server");

    let mut response = String::new();
    stream.read_to_string(&mut response).expect("Failed to read response from server");
    println!("Server responded: {}", response);
}

fn load_tls_config() -> ClientConfig {
    let mut config = ClientConfig::new();
    let ca_certs = certs(&mut CA_CERTIFICATE.as_ref()).unwrap();
    config.root_store.add(&ca_certs);
    config
}
