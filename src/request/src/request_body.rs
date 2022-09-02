use crate::utils::parse_kv;
use json::{JsonError, JsonValue};
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Body {
    pub content: JsonValue,
}

impl Body {
    pub fn parse_json(str: &str) -> Result<Self, JsonError> {
        let body = json::parse(str)?;

        Ok(Self { content: body })
    }

    pub fn parse(str: &str) -> Result<Self, JsonError> {
        Ok(Self {
            content: parse_kv(str),
        })
    }
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
        write!(f, "\r\n{}", self.content)
    }
}

#[cfg(test)]
mod test {
    use crate::Body;
    use json::object;

    #[test]
    fn parse_body_json() {
        let body = Body::parse_json("{\"test\": \"1\", \"hello\": \"hi\"}").unwrap();

        assert_eq!(
            body.content.to_string(),
            object! {
                test: "1",
                hello: "hi",
            }
            .to_string()
        )
    }

    #[test]
    fn parse_body_normal() {
        let body = Body::parse("test=1&hello=hi").unwrap();

        assert_eq!(
            body.content.to_string(),
            object! {
                test: "1",
                hello: "hi",
            }
            .to_string()
        )
    }
}
