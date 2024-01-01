pub enum Methods {
    GET(String),
    POST(String),
    DELETE(String),
    EMPTY,
}

pub enum Stat {
    OK(u16),
    KO(u16),
}

// 415 Unsupported Media
pub const _OK: Stat = Stat::OK(200);
pub const _KO: Stat = Stat::KO(404);

pub const HTTP_VERSION: &str = "HTTP/1.1";


pub const CONTENT_TYPES: [&str; 25] = [
            "text/plain",
            "text/html",
            "text/css",
            "text/javascript",
            "application/json",
            "application/xml",
            "image/jpeg",
            "image/png",
            "image/gif",
            "image/svg+xml",
            "audio/mpeg",
            "audio/wav",
            "audio/ogg",
            "video/mp4",
            "video/webm",
            "video/ogg",
            "application/pdf",
            "application/zip",
            "application/x-www-form-urlencoded",
            "multipart/form-data",
            "font/ttf",
            "font/otf",
            "font/woff",
            "application/octet-stream",
            "application/xhtml+xml",
        ];