mod client;
mod common;
mod server;

use std::net::SocketAddr;

use clap::{arg, value_parser, Command, Parser};
use client::Client;

use crate::server::Server;

fn main() {
    let command_matches = Command::new("ShareInput")
        .version("0.1")
        .author("Mohsan Ali <mohsan0073@gmail.com>")
        .about("Let you share the mouse between different computers on the same networ")
        .subcommand(Command::new("server"))
        .subcommand(
            Command::new("client").arg(
                arg!(-s --server <ServerIP> "Address of the server")
                    .value_parser(value_parser!(SocketAddr))
                    .required(true),
            ),
        )
        .get_matches();


    if let Some(_) = command_matches.subcommand_matches("server") {
        println!("here");
        let server = Server::new("127.0.0.1:9000".parse::<SocketAddr>().expect("Wrong server address"));
        if let Ok(ip) = local_ip_address::local_ip() {
            println!("Server is running at {}:{}", ip, 9000);
        }
        server.start();
    } 
    if let Some(client) = command_matches.subcommand_matches("client") {
        let server_address = client.get_one::<SocketAddr>("server").unwrap();
        let client = Client::new();
        Client::connect(*server_address, client);

    }
    // let client = Command::new("Client").arg(arg!(--client <VALUE >).required(true));
}
