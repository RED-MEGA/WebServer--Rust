pub mod methods;
pub mod request;
pub mod response;

use std::fs;
use std::net::{TcpListener, TcpStream};
use methods::Methods;
use request::*;
use response::*;

const ROOT: &str = "www/";

fn handle_connection(mut stream: TcpStream)
{
	let request = recive_request(&stream);

	println!("Body =>{}<= END", request.buff);

	let header = "HTTP/1.1 200 OK".to_string();
	let body_path = match request.method {
		Methods::GET(path) => path,
		_ => "index.html".to_owned()
	};
	let body = fs::read_to_string(ROOT.to_owned() + &body_path).unwrap();

	send_response(&stream, Response::new (
		header,
		body
	));
}

fn setup() -> TcpListener
{
	TcpListener::bind("127.0.0.1:1337").unwrap()
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
