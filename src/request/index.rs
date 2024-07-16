use crate::readfile::index::read_file;
use once_cell::sync::Lazy;
use regex::Regex;
use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpStream,
    str,
};

const RESPONSE_200: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\n\r\n";
const RESPONSE_405: &[u8] =
    b"HTTP/1.1 405 Method Not Allowed\r\nContent-Type: text/plain; charset=utf-8\r\n\r\n";

static REG: Lazy<Regex> = Lazy::new(|| Regex::new(r#"%%(\w+)%%"#).unwrap());

pub fn handle_connection(
    mut stream: TcpStream,
    params_map: &mut HashMap<&'static str, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 4096];
    stream.read(&mut buffer)?;

    let request = str::from_utf8(&buffer)?;

    let request = request.split_whitespace().collect::<Vec<_>>();
    let (method, path) = (request[0], request[1]);

    if method == "GET" {
        let template = read_file("template.html")?;

        if path.len() > 1 && path.starts_with("/") {
            params_map.insert("uri", (&path[1..]).into());
        }

        let html_content = REG.replace_all(&template, |caps: &regex::Captures| match caps.get(1) {
            Some(m) => match params_map.get(m.as_str()) {
                Some(s) => s.clone(),
                None => "Key not found in parameters map: ".to_string() + m.as_str(),
            },
            _ => String::new(),
        });
        let full_response = [RESPONSE_200, html_content.as_bytes()].concat();
        stream.write_all(&full_response)?;
    } else {
        let response = RESPONSE_405.to_vec();
        stream.write_all(&response)?;
    }
    Ok(())
}
