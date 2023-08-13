use std::collections::HashSet;
use std::net::{IpAddr, SocketAddr, TcpListener};
use std::io::Write;
use rdev::{grab, Event, EventType, listen};



pub enum TransitionRegion {
    TOP,
    BOTTOM,
    LEFT,
    RIGHT,
}

// pub struct Client {
//     address: SocketAddress,
//     display_size: (u32, u32),
//     transition_regions: []
// }

pub struct Server {
    address: SocketAddr,
    client: Vec<u32>,
    //transition_regions: HashSet<TransitionRegion>
}




impl Server {
    
    pub fn new(ip: IpAddr, port: u16) -> Self {
        Self {
            address: SocketAddr::new(ip, port),
            client: Vec::new(),
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

                sender.send((x, y));
                println!("{}, {}", x, y );
                //Ok(event.event_type);
            }
        
            //None
        };
        loop {
            println!("I am here, please wait for me to connect");
            //let (stream, address) = listener.accept().unwrap();
             
            listen(call_back);
            // stream.write()
        }
    }

}



