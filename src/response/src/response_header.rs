use json::{object, JsonValue};
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Header {
    content: JsonValue,
}

impl Header {
    pub fn new() -> Self {
        Self {
            content: object! {},
        }
    }
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

impl Display for Header {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.content
                .entries()
                .map(|(k, v)| { format!("{}: {}", k, v) })
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl From<JsonValue> for Header {
    fn from(value: JsonValue) -> Self {
        Header { content: value }
    }
}
