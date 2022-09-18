use std::sync::Arc;

use super::p2p_routes;
use super::routes;
use http_core::Server;
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
                let mut p2p_server: p2p::ServerWrapper =
                    p2p::ServerWrapper::new(add.to_string(), port + 1);
                let mut p2p_router = p2p::router::Router::new();
                p2p_routes::configure(&mut p2p_router);
                p2p_server.start(p2p_router).await;
            }
        })
    );
}
