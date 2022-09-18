use crate::message::Message;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use tokio::io::BufReader;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use crate::peer::Peer;
use crate::router;

async fn handle_connection(
    inner_self: Arc<Mutex<Server>>,
    mut stream: tokio::net::TcpStream,
    router: Arc<router::Router>,
) {
    let mut buf_reader = BufReader::new(&mut stream);
    loop {
        let mut line = String::new();
        let result = buf_reader.read_line(&mut line).await;
        match result {
            Ok(read) => {
                if read == 0 {
                    println!("Connection disconnected");
                    break;
                }
                //let gossip_str_res = GossipTypes::from_string(&line);
                let gossip_type_res = Message::unmarshall(&line.trim());
                match gossip_type_res {
                    Ok(message) => {
                        let res_string = router.handle(message, inner_self.clone()).await;
                        match buf_reader.write_all(res_string.as_bytes()).await {
                            Ok(_) => {
                                println!("sent message");
                            }
                            Err(_) => {
                                println!("err sending message");
                            }
                        }
                    }
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
    pub peers: Vec<Peer>,
}

pub struct ServerWrapper {
    pub inner: Arc<Mutex<Server>>,
}

impl ServerWrapper {
    pub fn new(address: String, port: u16) -> Self {
        println!("Initializing the P2P server at {} on {}", address, port);
        let server = Server {
            address: IpAddr::from_str(&address).unwrap(),
            port: port,
            peers: Vec::new(),
        };
        Self {
            inner: Arc::new(Mutex::new(server)),
        }
    }

    pub async fn start(&mut self, router: router::Router) {
        let inner_self = self.inner.clone();
        let server_addr = inner_self.lock().unwrap().address;
        let server_port = inner_self.lock().unwrap().port;
        let addr: SocketAddr = SocketAddr::new(server_addr, server_port);
        let listener = TcpListener::bind(addr).await.unwrap();
        let router_arc = Arc::new(router);

        loop {
            let stream = listener.accept().await;
            match stream {
                Ok(res) => {
                    println!("Accepted new connection from {}", res.1.to_string());
                    tokio::task::spawn({
                        let inner_self = inner_self.clone();
                        let router_arc = router_arc.clone();
                        async move {
                            handle_connection(inner_self, res.0, router_arc.clone()).await;
                        }
                    });
                }
                Err(err) => {
                    println!("Error accepting connection {}", err);
                }
            }
        }
    }
}
