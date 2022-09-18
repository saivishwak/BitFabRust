use crate::message;
use crate::message::MessageSuccessStatusCode;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::str::FromStr;
use tokio::io::BufReader;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[derive(Debug)]
pub enum GossipTypes {
    Ping,
    Pong,
    Def,
}

trait FromString {
    fn from_string(input: &String) -> Result<GossipTypes, ()>;
}

impl FromString for GossipTypes {
    fn from_string(input: &String) -> Result<GossipTypes, ()> {
        match input.trim() {
            "ping" => Ok(GossipTypes::Ping),
            "pong" => Ok(GossipTypes::Pong),
            "def" => Ok(GossipTypes::Def),
            _ => {
                return Err(());
            }
        }
    }
}

async fn handle_connection(mut stream: tokio::net::TcpStream) {
    //let mut buffer = Vec::with_capacity(1024);
    let mut buf_reader = BufReader::new(&mut stream);
    loop {
        //let mut buf: [u8; 8192] = [0; 8192];
        /*let message_handler = message::Message::new();
        let result = message_handler.handle(&mut stream).await;
        match result {
            Ok(status) => match status {
                message::MessageSuccessStatusCode::ClosConnection => {
                    println!("Closing connection");
                    break;
                }
                message::MessageSuccessStatusCode::Success => {
                    println!("Succesfully sent msg");
                }
            },
            Err(e) => {
                println!("{}", e);
                break;
            }
        }*/

        let mut line = String::new();
        let result = buf_reader.read_line(&mut line).await;
        match result {
            Ok(read) => {
                if read == 0 {
                    break;
                }
                let gossip_str_res = GossipTypes::from_string(&line);
                match gossip_str_res {
                    Ok(gossip_type) => match gossip_type {
                        GossipTypes::Ping => {
                            println!("Ping");
                            match buf_reader.write_all(String::from("Pong").as_bytes()).await {
                                Ok(_) => {
                                    println!("sent message");
                                }
                                Err(_) => {
                                    println!("err sending message");
                                }
                            }
                        }
                        GossipTypes::Pong => {
                            println!("pong");
                        }
                        GossipTypes::Def => {
                            println!("Def");
                        }
                    },
                    Err(_) => {
                        println!("Error in decoding type");
                    }
                }
            }
            Err(_) => {
                //return Err("Error reading message");
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
