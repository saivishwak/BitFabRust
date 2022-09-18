use hyper::{Body, Request, Response, StatusCode};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

type BoxedRouteHandler = Box<dyn Fn(Request<Body>) -> BoxedRouteResponse + Send + Sync + 'static>;
type BoxedRouteResponse = Box<dyn Future<Output = Response<Body>> + Send + Sync + 'static>;

//type HandlerFn = fn(Request<Body>) -> Response<Body>;
pub struct Router {
    pub handlers: HashMap<String, Option<BoxedRouteHandler>>,
}

impl Router {
    pub fn new() -> Self {
        println!("Router Object Initiated");
        Router {
            handlers: HashMap::new(),
        }
    }

    pub fn add_handler<H, R>(&mut self, key: String, f: H)
    where
        H: Fn(Request<Body>) -> R + Send + Sync + 'static,
        R: Future<Output = Response<Body>> + Send + Sync + 'static,
    {
        let handler: BoxedRouteHandler = Box::new(move |req: Request<Body>| Box::new(f(req)));
        self.handlers.insert(key, Some(handler));
    }

    pub async fn handle(&self, key: String, req: Request<Body>) -> Response<Body> {
        //self.handlers[&key](req)
        match self.handlers.get(&key) {
            Some(handler) => match handler {
                None => {
                    let resp = Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(Body::from("404 Not Found"));
                    resp.unwrap()
                }
                Some(handle) => Pin::from(handle(req)).await,
            },
            None => {
                println!("Path not found {}", key);
                let resp = Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from("404 Not Found"));
                resp.unwrap()
            }
        }
    }
}
