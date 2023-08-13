use std::{net::{IpAddr, SocketAddr, TcpListener}, collections::HashMap, sync::Arc};
use serde::{Serialize, Deserialize};
use rdev::{Event, EventType, listen};
use std::sync::Mutex;

use crate::common::TransitionRegion;
use crate::client::Client as SenderClient;


#[derive(Serialize, Deserialize)]
pub struct Client {
    address: SocketAddr,
    display_size: (u64, u64),
    transition_regions: HashMap<TransitionRegion, bool>,
}

impl Client {
     fn new(address: SocketAddr, sender_client: SenderClient) -> Self {
        Self {
            address,
            display_size: sender_client.display_size,
            transition_regions: sender_client.transition_regions,
        }
     }
}

pub struct Server {
    address: SocketAddr,
    current_selected_client: Option<Client>,
    clients: Arc<Mutex<Vec<Client>>>,
    transition_regions: Vec<TransitionRegion>
}




impl Server {
    
    pub fn new(ip: IpAddr, port: u16) -> Self {
        Self {
            address: SocketAddr::new(ip, port),
            clients: Arc::new(Mutex::new(Vec::new())),
            current_selected_client: None,
            transition_regions: Vec::new(),
        }
    }

    pub fn add_client(&self) {

    }
    pub fn remove_client(&self) {

    }


    fn handle_client(listener: Arc<TcpListener>, clients: Arc<Mutex<Vec<Client>>>) {
        for requests in listener.incoming() {
            match requests {
                Ok(stream) => {
                    let client: SenderClient = serde_json::from_reader(&stream).unwrap();
                    let mut clients = clients.lock().unwrap();
                    clients.push(Client::new(stream.local_addr().unwrap(), client));
                    println!("cliented added");
                },
                Err(_) => {
                    
                }
            }
        }
    }

    pub fn start(&self) {
        let listener = Arc::new(TcpListener::bind(self.address).unwrap());
        let (sender, reciever) = std::sync::mpsc::channel();
        let call_back = move |event: Event| {
            if let EventType::MouseMove{x, y} = event.event_type {
                let _ = sender.send((x, y));
            }
        };

        std::thread::scope(|s| {
            s.spawn(|| Self::handle_client(listener.clone(), self.clients.clone()));
        });

        std::thread::spawn(|| {listen(call_back)});

        loop 
        {
            while let Some(i) =  reciever.iter().next() {
              println!("{:?}", i);  
            }
        };
    }

}



