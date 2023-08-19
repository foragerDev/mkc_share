use rdev::{listen, Event, EventType, display_size};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::sync::Mutex;
use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr, TcpListener},
    sync::Arc,
};

use crate::client::Client as SenderClient;
use crate::common::Direction;

static mut CLIENT_ID: u32 = 0;

fn get_next_id() -> u32 {
    let id;
    unsafe {
        CLIENT_ID += 1;
        id = CLIENT_ID;
    }
    id
}
#[derive(Serialize, Deserialize)]
pub struct Client {
    name: Option<String>,
    address: SocketAddr,
    display_size: (u64, u64),
    parent: Option<u32>,
    transition_regions: HashMap<Direction, u32>,
}

impl Client {
    fn new(address: SocketAddr, sender_client: SenderClient) -> Self {
        Self {
            name: None,
            address,
            parent: None,
            display_size: sender_client.display_size,
            transition_regions: sender_client.transition_regions,
        }
    }

    fn set_parent(&mut self, parent: u32) {
        self.parent = Some(parent);
    }
}

pub struct Server {
    display_size: (u64, u64),
    address: SocketAddr,
    clients: Arc<Mutex<HashMap<u32, Client>>>,
    transition_regions: HashMap<Direction, u32>,
}

impl Server {
    pub fn new(address: SocketAddr) -> Self {
        Self {
            display_size: display_size().expect("Unable to find display size"),
            address,
            clients: Arc::new(Mutex::new(HashMap::new())),
            transition_regions: HashMap::new(),
        }
    }

    pub fn remove_client(&self) {}

    // For server client parent will be none which means root
    fn map_client(&mut self, client_id: u32, client_parent: Option<u32>, direction: Direction) {
        match client_parent {
            None => {
                self.transition_regions.insert(direction, client_id);
            }
            Some(id) => {
                let mut clients = self.clients.lock().unwrap();
                let parent_client = clients.get_mut(&id);
                match parent_client {
                    Some(parent) => {
                        parent.transition_regions.insert(direction, client_id);
                        clients.get_mut(&client_id).unwrap().set_parent(id);
                    }
                    None => {
                        println!("Parent not found");
                    }
                }
            }
        }
    }
    fn handle_client(listener: Arc<TcpListener>, clients: Arc<Mutex<HashMap<u32, Client>>>) {
        for requests in listener.incoming() {
            match requests {
                Ok(stream) => {
                    let client: SenderClient = serde_json::from_reader(&stream).unwrap();
                    let mut clients = clients.lock().unwrap();
                    clients.insert(
                        get_next_id(),
                        Client::new(stream.local_addr().unwrap(), client),
                    );
                    println!("cliented added");
                }
                Err(_) => {}
            }
        }
    }

    fn detect_edge(&self, points: (f64, f64), client: &Option<Client>) -> Option<Direction> {
        let (x, y) = points;

        let display_size = match client {
            Some(client) => client.display_size,
            None => self.display_size,
        };

        if x < 0_f64 && y > 0_f64 {
            return Some(Direction::LEFT);
        }

        if x > 0_f64 && y < 0_f64 {
            return Some(Direction::TOP);

        }

        if x > 0_f64 && y >= display_size.1 as f64 {
            return Some(Direction::BOTTOM);

        }

        if x >= display_size.0  as f64 && y > 0_f64 {
            return Some(Direction::RIGHT);
        }
        None
    }
    

    pub fn start(&self) {
        let listener = Arc::new(TcpListener::bind(self.address).unwrap());
        let (sender, reciever) = std::sync::mpsc::channel();
        let call_back = move |event: Event| {
            if let EventType::MouseMove { x, y } = event.event_type {
                let _ = sender.send((x, y));
            }
        };

        std::thread::scope(|s| {
            s.spawn(|| Self::handle_client(listener.clone(), self.clients.clone()));
            s.spawn(|| listen(call_back));

            loop {
                let current_selected_client: Option<Client> = None;
                while let Some(i) = reciever.iter().next() {
                    if let Some(edge) = self.detect_edge(i, &current_selected_client) {
                        if let Some(client_id) = self.transition_regions.get(&edge) {

                        }
                    }
                    
                }
            }
        });
    }
}
