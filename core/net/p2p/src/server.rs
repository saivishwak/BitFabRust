use crate::message::{GossipTypes, Message};
use crate::peer::Peer;
use crate::router;
use std::io;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::str;
use std::str::FromStr;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::io::Interest;
use tokio::net::TcpListener;
//use tokio::net::TcpSocket;
use crate::peer;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use uuid::Uuid;

async fn handle_connection(
    inner_self: Arc<Mutex<Server>>,
    stream: Arc<Mutex<tokio::net::TcpStream>>,
    router: Arc<router::Router>,
    stream_id: Uuid,
) {
    loop {
        let mut stream_mutex_guard = stream.lock().await;
        let mut buffer = Vec::with_capacity(4096);
        let stream_ready = stream_mutex_guard
            .ready(Interest::READABLE | Interest::WRITABLE)
            .await
            .unwrap();
        if stream_ready.is_readable() {
            let stream_data = stream_mutex_guard.try_read_buf(&mut buffer);
            match stream_data {
                Ok(data) => {
                    if data == 0 {
                        println!("Connection disconnected");
                        break;
                    }
                    println!("Received msg {:?}", str::from_utf8(&buffer));
                    let gossip_type_res = Message::unmarshall(&buffer);
                    match gossip_type_res {
                        Ok(message) => {
                            let res_string =
                                router.handle(message, stream_id, inner_self.clone()).await;
                            if stream_ready.is_writable() {
                                match stream_mutex_guard.write_all(res_string.as_bytes()).await {
                                    Ok(_) => {
                                        println!("Message sent");
                                    }
                                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                        println!("Error in would block write");
                                        continue;
                                    }
                                    Err(e) => {
                                        println!("Error sending message {}", e);
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            println!("Error in decoding type");
                        }
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(_) => {
                    println!("Error reading message");
                    //break;
                }
            }
        }
    }
}

pub struct Server {
    pub address: IpAddr,
    pub port: u16,
    pub peers: Vec<Peer>,
}

impl Server {
    pub fn handle(&self) {
        println!("Hanlde");
    }

    pub async fn connect_to_peer(&self) {
        println!("***** Coonnect to pper");
    }
}

pub struct ServerWrapper {
    pub inner: Arc<Mutex<Server>>,
}

impl ServerWrapper {
    pub fn new(address: String, port: u16) -> Self {
        println!("Initializing the P2P server at {} on {}", address, port);
        let server = Server {
            address: IpAddr::from_str(&address).unwrap(),
            port,
            peers: Vec::new(),
        };
        Self {
            inner: Arc::new(Mutex::new(server)),
        }
    }

    pub async fn start(&self, router: router::Router) {
        let inner_self = self.inner.clone();
        inner_self.lock().await.connect_to_peer().await;
        let server_addr = inner_self.lock().await.address;
        let server_port = inner_self.lock().await.port;
        let addr: SocketAddr = SocketAddr::new(server_addr, server_port);
        let listener = TcpListener::bind(addr).await.unwrap();
        let router_arc = Arc::new(router);

        loop {
            let stream = listener.accept().await;
            match stream {
                Ok(stream_data) => {
                    let stream_data_clone = Arc::new(Mutex::new(stream_data.0));
                    println!("Accepted new connection from {}", stream_data.1.to_string());
                    let stream_id = Uuid::new_v4();
                    inner_self.lock().await.peers.push(peer::Peer {
                        socket_stream: stream_data_clone.clone(),
                        stream_id,
                        direction: peer::PeerDirection::Inbound,
                        address: Some(stream_data.1.ip()),
                        port: 0,
                    });
                    let stream_data_clone_1 = stream_data_clone.clone();
                    tokio::task::spawn({
                        let inner_self = inner_self.clone();
                        let router_arc = router_arc.clone();
                        async move {
                            handle_connection(
                                inner_self,
                                stream_data_clone_1,
                                router_arc.clone(),
                                stream_id,
                            )
                            .await;
                        }
                    });

                    if server_port == 3002 {
                        println!("Sending first msg");
                        let stream_data_clone = stream_data_clone.clone();
                        let m = Message::new(
                            GossipTypes::RequestServerInfo,
                            "Hello",
                            Some(server_addr),
                            server_port,
                        );
                        let s = m.marshall();
                        match s {
                            Ok(st) => {
                                let _ = stream_data_clone.lock().await.write(st.as_bytes()).await;
                            }
                            Err(e) => {
                                println!("Error in marshalling {}", e);
                            }
                        }
                    }
                }
                Err(err) => {
                    println!("Error accepting connection {}", err);
                }
            }
        }
    }

    pub fn print_status(&self) {
        println!("Running");
    }

    pub async fn connect_to_peer(&self, port: u16, router: router::Router) {
        let inner_self = self.inner.clone();
        let router_arc = Arc::new(router);

        if self.inner.lock().await.port != port {
            let tcp_address = SocketAddr::from(([127, 0, 0, 1], port));
            let stream = TcpStream::connect(tcp_address).await;
            match stream {
                Ok(stream_data) => {
                    let stream_data_clone = Arc::new(Mutex::new(stream_data));
                    println!("Successfully connected to server in port {}", port);
                    let stream_id = Uuid::new_v4();
                    let inner_self = inner_self.clone();
                    let router_arc = router_arc.clone();
                    inner_self.lock().await.peers.push(peer::Peer {
                        socket_stream: stream_data_clone.clone(),
                        stream_id,
                        direction: peer::PeerDirection::Outbound,
                        address: None,
                        port: 0,
                    });
                    handle_connection(inner_self, stream_data_clone, router_arc.clone(), stream_id)
                        .await;
                }
                Err(e) => {
                    println!("Error Connecting Peer {}", e);
                }
            }
        } else {
            println!("Connected to self ignoring");
        }
    }
}
