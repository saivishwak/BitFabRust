#![allow(dead_code)]
use std::net::IpAddr;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug)]
pub enum PeerDirection {
    Inbound,
    Outbound,
}

#[derive(Debug)]
pub struct Peer {
    pub socket_stream: Arc<Mutex<TcpStream>>,
    pub stream_id: Uuid,
    pub direction: PeerDirection,
    pub address: Option<IpAddr>,
    pub port: u16,
}
