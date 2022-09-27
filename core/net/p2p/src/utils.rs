use crate::message::Message;
use crate::router;
use std::io;
use std::net::SocketAddr;
use std::str;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::io::Interest;
//use tokio::net::TcpSocket;
use crate::peer;
//use rand::Rng;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
//use uuid::Uuid;
use crate::Server;

pub async fn handle_connection(
    inner_self: Arc<Mutex<Server>>,
    stream: Arc<Mutex<tokio::net::TcpStream>>,
    router: Arc<router::Router>,
    stream_id: SocketAddr,
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

pub async fn connect_to_peer(server: Arc<Mutex<Server>>, port: u16) {
    let inner_self = server.clone();
    let router_arc = inner_self.lock().await.router.clone();

    if inner_self.lock().await.port != port {
        let tcp_address = SocketAddr::from(([127, 0, 0, 1], port));
        let stream = TcpStream::connect(tcp_address).await;
        match stream {
            Ok(stream_data) => {
                let stream_id = stream_data.peer_addr();
                let stream_data_clone = Arc::new(Mutex::new(stream_data));
                println!("Successfully connected to server in port {}", port);
                //let stream_id = Uuid::new_v4();

                match stream_id {
                    Ok(s) => {
                        let inner_self = inner_self.clone();
                        //let router_arc = router_arc.clone();
                        inner_self.lock().await.peers.push(peer::Peer {
                            socket_stream: stream_data_clone.clone(),
                            stream_id: s,
                            direction: peer::PeerDirection::Outbound,
                            address: None,
                            port: 0,
                        });
                        handle_connection(inner_self, stream_data_clone, router_arc.clone(), s)
                            .await;
                    }
                    Err(e) => {
                        println!("Err in getting peer_adddr {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Error Connecting Peer {}", e);
            }
        }
    } else {
        println!("Connected to self ignoring");
    }
}
