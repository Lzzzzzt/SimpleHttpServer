use std::fmt::{Display, Formatter};
use json::{JsonValue, object};
use methods::Methods;
use crate::utils::parse_kv;

#[derive(Debug)]
pub struct RequestLine {
    pub method: Methods,
    pub url: String,
    pub http_version: String,
    pub query: JsonValue,
}

impl RequestLine {
    pub fn parse(request_line: &str) -> Self {
        let mut request_line = request_line.split(' ');

        let method = match request_line.next().unwrap() {
            "GET" => Methods::Get,
            "POST" => Methods::Post,
            _ => Methods::Get,
        };

        let mut url_with_query = request_line.next().unwrap().split('?');

        let url = url_with_query.next().unwrap().to_string();

        let query = match url_with_query.next() {
            None => {
                object! {}
            }
            Some(q) => parse_kv(q),
        };

        let http_version = request_line.next().unwrap().to_string();

        Self {
            method,
            url,
            http_version,
            query,
        }
    }
}

impl From<&str> for RequestLine {
    fn from(request_line: &str) -> Self {
        RequestLine::parse(request_line)
    }
}

impl Display for RequestLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Request Line: \n\t{} {} {}\n\tQuery: {}",
            self.method, self.url, self.http_version, self.query
        )
    }
}