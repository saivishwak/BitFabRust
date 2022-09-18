use crate::message;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::server::Server;

type HandlerFn = fn(Arc<Mutex<Server>>) -> String;

pub struct Router {
    pub handlers: HashMap<message::GossipTypes, HandlerFn>,
}

impl Router {
    pub fn new() -> Self {
        println!("P2P Router Object Initiated");
        Router {
            handlers: HashMap::new(),
        }
    }

    pub fn add_handler(&mut self, key: message::GossipTypes, f: HandlerFn) {
        self.handlers.insert(key, f);
    }

    pub fn handle(&self, key: message::GossipTypes, server_state: Arc<Mutex<Server>>) -> String {
        match self.handlers.get(&key) {
            Some(handler) => handler(server_state),
            None => {
                println!("Path not found");
                String::from("Path not found")
            }
        }
    }
}
