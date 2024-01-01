pub mod errors;
pub mod methods;
pub mod request;
pub mod response;
pub mod tools;

use errors::*;
use request::*;
use response::*;
use std::{net::{TcpListener, TcpStream}, fs::{File, OpenOptions}, io::{Read, Write}, os::unix::thread};

fn handle_connection(stream: TcpStream) {
    let response;

    match recive_request(&stream) {
        Ok(new_request) => {
            response = gen_response(new_request);
            // panic!("{:?}", response);
        }
        Err(_) => {
            send_response(
                &stream,
                ErrorResponse::not_found(),
            );
            return;
        }
    }

    let response = match response {
        Some(response) => response,
        None => {
            send_response(
                &stream,
                ErrorResponse::not_found(),
            );
            return;
        }
    };
    // println!("====> {:?}");
    // send_response(&stream, Response {header:});
    send_response(&stream, response);
}

fn setup() -> TcpListener {
    TcpListener::bind("0.0.0.0:1337").unwrap()
}

fn event_loop(server: TcpListener) {
    for connection in server.incoming() {
        let stream = connection.unwrap();

            handle_connection(stream);
        // thread::(move || {
        //     handle_connection(stream);
        // });
    }
}

fn main() {
    event_loop(
        setup()
    );
}


/*
    HTTP/<http version> <status code> <stat>
    Content-Type: <type of file>
    Content-Length: <len of buffer>

    <Binary data of the image>
*/
