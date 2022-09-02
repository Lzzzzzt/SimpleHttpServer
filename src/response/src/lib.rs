mod response_header;
mod status;
mod status_line;
mod utils;

use crate::response_header::Header;
use crate::status::Status;
use crate::status_line::StatusLine;
use json::object;
use std::fmt::{Display, Formatter};
use std::fs;
use std::io::Error;

const NOT_FOUND: &str = "<h1>404 NOT FOUND!</h1>";

// type Headers = HashMap<String, String>;

#[derive(Debug)]
pub struct BaseResponse;

impl BaseResponse {
    pub fn success() -> SuccessResponse {
        SuccessResponse
    }

    // Todo redirect

    pub fn client_error() -> ClientErrorResponse {
        ClientErrorResponse
    }
}

pub struct Response {
    status_line: StatusLine,
    header: Header,
    content: Option<Vec<u8>>,
}

impl Response {
    pub fn new(
        http_version: &str,
        status: Status,
        header: Header,
        content: Option<Vec<u8>>,
    ) -> Self {
        let status_line = StatusLine {
            status,
            http_version: http_version.into(),
        };

        Self {
            status_line,
            header,
            content,
        }
    }

    pub fn as_bytes(&mut self) -> Vec<u8> {
        let response = self.to_string();
        let mut res = response.into_bytes();
        let content = self.content.take();
        res.append(&mut content.unwrap());
        res
    }

    pub fn set_content_type(mut self, content_type: &str) -> Self {
        self.header["Content-Type"] = json::JsonValue::String(content_type.into());
        self
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\r\n{}\r\n\r\n", self.status_line, self.header)
    }
}

pub struct SuccessResponse;

impl SuccessResponse {
    pub fn file(self, file: &str) -> Result<Response, Error> {
        let content = fs::read(file)?;

        let header = object! {
            "Content-Type": utils::parse_file_mime_type(file),
            "Content-Length": content.len()
        };

        Ok(Response::new(
            "HTTP/1.1",
            Status::ok(),
            header.into(),
            Some(content),
        ))
    }

    pub fn string(self, string: &str) -> Response {
        let content = string.as_bytes().to_vec();

        let header = object! {
            "Content-Type": "text/plain",
            "Content-Length": content.len()
        };

        Response::new("HTTP/1.1", Status::ok(), header.into(), Some(content))
    }
}

// Todo RedirectResponse

pub struct ClientErrorResponse;

impl ClientErrorResponse {
    pub fn not_found(self) -> Response {
        let content = NOT_FOUND.as_bytes().to_vec();

        let header = object! {
            "Content-Type": "text/html",
            "Content-Length": content.len()
        };

        Response::new(
            "HTTP/1.1",
            Status::not_found(),
            header.into(),
            Some(content),
        )
    }
}
