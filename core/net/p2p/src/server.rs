use crate::message::{GossipTypes, Message};
use crate::peer::Peer;
use crate::router;
use std::io;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::io::Interest;
use tokio::net::TcpListener;
//use tokio::net::TcpSocket;
use crate::peer;
use crate::utils;
use std::net::Ipv4Addr;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

pub struct Server {
    pub address: IpAddr,
    pub port: u16,
    pub peers: Vec<Peer>,
    pub router: Arc<router::Router>,
}

impl Server {
    pub async fn handle(&self) {
        println!("Hanlde");
    }

    pub async fn broadcast_to_peers(&self, _: Message, stream_id: SocketAddr, broadcast_port: u16) {
        println!("Broadcast initiated");

        for peer in &self.peers {
            if peer.stream_id != stream_id {
                let message = Message::new(
                    GossipTypes::ProcessNewPeer,
                    "",
                    Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
                    broadcast_port,
                );

                let response = message.marshall();
                let resp = match response {
                    Ok(res) => res,
                    Err(_) => String::new(),
                };

                let mut a = peer.socket_stream.lock().await;
                println!("Broadcasting to peer {}", peer.port);
                let stream_ready = a
                    .ready(Interest::READABLE | Interest::WRITABLE)
                    .await
                    .unwrap();
                if stream_ready.is_writable() {
                    match a.write_all(resp.as_bytes()).await {
                        Ok(_) => {
                            println!("Successfully sent braodcast message");
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            println!("Error in would block write for broadcast");
                            continue;
                        }
                        Err(e) => {
                            println!("Error sending message for broadcast {}", e);
                        }
                    }
                }
            }
        }
    }
}

pub struct ServerWrapper {
    pub inner: Arc<Mutex<Server>>,
}

impl ServerWrapper {
    pub fn new(address: String, port: u16, router: Arc<router::Router>) -> Self {
        println!("Initializing the P2P server at {} on {}", address, port);
        let server = Server {
            address: IpAddr::from_str(&address).unwrap(),
            port,
            peers: Vec::new(),
            router: router.clone(),
        };
        Self {
            inner: Arc::new(Mutex::new(server)),
        }
    }

    pub async fn start(&self) {
        let inner_self = self.inner.clone();
        let server_addr = inner_self.lock().await.address;
        let server_port = inner_self.lock().await.port;
        let addr: SocketAddr = SocketAddr::new(server_addr, server_port);
        let listener = TcpListener::bind(addr).await.unwrap();
        let server_router = inner_self.lock().await.router.clone();

        let a = tokio::task::spawn(async move {
            loop {
                let stream = listener.accept().await;
                match stream {
                    Ok(stream_data) => {
                        let stream_data_clone = Arc::new(Mutex::new(stream_data.0));
                        println!("Accepted new connection from {}", stream_data.1.to_string());
                        //let stream_id = Uuid::new_v4();
                        let stream_id = stream_data.1;
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
                            let router = server_router.clone();
                            async move {
                                utils::handle_connection(
                                    inner_self,
                                    stream_data_clone_1,
                                    router.clone(),
                                    stream_id,
                                    stream_data.1,
                                )
                                .await;
                            }
                        });

                        {
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
                                    let _ =
                                        stream_data_clone.lock().await.write(st.as_bytes()).await;
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
        });
        let inner_self = self.inner.clone();
        tokio::spawn(async move {
            println!("Trying to connect to bootstrap peers");
            utils::connect_to_peer(inner_self, 3002).await;
        });

        let inner_self = self.inner.clone();
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_millis(3000)).await;
                let peers = &inner_self.lock().await.peers;
                let mut count = 0;
                for _ in peers {
                    count += 1;
                }
                println!("Total Number of peers - {}", count);
            }
        });

        let _ = tokio::join!(a);
    }

    pub fn print_status(&self) {
        println!("Running");
    }
}
