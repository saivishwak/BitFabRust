use std::net::IpAddr;
use std::net::SocketAddr;
use std::str::FromStr;
use tokio::io::AsyncReadExt;
use tokio::io::BufReader;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

async fn handle_connection(mut stream: tokio::net::TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    //let mut buffer = Vec::with_capacity(1024);
    loop {
        let mut buf: [u8; 8192] = [0; 8192];
        let mut line = String::new();
        line.push_str("from server: ");
        let result = buf_reader.read_line(&mut line).await;
        match result {
            Ok(read) => {
                if read == 0 {
                    println!("Closing connection {}", read);
                    break;
                }
                buf_reader.write_all(line.as_bytes()).await;
            }
            Err(_) => {
                //
            }
        }
    }
}

pub struct Server {
    pub address: IpAddr,
    pub port: u16,
}

impl Server {
    pub fn new(address: String, port: u16) -> Self {
        println!("Initializing the P2P server at {} on {}", address, port);
        Self {
            address: IpAddr::from_str(&address).unwrap(),
            port: port,
        }
    }

    pub async fn start(&self) {
        let addr: SocketAddr = SocketAddr::new(self.address, self.port);
        let listener = TcpListener::bind(addr).await.unwrap();

        let handler = tokio::task::spawn(async move {
            loop {
                let stream = listener.accept().await;
                match stream {
                    Ok(res) => {
                        println!("Accepted new connection from {}", res.1.to_string());
                        tokio::task::spawn(handle_connection(res.0));
                    }
                    Err(err) => {
                        println!("Error accepting connection {}", err);
                    }
                }
            }
        });

        let _ = tokio::join!(handler);
    }
}
