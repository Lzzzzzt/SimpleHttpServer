use crate::status::Status;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct StatusLine {
    pub status: Status,
    pub http_version: String,
}

impl Display for StatusLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.http_version, self.status)
    }
}
