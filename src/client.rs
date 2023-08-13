use rdev::display_size;
use serde::{Serialize, Deserialize, __private::de::ContentDeserializer};
use std::{net::{SocketAddr, TcpStream}, collections::HashMap};

use crate::common::TransitionRegion;

#[derive(Serialize, Deserialize)]
pub struct Client {
    pub display_size: (u64, u64),
    pub transition_regions: HashMap<TransitionRegion, bool>,
}

impl Client {
    fn new() -> Self {
        Self {
            display_size: display_size().expect("Cound not get display size"),
            transition_regions: HashMap::new(),
        }
    }

    pub fn connect(address: SocketAddr, client: Client) -> TcpStream {
        let stream = TcpStream::connect(address).expect("Failed to connect to the server");
        let response = serde_json::to_writer(&stream, &serde_json::json!(&client).to_string());
        return stream
    }
}

fn main() {
    let client = Client::new();
    Client::connect("127.0.0.0:8000".parse().unwrap(), client);
}


