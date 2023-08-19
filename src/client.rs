use rdev::display_size;
use serde::{Serialize, Deserialize};
use std::{net::{SocketAddr, TcpStream}, collections::HashMap};

use crate::common::Direction;


#[derive(Serialize, Deserialize)]
pub struct Client {
    pub display_size: (u64, u64),
    pub parent: u32,
    pub transition_regions: HashMap<Direction, u32>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            display_size: display_size().expect("Cound not get display size"),
            parent: 0,
            transition_regions: HashMap::new(),
        }
    }

    pub fn connect(address: SocketAddr, client: Client) -> TcpStream {
        let stream = TcpStream::connect(address).expect("Failed to connect to the server");
        let _ = serde_json::to_writer(&stream, &serde_json::json!(&client));
        return stream
    }
}

pub fn main() {
    println!("clinet is running");
    let client = Client::new();
    Client::connect("127.0.0.1:8000".parse().unwrap(), client);
}


