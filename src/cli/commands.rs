use http_core::Server;
use router::{Router};

use super::routes;

pub async fn start(address: &String, port: &u16){
    let mut router = Router::new();
    routes::configure(&mut router);

    let server: Server = Server::new(address.to_string(), *port);
    server.start(router).await;
}