use router::{Router, Methods};
use hyper::{
    Body,
    Response
};

pub fn configure(router: &mut Router){
    router.add_handler(String::from(Methods::GET.to_string()+"/"), |_| -> Response<Body>{
        Response::new(Body::from("Home path"))
    });

    router.add_handler(String::from(Methods::GET.to_string()+"hello"), |_| -> Response<Body>{
        Response::new(Body::from("Hello path"))
    });
}