use std::sync::Arc;

use super::p2p_routes;
use super::routes;
use http_core::Server;
use router::Router;

pub async fn start(address: String, port: u16) {
    let mut router = Router::new();
    routes::configure(&mut router);
    let add = Arc::new(address);

    let p2p_server: Arc<p2p::ServerWrapper> =
        Arc::new(p2p::ServerWrapper::new(add.clone().to_string(), port + 1));

    let (_, _, _) = tokio::join!(
        tokio::task::spawn({
            let add = add.clone();
            async move {
                let server: Server = Server::new(add.to_string(), port);
                server.start(router).await;
            }
        }),
        tokio::task::spawn({
            let p2p_server = p2p_server.clone();
            async move {
                let mut p2p_router = p2p::router::Router::new();
                p2p_routes::configure(&mut p2p_router);
                p2p_server.start(p2p_router).await;
            }
        }),
        tokio::task::spawn({
            let p2p_server = p2p_server.clone();
            async move {
                p2p_server.print_status();
                let mut p2p_router = p2p::router::Router::new();
                p2p_routes::configure(&mut p2p_router);

                p2p_server.connect_to_peer(3002, p2p_router).await;
            }
        })
    );
}
