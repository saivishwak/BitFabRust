use p2p;
use p2p::message::Message;
use p2p::GossipTypes;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

pub fn configure(router: &mut p2p::router::Router) {
    router.add_handler(
        GossipTypes::Ping,
        |message: Message, _: Uuid, server_state: Arc<Mutex<p2p::Server>>| async move {
            let server_addr = server_state.lock().await.address;
            let server_port = server_state.lock().await.port;
            println!("Ping Handler - Server Add {} {:?}", server_addr, message);
            //To simluate async
            sleep(Duration::from_millis(2000)).await;
            let message =
                Message::new(GossipTypes::Pong, "Ponging", Some(server_addr), server_port);
            let response = message.marshall();
            match response {
                Ok(res) => res,
                Err(_) => String::new(),
            }
        },
    );

    router.add_handler(
        GossipTypes::Pong,
        |message: Message, _: Uuid, server_state: Arc<Mutex<p2p::Server>>| async move {
            let server_addr = server_state.lock().await.address;
            let server_port = server_state.lock().await.port;

            println!("Pong Handler - Server Add {} {:?}", server_addr, message);
            //sleep(Duration::from_millis(2000)).await;
            let message =
                Message::new(GossipTypes::Ping, "Pinging", Some(server_addr), server_port);
            let response = message.marshall();
            match response {
                Ok(res) => res,
                Err(_) => String::new(),
            }
        },
    );

    router.add_handler(
        GossipTypes::RequestServerInfo,
        |message: Message, _: Uuid, server_info: Arc<Mutex<p2p::Server>>| async move {
            println!("Request server info {:?}", message);
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
                Ok(res) => res,
                Err(_) => String::new(),
            }
        },
    );

    router.add_handler(
        GossipTypes::ProcessServerInfo,
        |message: Message, stream_id: Uuid, server_info: Arc<Mutex<p2p::Server>>| async move {
            println!("Process server info {:?}", message);
            let server_addr = server_info.lock().await.address;
            let server_port = server_info.lock().await.port;
            let peers = &mut server_info.lock().await.peers;

            for peer in peers {
                if peer.stream_id == stream_id {
                    println!("Found Peer");
                    peer.port = message.body.peer_info.port;
                }
                println!("{:?}", peer);
            }

            let message = Message::new(
                GossipTypes::ProcessNewPeer,
                "",
                Some(server_addr),
                server_port,
            );
            let response = message.marshall();
            match response {
                Ok(res) => res,
                Err(_) => String::new(),
            }
        },
    );

    router.add_handler(
        GossipTypes::ProcessNewPeer,
        |message: Message, stream_id: Uuid, server_info: Arc<Mutex<p2p::Server>>| async move {
            println!("Process New Peer {:?}", message);
            let server_addr = server_info.lock().await.address;
            let server_port = server_info.lock().await.port;
            let peers = &mut server_info.lock().await.peers;
            let mut found = false;

            for peer in peers {
                if peer.stream_id == stream_id {
                    found = true;
                }
            }

            println!("********");
            server_info.lock().await.handle();

            if !found {
                if server_port != message.body.peer_info.port {
                    server_info.lock().await.handle();
                }
            }

            let message = Message::new(GossipTypes::Def, "", Some(server_addr), server_port);
            let response = message.marshall();
            match response {
                Ok(res) => res,
                Err(_) => String::new(),
            }
        },
    );
}
