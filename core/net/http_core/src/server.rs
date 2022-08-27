use std::net::{IpAddr};
use std::str::FromStr;
use std::{convert::Infallible, net::SocketAddr};
use hyper::{Body, Request, Response};
use hyper::service::{make_service_fn, service_fn};
use tokio::net::TcpListener;
use hyper::server::conn::Http;

async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello World! From Server")))
}

pub enum  ServerError {
    StartError
}


pub struct Server{
    pub address: IpAddr,
    pub port: u16
}

impl Server {
    pub fn new(address: String, port:u16) -> Self{
        println!("Initializing the server at {} on {}", address, port);
        Self {
            address: IpAddr::from_str(&address).unwrap(),
            port: port
        }
    }

    pub async fn start(&self) -> Result<(), (Box<dyn std::error::Error + Send + Sync>)>{
        let addr: SocketAddr = SocketAddr::new(self.address, self.port);
        let listener = TcpListener::bind(addr).await?;
        println!("Listening on http://{}", addr);
        loop {
            let (stream, _) = listener.accept().await?;

            tokio::task::spawn(async move {
                if let Err(err) = Http::new()
                    .serve_connection(stream, service_fn(hello))
                    .await
                {
                    println!("Error serving connection: {:?}", err);
                }
            });
        }
    }

    pub fn get_addr(&self) -> IpAddr{
        self.address
    }

    pub fn get_addr_string(&self) -> String{
        self.address.to_string()
    }
}