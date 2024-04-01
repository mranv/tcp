use std::io::{Read, Write};
use std::net::TcpStream;
use rustls::{ClientConfig, ClientSession, Stream};

const SERVER_ADDR: &str = "127.0.0.1:8080";

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
    config.root_store.add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
    config
}
