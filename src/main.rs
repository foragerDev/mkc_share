mod server;
mod client;
mod common;

use crate::server::Server;

fn main() {
    let mut server = Server::new("127.0.0.1".parse().unwrap(), 8000);
    server.start();
}
 