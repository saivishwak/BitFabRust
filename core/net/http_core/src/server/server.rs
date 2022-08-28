use hyper::server::conn::AddrStream;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Server as hyperServer,
};
use std::net::IpAddr;
use std::str::FromStr;
use std::{convert::Infallible, net::SocketAddr};

use router;

//submodules
mod types;

type Response = hyper::Response<hyper::Body>;

pub struct Server {
    pub address: IpAddr,
    pub port: u16,
}

impl Server {
    pub fn new(address: String, port: u16) -> Self {
        println!("Initializing the server at {} on {}", address, port);
        Self {
            address: IpAddr::from_str(&address).unwrap(),
            port,
        }
    }

    pub async fn start(&self) {
        router::router();
        let addr: SocketAddr = SocketAddr::new(self.address, self.port);
        //let listener = TcpListener::bind(addr).await?;

        let make_svc = make_service_fn(|socket: &AddrStream| {
            let remote_addr = socket.remote_addr();
            async move {
                Ok::<_, Infallible>(service_fn(move |_: Request<Body>| async move {
                    Ok::<_, Infallible>(Response::new(Body::from(format!(
                        "Hello, {}!",
                        remote_addr
                    ))))
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

    pub fn get_addr(&self) -> IpAddr {
        self.address
    }

    pub fn get_addr_string(&self) -> String {
        self.address.to_string()
    }
}
