use crate::methods::Methods;
use std::{io::Read, net::TcpStream};

pub struct Request {
    pub method: Methods,
    pub http_version: String,
}

impl Request {
    pub fn new(buff: String) -> Result<Request, String> {

        let method: Methods;
        let request: Vec<&str>;

        request = match buff.lines().next() {
            Some(line) => line.split(' ').collect(),
            None => return Err("Empty request".to_owned()),
        };

        /* Debug  */
        println!("<\n");
        for elm in &request {
            println!("{}\n", elm);
        }
        println!("\n>");
        
        eprintln!("<\n");
        eprintln!("{}", buff);
        eprintln!("\n>");
        /* Debug  */

        method = match request[0] {
            "GET" => Methods::GET(request[1].to_owned()),
            "POST" => Methods::POST(request[1].to_owned()),
            "DELETE" => Methods::DELETE(request[1].to_owned()),
            _ => Methods::EMPTY,
        };

        Ok(Request {
            method,
            http_version: request[2].to_string(),
        })
    }
}

pub fn recive_request(mut stream: &TcpStream) -> Result<Request, String> {
    let mut buf: [u8; 1024] = [0; 1024];

    stream.read(&mut buf).unwrap();

    Request::new(String::from_utf8_lossy(&buf[..]).to_string())
}
