use std::{fmt::format, io::Write, net::TcpStream};

use crate::{
    errors::ErrorResponse,
    methods::*,
    request::Request,
    tools::{get_permissions, to_body, OK_HEADER, ROOT},
};

pub struct Response {
    http_version: String,
    stat: Stat,
    content_type: String,
    content_length: u32,
    body: Vec<u8>,
}

impl Response {
    pub fn new(
        http_version: String,
        stat: Stat,
        content_type: String,
        content_length: u32,
        body: Vec<u8>,
    ) -> Response {
        Response {
            http_version,
            stat,
            content_type,
            content_length,
            body,
        }
    }

    pub fn header(self) -> String {
        let stat = match self.stat {
            Stat::OK(status_code) => (status_code, "OK"),
            Stat::KO(status_code) => (status_code, "KO"),
        };

        format!(
            "{} {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
            self.http_version, stat.0, stat.1, self.content_type, self.content_length
        )
    }

    pub fn get(path: &str, http_version: String) -> Option<Response> {

        // Response::new(
        //     http_version,
        //     stat,
        //     content_type,
        //     content_length,
        //     body
        // )
    }

    pub fn post(path: &str, http_version: String) -> Option<Response> {
        Some(ErrorResponse::not_found())
    }

    pub fn delete(path: &str, http_version: String) -> Option<Response> {
        Some(ErrorResponse::not_found())
    }
}

pub fn gen_response(request: Request) -> Option<Response> {
    match request.method {
        Methods::GET(path) => Response::get(&path, request.http_version),
        Methods::POST(path) => Response::post(&path, request.http_version),
        Methods::DELETE(path) => Response::delete(&path, request.http_version),
        _ => None,
    }
}

pub fn send_response(mut stream: &TcpStream, response: Response) {
    stream.write_all(response.header().as_bytes());
    stream.write_all(&response.body);
    stream.flush();
}
