use json::{object, JsonValue};
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::str::Split;

#[derive(Debug)]
pub struct Header {
    content: JsonValue,
}

impl Deref for Header {
    type Target = JsonValue;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

impl DerefMut for Header {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}

impl From<Split<'_, &str>> for Header {
    fn from(request_headers: Split<&str>) -> Self {
        let mut headers = object! {};

        request_headers.for_each(|header| {
            let mut header = header.split(':');
            let key = header.next().unwrap().trim();
            let value = header.next().unwrap().trim();

            headers.insert(key, value).unwrap();
        });

        Self { content: headers }
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.content
                .entries()
                .map(|(k, v)| { format!("{}: {}", k, v) })
                .collect::<Vec<String>>()
                .join("\r\n")
        )
    }
}

#[cfg(test)]
mod test {
    use crate::Header;
    use json::object;

    #[test]
    fn parse_header() {
        let header: Header = "Content-Length: 10\r\naccept: */*".split("\r\n").into();

        assert_eq!(
            header.content.to_string(),
            object! {
                "Content-Length": "10",
                "accept": "*/*"
            }
            .to_string()
        );
    }
}
