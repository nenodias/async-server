use std::io::{Result as IoResult};
use std::string::ToString;
use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt};
use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    pub async fn send(&self, stream: &mut TcpStream) -> IoResult<usize> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        let res = format!("HTTP/1.1 {} {}\r\n\r\n{}",
                self.status_code,
                self.status_code.reason_phrase(),
                body);
        stream.write(res.as_bytes()).await
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        return format!("HTTP/1.1 {} {}\r\n\r\n{}",
                self.status_code,
                self.status_code.reason_phrase(),
                body);
    }
}