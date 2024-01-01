use std::{fs::File, io::Read};

pub const OK_HEADER: &str = "HTTP/1.1 200 OK";
pub const CRLF: &str = "\r\n";
pub const ROOT: &str = "./www";
pub const ACCEPTED_TYPES: [&str; 2] = ["text/html", "image/png"];

pub fn get_permissions(path: &str, o_read: bool, o_write: bool) -> bool {
    File::open(path).is_ok()
}

pub fn to_body(path: &str) -> Option<Vec<u8>> {
    let file_name = match path.contains(".") {
        true => format!("{}{}", ROOT, path),
        false => format!("{}{}/index.html", ROOT, path),
    };

    let mut buffer: Vec<u8> = vec![];

    match File::open(file_name) {
        Ok(mut file) => {
            let _ = file.read_to_end(&mut buffer);
            Some(buffer)
        }
        Err(error) => {
            eprintln!("{}", error);
            None
        }
    }
}

pub fn get_extension(filename: &str) -> &str {
    // Path::new(filename).extension().unwrap().to_str().unwrap()
    "html"
}
