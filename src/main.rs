mod request;
mod readfile;
use request::index::handle_connection;

use std::{collections::HashMap, net::TcpListener, str};

fn main() {
    let listener = TcpListener::bind("localhost:3000").expect("Failed to bind address");
    println!("Listening on localhost:3000");
    let mut params_map: HashMap<&'static str, String> = [
        ("uri", "github.com/Bayn-Web".into()),
        ("username", "bwb".into()),
        ("act", "go".into()),
    ]
    .iter()
    .cloned()
    .collect();

    for stream in listener.incoming() {
        let stream = stream.expect("Failed to accept stream");
        handle_connection(stream, &mut params_map).unwrap();
    }
}
