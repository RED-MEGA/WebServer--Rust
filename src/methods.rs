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

pub const _OK: Stat = Stat::OK(200);
pub const _KO: Stat = Stat::KO(404);

pub const HTTP_VERSION: &str = "HTTP/1.1";