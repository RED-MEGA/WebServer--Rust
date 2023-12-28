use std::{io::Write, net::TcpStream, option, path};

use crate::{methods::*, request::Request, ROOT};

pub struct Response {
    pub header: String,
    pub body: String,
}

impl Response {
    pub fn new(header: String, body: String) -> Response {
        Response { header, body }
    }

    pub fn get(path: &str) -> Option<Response>
	{
		ROOT + path.to_owned();
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
        _ => None,
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
