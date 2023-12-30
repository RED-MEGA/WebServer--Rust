use std::fs::{Permissions, self};


pub const OK_HEADER: &str = "HTTP/1.1 200 OK";
pub const CRLF: &str = "\r\n";
pub const ROOT: &str = "./www";

pub fn get_permissions(path: &str) -> Option<Permissions>
{
	match fs::metadata(path) {
		Ok(md) => return Some(md.permissions()),
		Err(_) => return None,
	};
}

pub fn to_body(path: &str) -> Option<String> {

	match std::fs::read_to_string(path) {
		Ok(data) => Some(data),
		Err(_) => None,
	}
}
