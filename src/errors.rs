use std::fs::read_to_string;

use crate::{
    methods::{HTTP_VERSION, _KO},
    response::Response,
};

pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    pub details: String,
}

impl ErrorResponse {
    pub fn error_response(self) -> String {
        String::from(format!("HTTP/1.1 {} {}\r\n\r\n", self.code, self.message))
    }

    pub fn not_found() -> Response {
        let body = read_to_string("www/err/error.html").unwrap();

        Response::new(
            HTTP_VERSION.to_owned(),
            _KO,
            "text/html".to_owned(),
            body.len() as u32,
            body.as_bytes().to_vec(),
        )
    }
}
