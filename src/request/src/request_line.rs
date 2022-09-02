use crate::utils::parse_kv;
use json::{object, JsonValue};
use methods::Methods;
use std::fmt::{Display, Formatter};

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

#[cfg(test)]
mod test {
    use crate::RequestLine;
    use json::object;
    use methods::Methods;

    #[test]
    fn parse_get_request_without_query() {
        let req: RequestLine = "GET / HTTP/1.1".into();

        assert_eq!(req.url, "/".to_string());
        assert_eq!(req.method.to_string(), Methods::Get.to_string());
        assert_eq!(req.http_version, "HTTP/1.1".to_string());
        assert_eq!(req.query.to_string(), object! {}.to_string());
    }

    #[test]
    fn parse_get_request_with_query() {
        let req: RequestLine = "GET /hello?name=rust&age=7 HTTP/1.1".into();

        assert_eq!(req.url, "/hello".to_string());
        assert_eq!(req.method.to_string(), Methods::Get.to_string());
        assert_eq!(req.http_version, "HTTP/1.1".to_string());
        assert_eq!(
            req.query.to_string(),
            object! {name: "rust", age: "7"}.to_string()
        )
    }
}
