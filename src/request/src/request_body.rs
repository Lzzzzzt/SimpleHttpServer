use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use json::{JsonError, JsonValue};
use crate::utils::parse_kv;

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
        write!(f, "Body: \n\t{}", self.content)
    }
}