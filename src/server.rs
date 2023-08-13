use std::collections::HashSet;
use std::net::{IpAddr, SocketAddr, TcpListener};
use std::io::Write;
use serde::{Serialize, Deserialize};
use rdev::{grab, Event, EventType, listen};




#[derive(Serialize, Deserialize)]
pub enum TransitionRegion {
    TOP,
    BOTTOM,
    LEFT,
    RIGHT,
}


#[derive(Serialize, Deserialize)]
pub struct Client {
    address: SocketAddr,
    display_size: (u32, u32),
    transition_regions: Vec<TransitionRegion>,
}

pub struct Server {
    address: SocketAddr,
    current_selected_client: Option<Client>,
    clients: Vec<Client>,
    transition_regions: Vec<TransitionRegion>
}




impl Server {
    
    pub fn new(ip: IpAddr, port: u16) -> Self {
        Self {
            address: SocketAddr::new(ip, port),
            clients: Vec::new(),
            current_selected_client: None,
            transition_regions: Vec::new(),
        }
    }

    pub fn add_client(&self) {

    }
    pub fn remove_client(&self) {

    }

    pub fn start(&self) {
        let listener = TcpListener::bind(self.address).unwrap();

        let (sender, reciever) = std::sync::mpsc::channel();
        let call_back = move |event: Event| {
            if let EventType::MouseMove{x, y} = event.event_type {
                let _ = sender.send((x, y));
            }
        };
        std::thread::spawn(|| {listen(call_back)});


        loop 
        {
            while let Some(i) =  reciever.iter().next() {
              println!("{:?}", i);  
            }
        };
    }

}



