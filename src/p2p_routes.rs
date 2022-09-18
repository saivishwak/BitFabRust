use p2p;
use p2p::GossipTypes;
use std::sync::{Arc, Mutex};
use std::{thread, time};

pub fn configure(router: &mut p2p::router::Router) {
    router.add_handler(
        GossipTypes::Ping,
        |server_state: Arc<Mutex<p2p::Server>>| -> String {
            println!(
                "*** FROM P2P handler {} ",
                server_state.lock().unwrap().address
            );
            String::from("Pong\n")
        },
    );

    router.add_handler(
        GossipTypes::Pong,
        |server_state: Arc<Mutex<p2p::Server>>| -> String {
            println!(
                "*** FROM P2P handler Pong {} ",
                server_state.lock().unwrap().address
            );
            String::from("Ping\n")
        },
    );

    router.add_handler(
        GossipTypes::Def,
        |server_state: Arc<Mutex<p2p::Server>>| -> String {
            println!(
                "*** FROM P2P handler {} Default",
                server_state.lock().unwrap().address
            );
            String::from("Default\n")
        },
    );
}
