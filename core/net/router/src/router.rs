use hyper::{Body, Request, Response, StatusCode};
use std::collections::HashMap;

type HandlerFn = fn(Request<Body>) -> Response<Body>;
pub struct Router {
    pub handlers: HashMap<String, HandlerFn>,
}

impl Router {
    pub fn new() -> Self {
        println!("Router Object Initiated");
        Router {
            handlers: HashMap::new(),
        }
    }

    pub fn add_handler(&mut self, key: String, f: HandlerFn) {
        self.handlers.insert(key, f);
    }

    pub fn handle(&self, key: String, req: Request<Body>) -> Response<Body> {
        //self.handlers[&key](req)
        match self.handlers.get(&key){
            Some(handler) => handler(req),
            None => {
                println!("Path not found {}", key);
                let resp = Response::builder().status(StatusCode::NOT_FOUND)
                .body(Body::from("404 Not Found"));
                resp.unwrap()
            }
        }
    }
}
