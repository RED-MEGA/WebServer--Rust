pub enum Methods {
	GET(String),
	POST(String),
	DELETE(String),
	EMPTY
}

const ROOT: &str = "./www";