use std::{io::Write, net::TcpStream};

use crate::{methods::*, request::Request, tools::{ROOT, get_permissions, OK_HEADER, to_body}, errors::ErrorResponse};

#[derive(Debug)]
pub struct Response {
    pub header: String,
    pub body: String,
}

impl Response {
    pub fn new(header: String, body: String) -> Response {
        Response { header, body }
    }
	
    pub fn default() -> Response {
        Response {
			header: ErrorResponse::not_found(),
			body: "".to_owned()
		}
    }

    pub fn get(path: &str) -> Option<Response> {
        let path = String::from(ROOT) + path;
		match  {
            true => {
                if get_permissions(&path, (true, false)) {
                    Some(Response::new(
                        OK_HEADER.to_owned(),
                        (|| {
                            match to_body(&path) {
                                Some(body) => body,
                                None => "forbidden".to_owned(),
                            }
                        })()
                    )) /// here
                }
                else { None }
            },
			false => { println!("EMPTY Method"); None }
		}
    }

    pub fn post(path: &str) -> Option<Response> {
        Some(Response {
            header: String::from("_"),
            body: String::from("_"),
        })
    }

    pub fn delete(path: &str) -> Option<Response> {
        Some(Response {
            header: String::from("_"),
            body: String::from("_"),
        })
    }
}

pub fn gen_response(request: Request) -> Option<Response> {
    match request.method {
        Methods::GET(path) => Response::get(&path),
        Methods::POST(path) => Response::post(&path),
        Methods::DELETE(path) => Response::delete(&path),
        _ => None
    }
}

pub fn send_response(mut stream: &TcpStream, response: Response) {
    let buff = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        response.header,
        response.body.len(),
        response.body
    );
    stream.write(buff.as_bytes()).unwrap();
    stream.flush().unwrap();
}
