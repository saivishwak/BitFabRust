use std::sync::Arc;

use super::p2p_routes;
use super::routes;
use http_core::Server;
use router::Router;

pub async fn start(address: String, port: u16) {
    let mut router = Router::new();
    routes::configure(&mut router);

    let mut p2p_router = p2p::router::Router::new();
    p2p_routes::configure(&mut p2p_router);

    let p2p_router_arc = Arc::new(p2p_router);

    let addr = Arc::new(address);

    let p2p_server: Arc<p2p::ServerWrapper> = Arc::new(p2p::ServerWrapper::new(
        addr.clone().to_string(),
        port + 1,
        p2p_router_arc.clone(),
    ));

    let (_, _) = tokio::join!(
        tokio::task::spawn({
            let addr = addr.clone();
            async move {
                let server: Server = Server::new(addr.to_string(), port);
                server.start(router).await;
            }
        }),
        tokio::task::spawn({
            let p2p_server = p2p_server.clone();
            async move {
                let mut p2p_router = p2p::router::Router::new();
                p2p_routes::configure(&mut p2p_router);
                p2p_server.start().await;
            }
        }),
    );
}
