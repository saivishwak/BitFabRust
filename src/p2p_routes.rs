use p2p;
use p2p::GossipTypes;
use std::sync::{Arc, Mutex};
//use tokio::time::{sleep, Duration};

pub fn configure(router: &mut p2p::router::Router) {
    router.add_handler(
        GossipTypes::Ping,
        |server_state: Arc<Mutex<p2p::Server>>| async move {
            println!(
                "Pin Handler - Server Add {}",
                server_state.lock().unwrap().address
            );
            //To simluate async
            //sleep(Duration::from_millis(5000)).await;
            String::from("Pong\n")
        },
    );

    router.add_handler(
        GossipTypes::Pong,
        |server_state: Arc<Mutex<p2p::Server>>| async move {
            println!(
                "Pong Handler - Server Add {}",
                server_state.lock().unwrap().address
            );
            String::from("Ping\n")
        },
    );
}
