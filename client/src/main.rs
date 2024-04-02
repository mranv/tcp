use std::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_native_tls::TlsConnector;
use native_tls::{Certificate, TlsConnector as NativeTlsConnector};

const SERVER_ADDR: &str = "127.0.0.1:8080";
const CA_CERT_PATH: &str = "ca_cert.pem";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ca_cert = fs::read(CA_CERT_PATH)?;
    let cert = Certificate::from_pem(&ca_cert)?;

    let connector = TlsConnector::from(
        NativeTlsConnector::builder()
            .add_root_certificate(cert.clone())
            .build()?,
    );

    let stream = TcpStream::connect(SERVER_ADDR).await?;
    let domain = SERVER_ADDR.split(':').next().unwrap();
    let mut stream = connector.connect(domain, stream).await?;

    stream.write_all(b"Hello, server!").await?;

    let mut response = Vec::new();
    stream.read_to_end(&mut response).await?;

    println!("Server responded: {}", String::from_utf8_lossy(&response));

    Ok(())
}