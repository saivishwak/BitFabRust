#![allow(dead_code)]
pub enum MessageSuccessStatusCode {
    Success,
    ClosConnection,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum GossipTypes {
    Ping,
    Pong,
    Def,
}

pub trait FromString {
    fn from_string(input: &String) -> Result<GossipTypes, ()>;
}

impl FromString for GossipTypes {
    fn from_string(input: &String) -> Result<GossipTypes, ()> {
        match input.trim() {
            "ping" => Ok(GossipTypes::Ping),
            "pong" => Ok(GossipTypes::Pong),
            "default" => Ok(GossipTypes::Def),
            _ => {
                return Err(());
            }
        }
    }
}

pub struct Message {}

impl Message {
    pub fn new() -> Self {
        Self {}
    }
}
