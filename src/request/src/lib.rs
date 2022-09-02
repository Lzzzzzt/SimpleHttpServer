mod request_body;
mod request_header;
mod request_line;
mod utils;

use crate::request_body::Body;
use crate::request_header::Header;
use crate::request_line::RequestLine;
use json::{object, JsonValue};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Request {
    pub query: JsonValue,
    pub request_line: RequestLine,
    pub headers: Header,
    pub body: Body,
}
impl Request {
    pub fn parse(stream: &[u8]) -> Self {
        let temp = String::from_utf8_lossy(stream).to_string();

        let mut request_str = temp.split("\r\n\r\n");

        let mut request_headers = request_str.next().unwrap().split("\r\n");

        let request_line = RequestLine::parse(request_headers.next().unwrap());

        let query = request_line.query.clone();

        let headers: Header = request_headers.into();

        let content_len = headers["Content-Length"].as_usize().unwrap_or(0);

        let default_content_type = String::from("application/x-www-form-urlencoded");

        let content_type = headers["Content-Type"]
            .as_str()
            .unwrap_or(&default_content_type);

        let body_str = request_str.next().unwrap().split_at(content_len).0;

        let body = match content_type {
            "application/x-www-form-urlencoded" if !body_str.is_empty() => Body::parse(body_str)
                .unwrap_or(Body {
                    content: object! {},
                }),
            "application/json" if !body_str.is_empty() => {
                Body::parse_json(body_str).unwrap_or(Body {
                    content: object! {},
                })
            }
            _ if !body_str.is_empty() => {
                log::error!("Error: wrong `Content-Type`");
                Body {
                    content: object! {},
                }
            }
            _ => Body {
                content: object! {},
            },
        };

        Self {
            query,
            request_line,
            headers,
            body,
        }
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let request_line = self.request_line.to_string();
        let header = self.headers.to_string();
        let body = self.body.to_string();

        write!(f, "{}", [request_line, header, body].join("\r\n"))
    }
}
