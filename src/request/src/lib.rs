use json::{object, JsonError, JsonValue};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

type Headers = HashMap<String, String>;

#[derive(Debug)]
pub struct Request {
    pub query: JsonValue,
    pub request_line: RequestLine,
    pub headers: Headers,
    pub body: Body,
}
impl Request {
    pub fn parse(stream: &[u8]) -> Self {
        let temp = String::from_utf8_lossy(stream).to_string();

        let mut request_str = temp.split("\r\n\r\n");

        let mut request_headers = request_str.next().unwrap().split("\r\n");

        let request_line = RequestLine::parse(request_headers.next().unwrap());

        let query = request_line.query.clone();

        let mut headers = Headers::new();

        request_headers.for_each(|header| {
            let mut header = header.split(':');
            let key = header.next().unwrap().trim().to_string();
            let value = header.next().unwrap().trim().to_string();

            headers.insert(key, value);
        });

        let content_len = headers
            .get("Content-Length")
            .unwrap_or(&String::from('0'))
            .parse::<usize>()
            .unwrap();

        let default_content_type = String::from("application/x-www-form-urlencoded");

        let content_type = headers
            .get("Content-Type")
            .unwrap_or(&default_content_type)
            .as_str();

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
                println!("Error: wrong `Content-Type`");
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
        let header = self
            .headers
            .iter()
            .map(|(k, v)| format!("\t{}: {}", k, v))
            .collect::<Vec<String>>()
            .join("\n");
        let body = self.body.to_string();

        write!(
            f,
            "{}",
            [request_line, String::from("Headers:"), header, body].join("\n")
        )
    }
}



#[derive(Debug)]
pub struct RequestLine {
    pub method: Methods,
    pub url: String,
    pub http_version: String,
    pub query: JsonValue,
}

impl RequestLine {
    fn parse(request_line: &str) -> Self {
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

impl Display for RequestLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Request Line: \n\t{} {} {}\n\tQuery: {}",
            self.method, self.url, self.http_version, self.query
        )
    }
}

#[derive(Debug)]
pub struct Body {
    content: JsonValue,
}

impl Body {
    fn parse_json(str: &str) -> Result<Self, JsonError> {
        let body = json::parse(str)?;

        Ok(Self { content: body })
    }

    fn parse(str: &str) -> Result<Self, JsonError> {
        Ok(Self {
            content: parse_kv(str),
        })
    }
}

fn parse_kv(str: &str) -> JsonValue {
    let mut obj = object! {};

    str.split('&').for_each(|kv| {
        let mut kv = kv.split('=');
        let key = kv.next().unwrap();
        let value = kv.next().unwrap();

        obj.insert(key, value).unwrap();
    });

    obj
}

impl Deref for Body {
    type Target = JsonValue;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

impl DerefMut for Body {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}

impl Display for Body {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Body: \n\t{}", self.content)
    }
}
