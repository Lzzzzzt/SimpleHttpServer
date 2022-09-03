use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Status {
    code: u16,
    message: String,
}

impl Status {
    pub fn ok() -> Self {
        Self {
            code: 200,
            message: String::from("OK"),
        }
    }

    pub fn moved_permanently() -> Self {
        Self {
            code: 301,
            message: String::from("Moved Permanently"),
        }
    }

    pub fn found() -> Self {
        Self {
            code: 302,
            message: String::from("Found"),
        }
    }

    pub fn bad_request() -> Self {
        Self {
            code: 400,
            message: String::from("Bad Request"),
        }
    }

    pub fn unauthorized() -> Self {
        Self {
            code: 401,
            message: String::from("Unauthorized"),
        }
    }

    pub fn forbidden() -> Self {
        Self {
            code: 403,
            message: String::from("Forbidden"),
        }
    }

    pub fn not_found() -> Self {
        Self {
            code: 404,
            message: String::from("Not Found"),
        }
    }

    pub fn internal_server_error() -> Self {
        Self {
            code: 500,
            message: String::from("Internal Server Error"),
        }
    }

    pub fn server_unavailable() -> Self {
        Self {
            code: 503,
            message: String::from("Server Unavailable"),
        }
    }
}

impl From<&str> for Status {
    fn from(str: &str) -> Self {
        if str.starts_with("200") {
            Status::ok()
        } else if str.starts_with("400") {
            Status::bad_request()
        } else if str.starts_with("401") {
            Status::unauthorized()
        } else if str.starts_with("403") {
            Status::forbidden()
        } else if str.starts_with("404") {
            Status::not_found()
        } else if str.starts_with("500") {
            Status::internal_server_error()
        } else if str.starts_with("503") {
            Status::server_unavailable()
        } else {
            Status::internal_server_error()
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.code, self.message)
    }
}
