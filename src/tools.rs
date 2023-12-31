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

pub fn to_body(path: &str) -> Option<Vec<u8>> {
    if !get_permissions(path, (true, false)) {
        return None;
    }

    let file_name = match path.contains(".") {
        true => format!("{}{}", ROOT, path),
        false => format!("{}{}/index.html", ROOT, path),
    };

    match File::open(file_name) {
        Ok(file) => {
            file. // open the file and make it to body
        },
        Err(error) => {eprintln!("{}", error); None},
    }
}

// false => panic!("invalid path: {}\nError: {}", path, error.kind().to_string()),