use std::{sync::Arc};

use super::routes;
use http_core::Server;
use p2p;
use router::Router;

pub async fn start(address: String, port: u16) {
    let mut router = Router::new();
    routes::configure(&mut router);

    let add = Arc::new(address);

    let (_, _) = tokio::join!(
        tokio::task::spawn({
            let add = add.clone();
            async move {
                let server: Server = Server::new(add.to_string(), port);
                server.start(router).await;
            }
        }),
        tokio::task::spawn({
            let add = add.clone();
            async move {
                let add = add.clone();
                let p2p_server: p2p::Server = p2p::Server::new(add.to_string(), port+1);
                p2p_server.start().await;
            }
        })
    );
}
