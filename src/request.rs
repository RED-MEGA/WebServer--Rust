use std::{net::TcpStream, io::Read, alloc::System};
use crate::methods::Methods;

pub struct Resquest {
	pub buff: String,
	pub method: Methods,
	pub protocol: String,
}

impl Resquest
{
	pub fn new(buff: String) -> Resquest {
		
		let request = match buff.lines().next() {
			Some(line) => line,
			None => panic!("Empty request") // you need handle this 
		};
		
		let request: Vec<&str> = request.split(' ').collect();
	
		let file = String::from("index.html");

		println!("<\n");
		for elm in &request {
			println!("{}\n", elm);
		}
		println!("\n>");
		

		let method_str = request[0]; // red
		let method: Methods = match method_str {
				"GET" => Methods::GET(file),
				"POST" => Methods::POST(file),
				"DELETE" => Methods::DELETE,
				_ => Methods::EMPTY
		};
 
		Resquest {
			buff,
			method,
			protocol: "HTTP/1.1".to_string()
		}
	}
}

pub fn recive_request(mut stream: &TcpStream) -> Resquest
{
	let mut buf: [u8; 1024] = [0; 1024];

	stream.read(&mut buf).unwrap();	

	Resquest::new(
		String::from_utf8_lossy(&buf[..]).to_string()
	)
}
