use std::fs::File;
use std::io::Read;

pub struct Http {
    header: Header,
    body: Vec<u8>,
}


pub struct Header {
    status: String,
    content_length: i64,
    content_type: String,
    connection: String,
    access_control_allow_origin: String,
    cache_control: String,
}


pub const HEADER_SUCCESS: &str = "HTTP/1.1 200 OK";
pub const HEADER_NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND";
pub const HEADER_BAD_REQUEST: &str = "HTTP/1.1 400 BAD REQUEST";



impl Header {
    pub fn new(status: &str, content_length: i64, content_type: &str, connection: &str, access_control_allow_origin: &str, cache_control: &str) -> Self {
        Self { status: status.to_string(), content_length, content_type: content_type.to_string(), connection: connection.to_string(), access_control_allow_origin: access_control_allow_origin.to_string(), cache_control: cache_control.to_string() }
    }

    pub fn get_header(&self) -> String {
        format!("{}Content-Length: {}\r\nContent-Type: {}\r\nConnection: {}\r\nAccess-Control-Allow-Origin: {}\r\nCache-Control: {}", self.status, self.content_length, self.content_type, self.connection, self.access_control_allow_origin, self.cache_control)
    }
}



impl Http {
    pub fn new(header: Header, body: Vec<u8>) -> Self {
        Self { header, body }
    }


    pub fn get_response_header(&self) -> String {
        format!(
            "{}\r\nContent-Length: {}\r\nContent-Type: {}\r\nConnection: {}\r\nAccess-Control-Allow-Origin: {}\r\nCache-Control: {}\r\n\r\n",
            self.header.status,
            self.header.content_length,
            self.header.content_type,
            self.header.connection,
            self.header.access_control_allow_origin,
            self.header.cache_control
        )
    }


    pub fn get_body(&self) -> &Vec<u8> {
        &self.body
    }
}



pub fn read_file(path: &str) -> Option<Vec<u8>> {
    let mut file = File::open(path).ok()?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).ok()?;
    Some(contents)
}


pub fn get_content_type(path: &str) -> &str {
    match path.split('.').last().unwrap_or("") {
        "css" => "text/css",
        "html" => "text/html; charset=utf-8",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "js" => "application/javascript",
        "json" => "application/json",
        "txt" => "text/plain; charset=utf-8",
        "svg" => "image/svg+xml",
        _ => "application/octet-stream",
    }
}
