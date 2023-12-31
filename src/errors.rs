// HTTP/1.1 404 Not Found
// Content-Type: text/html

// {
//   "error": {
//     "code": "RESOURCE_NOT_FOUND",
//     "message": "The requested resource could not be found.",
//     "details": "Provide additional details or troubleshooting information here."
//   }
// }

use std::fs::read_to_string;

use crate::{response::Response, methods::{_KO, HTTP_VERSION}};

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
            body.as_bytes().to_vec()
        )
        
    }
}
