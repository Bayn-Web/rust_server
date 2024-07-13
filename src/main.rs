use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;

use regex::Regex;

const RESPONSE_200: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\n\r\n";
const RESPONSE_405: &[u8] =
    b"HTTP/1.1 405 Method Not Allowed\r\nContent-Type: text/plain; charset=utf-8\r\n\r\n";

fn main() {
    let listener = TcpListener::bind("localhost:3000").expect("Failed to bind address");
    println!("Listening on localhost:3000");
    let mut params_map: HashMap<&str, String> = HashMap::new();
    params_map.insert("uri", "github.com/Bayn-Web".to_string());
    params_map.insert("username", "bwb".to_string());
    params_map.insert("act", "go".to_string());
    for stream in listener.incoming() {
        let stream = stream.expect("Failed to accept stream");
        handle_connection(stream, params_map.clone());
    }
}

fn read_file(path: &str) -> io::Result<String> {
    let exe_path = env::current_dir()?;
    let static_path = exe_path.join("static").join(path); // 构建到静态文件的路径
    let mut file = File::open(static_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
fn handle_connection(mut stream: TcpStream, mut params_map: HashMap<&str, String>) {
    let mut buffer = [0; 4096];
    stream
        .read(&mut buffer)
        .expect("Failed to read from socket");

    let request = String::from_utf8_lossy(&buffer).into_owned();

    // Handle parsed HTTP request method and path
    let result = parse_request(request.as_str());
    match result {
        Some((method, path)) => {
            if method == "GET" {
                let template = read_file("template.html");
                let template_str = match template {
                    Ok(temp) => temp,
                    Err(e) => panic!("Error processing template: {}", e),
                };

                if path.chars().count() != 1 {
                    params_map.insert("uri", path[1..].to_string());
                }
                let re = Regex::new(r#"%%(\w+)%%"#).unwrap();
                let html_content = re
                    .replace_all(&template_str, |caps: &regex::Captures| {
                        params_map.get(caps.get(1).unwrap().as_str()).unwrap()
                    })
                    .to_string();
                let response = RESPONSE_200.to_vec();
                let body = html_content.as_bytes().to_vec();
                let full_response = [response, body].concat();
                stream
                    .write_all(&full_response)
                    .expect("Failed to write to socket");
            } else {
                let response = RESPONSE_405.to_vec();
                stream
                    .write_all(&response)
                    .expect("Failed to write to socket");
            }
        }
        _ => {
            let response = RESPONSE_405.to_vec();
            stream
                .write_all(&response)
                .expect("Failed to write to socket");
        }
    }
}

fn parse_request(request: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = request.split_whitespace().collect();
    if parts.len() >= 2 {
        Some((parts[0].to_string(), parts[1].to_string()))
    } else {
        None
    }
}
