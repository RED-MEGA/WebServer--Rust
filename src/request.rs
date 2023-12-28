use std::{net::TcpStream, io::Read};
use crate::methods::Methods;

pub struct Request {
	pub buff: String,
	pub method: Methods,
	pub protocol: String,
}

impl Request
{
	pub fn new(buff: String) -> Result<Request, String> {
		
		let request = match buff.lines().next() {
			Some(line) => line,
			None => return Err("Empty request".to_owned()),
		};
		
		let request: Vec<&str> = request.split(' ').collect();


		println!("<\n");
		for elm in &request {
			println!("{}\n", elm);
		}
		println!("\n>");
		
		let method = match request[0] {
				"GET" => Methods::GET(request[1].to_owned()),
				"POST" => Methods::POST(request[1].to_owned()),
				"DELETE" => Methods::DELETE(request[1].to_owned()),
				_ => Methods::EMPTY
		};
 
		Ok(Request {
			buff,
			method,
			protocol: request[3].to_owned()
		})
	}
}

pub fn recive_request(mut stream: &TcpStream) -> Result<Request, String>
{
	let mut buf: [u8; 1024] = [0; 1024];

	stream.read(&mut buf).unwrap();	

	Request::new(
		String::from_utf8_lossy(&buf[..]).to_string()
	)
}
