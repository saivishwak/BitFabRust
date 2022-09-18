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

    /*pub async fn handle(
        &self,
        stream: &mut tokio::net::TcpStream,
    ) -> Result<MessageSuccessStatusCode, &str> {
        let mut buf_reader = BufReader::new(stream);
        let mut line = String::new();
        let result = buf_reader.read_line(&mut line).await;
        match result {
            Ok(read) => {
                if read == 0 {
                    return Ok(MessageSuccessStatusCode::ClosConnection);
                }
                let gossip_str_res = GossipTypes::from_string(&line);
                match gossip_str_res {
                    Ok(gossip_type) => match gossip_type {
                        GossipTypes::Ping => {
                            println!("Ping");
                        }
                        GossipTypes::Pong => {
                            println!("pong");
                        }
                        GossipTypes::Def => {
                            println!("Def");
                        }
                    },
                    Err(_) => {
                        println!("Error in decoding type");
                    }
                }
                match buf_reader.write_all(line.as_bytes()).await {
                    Ok(_) => {
                        return Ok(MessageSuccessStatusCode::Success);
                    }
                    Err(_) => {
                        println!("Error sending msg");
                        return Err("Error sending msg");
                    }
                }
            }
            Err(_) => {
                return Err("Error reading message");
            }
        }
    }*/
}
