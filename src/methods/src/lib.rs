use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Methods {
    Get,
    Post,
}

impl Display for Methods {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Methods::Get => {
                write!(f, "GET")
            }
            Methods::Post => {
                write!(f, "POST")
            }
        }
    }
}

impl From<&str> for Methods {
    fn from(method: &str) -> Self {
        match method {
            "GET" => Methods::Get,
            "POST" => Methods::Post,
            _ => Methods::Get,
        }
    }
}