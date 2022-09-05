//use hyper::server::conn::Http;
use hyper::service::{service_fn, make_service_fn};
use hyper::{Server as hyperServer};
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::{convert::Infallible, net::SocketAddr};
//use tokio::net::TcpListener;
//use std::time::Duration;

//submodules
//use super::constants;

/*fn handle_connection(stream: tokio::net::TcpStream, r: &Arc<router::Router>){
    tokio::task::spawn({
        let r = r.clone();
        async move {
            if let Err(err) = Http::new()
            .http2_keep_alive_timeout(Duration::from_millis(constants::HTTP2_KEEP_ALIVE_TIMEOUT))
            .http1_keep_alive(true)
                .serve_connection(
                    stream,
                    service_fn(move |req| {
                        let r = r.clone();
                        async move {
                            let s = req.method().to_string()
                                + &req.uri().to_string();
                            let a = r.handle(s, req);
                            Ok::<_, Infallible>(a)
                        }
                    }),
                )
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        }
    });
}*/

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
        //let listener = TcpListener::bind(addr).await.unwrap();
        let r = Arc::new(r);
        // use if hyper::Server is used
        let make_svc = make_service_fn(move |_conn| {
            let r = r.clone();
            async move {
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
        /*let handler = tokio::task::spawn(async move {
            loop {
                let stream = listener.accept().await;
                match stream {
                    Ok(res) => {
                        handle_connection(res.0, &r);
                    }
                    Err(err)=> {
                        println!("Error accepting connection {}", err);
                    }
                }
            }
        });

        let _ = tokio::join!(handler);*/

        // When using hyper as internal server
        let server = hyperServer::bind(&addr).serve(make_svc);
        println!("Listening on http://{}", addr);
        if let Err(e) = server.await {
            eprintln!("{}", e);
        };
    }
}
