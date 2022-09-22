use p2p;
use p2p::message::Message;
use p2p::GossipTypes;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

pub fn configure(router: &mut p2p::router::Router) {
    router.add_handler(
        GossipTypes::Ping,
        |message: Message, server_state: Arc<Mutex<p2p::Server>>| async move {
            println!(
                "Ping Handler - Server Add {} {:?}",
                server_state.lock().await.address,
                message
            );
            //To simluate async
            sleep(Duration::from_millis(2000)).await;
            let message = Message::new(GossipTypes::Pong, "Ponging");
            let response = message.marshall();
            match response {
                Ok(res) => res,
                Err(_) => String::new(),
            }
        },
    );

    router.add_handler(
        GossipTypes::Pong,
        |message: Message, server_state: Arc<Mutex<p2p::Server>>| async move {
            println!(
                "Pong Handler - Server Add {} {:?}",
                server_state.lock().await.address,
                message
            );
            //sleep(Duration::from_millis(2000)).await;
            let message = Message::new(GossipTypes::Ping, "Pinging");
            let response = message.marshall();
            match response {
                Ok(res) => res,
                Err(_) => String::new(),
            }
        },
    );

    router.add_handler(
        GossipTypes::RequestServerInfo,
        |message: Message, _: Arc<Mutex<p2p::Server>>| async move {
            println!("Request server info {:?}", message);
            String::from("Request")
        },
    )
}
