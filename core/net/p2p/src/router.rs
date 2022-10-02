use crate::message;
use crate::server::Server;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;
//use uuid::Uuid;
use std::net::SocketAddr;

type BoxedRouteHandler = Box<
    dyn Fn(message::Message, SocketAddr, Arc<Mutex<Server>>) -> BoxedRouteResponse
        + Send
        + Sync
        + 'static,
>;
type BoxedRouteResponse = Box<dyn Future<Output = Option<String>> + Send + Sync + 'static>;

pub struct Router {
    pub handlers: HashMap<message::GossipTypes, Option<BoxedRouteHandler>>,
}

impl Router {
    pub fn new() -> Self {
        println!("P2P Router Object Initiated");
        Router {
            handlers: HashMap::new(),
        }
    }

    pub fn add_handler<H, R>(&mut self, key: message::GossipTypes, f: H)
    where
        H: Fn(message::Message, SocketAddr, Arc<Mutex<Server>>) -> R + Send + Sync + 'static,
        R: Future<Output = Option<String>> + Send + Sync + 'static,
    {
        let handler: BoxedRouteHandler = Box::new(
            move |msg: message::Message,
                  stream_id: SocketAddr,
                  server_state: Arc<Mutex<Server>>| {
                Box::new(f(msg, stream_id, server_state))
            },
        );
        self.handlers.insert(key, Some(handler));
    }

    pub async fn handle(
        &self,
        msg: message::Message,
        stream_id: SocketAddr,
        server_state: Arc<Mutex<Server>>,
    ) -> Option<String> {
        match self.handlers.get(&msg.gossip_type) {
            Some(handler) => match handler {
                None => Some(String::from("No handler to handle")),
                Some(handle) => Pin::from(handle(msg, stream_id, server_state)).await,
            },
            None => {
                println!("Path not found");
                None
            }
        }
    }
}
