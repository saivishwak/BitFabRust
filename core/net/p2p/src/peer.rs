#![allow(dead_code)]
use std::net::TcpStream;

pub struct Peer {
    socket_stream: TcpStream,
}
