use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
//use uuid::Uuid;
use std::net::SocketAddr;

use p2p;
use p2p::message::Message;
use p2p::GossipTypes;

// function to configure p2p router
pub fn configure(router: &mut p2p::router::Router) {
    router.add_handler(
        GossipTypes::Ping,
        |message: Message, _: SocketAddr, server_state: Arc<Mutex<p2p::Server>>| async move {
            let server_addr = server_state.lock().await.address;
            let server_port = server_state.lock().await.port;
            println!("Ping Handler - Server Add {} {:?}", server_addr, message);
            //To simluate async
            sleep(Duration::from_millis(2000)).await;
            let message =
                Message::new(GossipTypes::Pong, "Ponging", Some(server_addr), server_port);
            let response = message.marshall();
            match response {
                Ok(res) => Some(res),
                Err(_) => None,
            }
        },
    );

    router.add_handler(
        GossipTypes::Pong,
        |message: Message, _: SocketAddr, server_state: Arc<Mutex<p2p::Server>>| async move {
            let server_addr = server_state.lock().await.address;
            let server_port = server_state.lock().await.port;

            println!("Pong Handler - Server Add {} {:?}", server_addr, message);
            //sleep(Duration::from_millis(2000)).await;
            let message =
                Message::new(GossipTypes::Ping, "Pinging", Some(server_addr), server_port);
            let response = message.marshall();
            match response {
                Ok(res) => Some(res),
                Err(_) => None,
            }
        },
    );

    router.add_handler(
        GossipTypes::RequestServerInfo,
        |_: Message, _: SocketAddr, server_info: Arc<Mutex<p2p::Server>>| async move {
            println!("Request server info handler");
            let server_addr = server_info.lock().await.address;
            let server_port = server_info.lock().await.port;

            let message = Message::new(
                GossipTypes::ProcessServerInfo,
                "",
                Some(server_addr),
                server_port,
            );
            let response = message.marshall();
            match response {
                Ok(res) => Some(res),
                Err(_) => None,
            }
        },
    );

    router.add_handler(
        GossipTypes::ProcessServerInfo,
        |message: Message, stream_id: SocketAddr, server_info: Arc<Mutex<p2p::Server>>| async move {
            println!("Process server info handler");
            let server_addr = server_info.lock().await.address;
            let server_port = server_info.lock().await.port;
            let new_peer_port = message.body.peer_info.port;

            {
                let peers = &mut server_info.lock().await.peers;

                for peer in peers {
                    if peer.stream_id == stream_id {
                        println!("Found Peer and changing the port internally");
                        peer.port = message.body.peer_info.port;
                    }
                    println!("After port change Peer - {:?}", peer);
                }
            }

            let message = Message::new(
                GossipTypes::ProcessNewPeer,
                "",
                Some(server_addr),
                server_port,
            );
            //let response = message.marshall();
            let _ = server_info
                .lock()
                .await
                .broadcast_to_peers(message, stream_id, new_peer_port)
                .await;

            None
        },
    );

    router.add_handler(
        GossipTypes::ProcessNewPeer,
        |message: Message, _: SocketAddr, server_info: Arc<Mutex<p2p::Server>>| async move {
            println!("Process New Peer handler");
            let server_port = server_info.lock().await.port;
            let mut found = false;
            let p = message.body.peer_info.port;

            {
                let peers = &server_info.lock().await.peers;
                for peer in peers {
                    if peer.port == p {
                        found = true;
                    }
                }
            }

            if !found {
                if server_port != p {
                    let _ = tokio::task::spawn(async move {
                        if let Err(e) = p2p::utils::connect_to_peer(server_info, p).await {
                            println!("{}", e);
                        }
                    });
                    //let _ = tokio::join!(a);
                }
            } else {
                println!("Found Peer - ingoring connect to peer");
            }

            None
        },
    );
}
