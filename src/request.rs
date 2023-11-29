use std::{net::TcpStream, io::Read};
use crate::methods::Methods;


pub struct Resquest {
	pub buff: String,
	pub method: Methods,
}

// Methods::GET("index.html".to_string())
impl Resquest
{
	pub fn new(buff: String) -> Resquest {
		
		let request = match buff.lines().next() {
			Some(line) => line,
			None => panic!("Empty request") // you need handle this 
		};
		
		let request: Vec<&str> = request.split(' ').collect();;
	
		let file;


		let method_str = request[0]; // red
		let method: Methods = match method_str {
				"GET" => Methods::GET(file),
				"POST" => Methods::POST(file),
				"DELETE" => Methods::DELETE 
		};
 
		Resquest {
			buff,
			method: Methods::GET("index.html".to_string())
		}
	}
}

pub fn recive_request(mut stream: &TcpStream) -> Resquest
{
	let mut buf = [0; 1024];

	Resquest::new(|| -> String {
		stream.read(&mut buf).unwrap();	
		String::from_utf8_lossy(&buf[..]).to_string()
	}())
}
