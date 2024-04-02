use std::fs;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_native_tls::{TlsAcceptor, TlsStream};
use native_tls::{Identity, TlsAcceptor as NativeTlsAcceptor};

const SERVER_ADDR: &str = "127.0.0.1:8080";
const SERVER_CERT_PATH: &str = "server_cert.pem";
const SERVER_KEY_PATH: &str = "server_key.pem";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cert_data = fs::read(SERVER_CERT_PATH)?;
    let key_data = fs::read(SERVER_KEY_PATH)?;

    let cert = Identity::from_pkcs8(&cert_data, &key_data)?;

    let acceptor = TlsAcceptor::from(NativeTlsAcceptor::new(Arc::new(cert))?);

    let listener = TcpListener::bind(SERVER_ADDR).await?;
    println!("Server listening on {}", SERVER_ADDR);

    loop {
        let (stream, _) = listener.accept().await?;
        let acceptor = acceptor.clone();

        tokio::spawn(async move {
            let mut stream = acceptor.accept(stream).await.unwrap();
            handle_client(&mut stream).await;
        });
    }
}

async fn handle_client(stream: &mut TlsStream<TcpStream>) {
    let mut buf = vec![0; 1024];

    loop {
        let size = stream.read(&mut buf).await.unwrap();
        if size == 0 {
            break;
        }

        let message = String::from_utf8_lossy(&buf[..size]);
        println!("Received message: {}", message);

        stream.write_all(&buf[..size]).await.unwrap();
    }
}