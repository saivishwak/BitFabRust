use router::{Router};
use hyper::{
    Body,
    Response
};

pub fn configure(router: &mut Router){
    router.add_handler(String::from("GET/"), |_| -> Response<Body>{
        Response::new(Body::from("Home path"))
    });

    router.add_handler(String::from("GET/hello"), |_| -> Response<Body>{
        Response::new(Body::from("Hello path"))
    });
}