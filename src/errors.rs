// HTTP/1.1 404 Not Found
// Content-Type: text/html

// {
//   "error": {
//     "code": "RESOURCE_NOT_FOUND",
//     "message": "The requested resource could not be found.",
//     "details": "Provide additional details or troubleshooting information here."
//   }
// }

const CRLF: &str = "\r\n";

// const ERROR_BODY: &str = ;

pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    pub details: String,
}

impl ErrorResponse {
    pub fn error_response(self) -> String {
        String::from(format!("HTTP/1.1 {} {}\r\n\r\n", self.code, self.message))
    }

    pub fn not_found() -> String {
        let error = ErrorResponse {
            code: "404".to_owned(),
            message: "Not Found".to_owned(),
            details: "Not Found".to_owned()
        };
        error.error_response()
    }
}
