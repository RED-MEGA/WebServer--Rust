use std::fs;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::thread::sleep;
use std::time::Duration;

enum Methods {
	GET(String),
	POST(String),
	DELETE
}

struct Resquest {
	buff: String,
	method: Methods,
}

// Methods::GET("index.html".to_string())
impl Resquest
{
	fn new(buff: String) -> Resquest {
		Resquest {
			buff,
			method: Methods::GET("index.html".to_string())
		}
	}
}

struct Response {
	header: String,
	body: String
}

impl Response
{
	fn new(header: String, body: String) -> Response {
		Response {
			header,
			body
		}
	}
}

fn read_request(mut stream: &TcpStream) -> Resquest
{
	let mut buf = [0; 1024];

	Resquest::new(|| -> String {
		stream.read(&mut buf).unwrap();	
		String::from_utf8_lossy(&buf[..]).to_string()
	}())
}

fn send_response(mut stream: &TcpStream, response: Response)
{
	let buff = format!(
		"{}\r\nContent-Length: {}\r\n\r\n{}",
		response.header,
		response.body.len(),
		response.body
	);
	stream.write(buff.as_bytes()).unwrap();
	stream.flush().unwrap();
}

fn handle_connection(mut stream: TcpStream)
{
	let request = read_request(&stream);

	println!("Body =>{}<= END", request.buff);

	let header = "HTTP/1.1 200 OK".to_string();
	let body_path = match request.method {
		Methods::GET(path) => path,
		_ => "index.html".to_string()
	};
	let body = fs::read_to_string(body_path).unwrap();

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
