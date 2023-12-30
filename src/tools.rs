use std::{fs::File, path::Path, fmt::format};

pub const OK_HEADER: &str = "HTTP/1.1 200 OK";
pub const CRLF: &str = "\r\n";
pub const ROOT: &str = "./www";

pub fn get_permissions(path: &str, permissions: (bool, bool)) -> bool {
    File::options()
        .read(permissions.0)
        .write(permissions.1)
        .open(path)
        .is_ok()
}

pub fn to_body(path: &str) -> Option<String> {
    match std::fs::read_to_string(path) {
        Ok(data) => Some(data),
        Err(error) => match error.kind().to_string().contains("is a directory") {
            true => match std::fs::read_to_string(format!("{}/index.html", path)) {
                Ok(data) => Some(data),
                Err(_) => panic!("invalid path + /index.html"),
            },
            false => match error.kind().to_string().contains("invalid data") {
                true => ,
                false => panic!("{}", error.kind()),
            }
        },
    }
}

// false => panic!("invalid path: {}\nError: {}", path, error.kind().to_string()),