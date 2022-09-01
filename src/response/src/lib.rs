use std::fs;
use std::io::Error;

const NOT_FOUND: &str = "<h1>404 NOT FOUND!</h1>";

// type Headers = HashMap<String, String>;

#[derive(Debug)]
pub struct Response {
    pub http_version: String,
    pub status: String,
    pub content_type: Option<String>,
    pub content: Option<Vec<u8>>,
}

impl Response {
    pub fn new(
        http_version: &str,
        status: String,
        content: Option<Vec<u8>>,
        content_type: Option<String>,
    ) -> Self {
        Self {
            http_version: http_version.to_string(),
            status,
            content,
            content_type,
        }
    }

    // pub fn redirect_response(target: &str) -> Self {
    //     Response::new("HTTP/1.1", "302")
    // }

    pub fn file_response(file: &str) -> Result<Self, Error> {
        let content = fs::read(file)?;
        let content_type = Self::parse_file_mime_type(file);

        Ok(Self::new(
            "HTTP/1.1",
            String::from("200 OK"),
            Some(content),
            Some(content_type),
        ))
    }

    pub fn response_404(file: Option<&str>) -> Self {
        let file = file.unwrap_or("");

        let mut response = Self::file_response(file).unwrap_or_else(|_| {
            Response::new(
                "HTTP/1.1",
                String::from("404 NOT FOUND"),
                Some(NOT_FOUND.as_bytes().to_vec()),
                Some(String::from("text/html")),
            )
        });
        response.status = "404 NOT FOUND".to_string();
        response
    }

    pub fn as_bytes(&mut self) -> Vec<u8> {
        let response = match &self.content {
            None => {
                format!(
                    "{} {}\r\nContent-Length: 0\r\n\r\n",
                    self.http_version, self.status,
                )
            }
            Some(content) => {
                let mime_type = self.content_type.clone().unwrap();
                format!(
                    "{} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
                    self.http_version,
                    self.status,
                    mime_type,
                    content.len(),
                )
            }
        };

        let mut res = response.into_bytes();

        let content = self.content.take();

        res.append(&mut content.unwrap());

        res
    }

    fn parse_file_mime_type(file: &str) -> String {
        let file_name = file.to_string();
        let ext_name = file_name.split('.').last().unwrap_or("txt");

        match ext_name {
            "html" => "text/html".to_string(),
            "css" => "text/css".to_string(),
            "js" => "text/javascript".to_string(),
            "map" => "application/json".to_string(),
            "jpg" | "jpeg" => "image/jpeg".to_string(),
            "png" => "image/png".to_string(),
            "ico" => "image/x-icon".to_string(),
            "svg" => "image/svg+xml".to_string(),
            _ => "text/plain".to_string(),
        }
    }
}

// pub struct ResponseLine {
//     pub method: Method
// }
