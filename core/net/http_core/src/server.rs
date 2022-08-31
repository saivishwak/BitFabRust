use hyper::{
    service::{make_service_fn, service_fn},
     Server as hyperServer,
};
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::{convert::Infallible, net::SocketAddr};

//submodules
//use crate::types;
pub struct Server {
    pub address: IpAddr,
    pub port: u16,
}

impl Server {
    pub fn new(address: String, port: u16) -> Self {
        println!("Initializing the server at {} on {}", address, port);
        Self {
            address: IpAddr::from_str(&address).unwrap(),
            port: port,
        }
    }

    pub async fn start(&self, r: router::Router) {
        let addr: SocketAddr = SocketAddr::new(self.address, self.port);
        //let listener = TcpListener::bind(addr).await?;
        let r = Arc::new(r);
        let make_svc = make_service_fn(move |_conn| {
            let r = r.clone();
            async move {
                let r = r.clone();
                Ok::<_, Infallible>(service_fn(move |req| {
                    let r = r.clone();
                    async move {
                        let mut s = req.method().to_string();
                        s = s + &req.uri().to_string();
                        let a = r.handle(s, req);
                        Ok::<_, Infallible>(a)
                    }
                }))
            }
        });

        // IF need to create a own server handler use below -- note its not ready yet
        /*loop {
            let (stream, _) = listener.accept().await?;

            tokio::task::spawn(async move {
                if let Err(err) = Http::new()
                    .serve_connection(stream, service_fn(hello))
                    .await
                {
                    println!("Error serving connection: {:?}", err);
                }
            });
        }*/

        let server = hyperServer::bind(&addr).serve(make_svc);
        println!("Listening on http://{}", addr);
        if let Err(e) = server.await {
            eprintln!("{}", e);
        };
    }
}
