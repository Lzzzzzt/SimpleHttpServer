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

#[cfg(test)]
mod test {
    use crate::Methods;
    use crate::Methods::{Get, Post};

    #[test]
    fn method_to_str() {
        assert_eq!(Get.to_string().as_str(), "GET");
        assert_eq!(Post.to_string().as_str(), "POST");
    }

    #[test]
    fn str_to_method() {
        let get: Methods = "GET".into();
        let post: Methods = "POST".into();
        let other: Methods = "AAA".into();

        assert_eq!(get.to_string(), Get.to_string());
        assert_eq!(post.to_string(), Post.to_string());
        assert_eq!(other.to_string(), Get.to_string());
    }
}
