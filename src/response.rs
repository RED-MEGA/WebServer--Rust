use std::{net::TcpStream, io::Write};

use crate::request::Request;

pub struct Response {
	pub header: String,
	pub body: String
}

impl Response
{
	pub fn new(header: String, body: String) -> Response {
		Response {
			header,
			body
		}
	}
}

pub fn gen_response(request: Request) -> Response
{

}

pub fn send_response(mut stream: &TcpStream, response: Response)
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
