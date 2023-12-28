pub mod methods;
pub mod request;
pub mod response;
pub mod errors;

use std::fs;
use std::net::{TcpListener, TcpStream};
use methods::*;
use request::*;
use response::*;
use errors::*;

const ROOT: &str = "www/";
const OK_HEADER: &str = "HTTP/1.1 200 OK";

fn handle_connection(stream: TcpStream)
{
	let request;
	let body;

	match recive_request(&stream) {
		Ok(new_request) => {request = new_request},
		Err(_) => {
			send_response(&stream, Response::new (
				ErrorResponse::not_found(),
				"".to_owned()
			));
			return ;
		},
	}
	
	
	let response: Response = gen_response(request);


	let header = OK_HEADER.to_owned();
	let response = Response {header, body};

	send_response(&stream, response);
}

fn setup() -> TcpListener
{
	TcpListener::bind("0.0.0.0:1337").unwrap()
}

fn event_loop(server: TcpListener)
{
	for connection in server.incoming()
	{
		let stream = connection.unwrap();
	
		handle_connection(stream);
	}
}

fn main()
{
	let server = setup();

	event_loop(server);
}
