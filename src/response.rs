use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Response {
    pub status_code: u16,
    pub reason: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Response {
    pub fn new(status_code: u16, body: impl Into<String>) -> Self {
        let reason = match status_code {
            200 => "OK".to_string(),
            201 => "Created".to_string(),
            204 => "No Content".to_string(),
            400 => "Bad Request".to_string(),
            404 => "Not Found".to_string(),
            405 => "Method Not Allowed".to_string(),
            500 => "Internal Server Error".to_string(),
            _ => "OK".to_string(),
        };

        let body = body.into();
        let mut headers = HashMap::new();
        headers.insert("Content-Length".to_string(), body.len().to_string());
        headers.insert("Connection".to_string(), "close".to_string());

        Self {
            status_code,
            reason,
            headers,
            body,
        }
    }

    pub fn text(status_code: u16, body: impl Into<String>) -> Self {
        let mut response = Self::new(status_code, body);
        response.headers.insert("Content-Type".to_string(), "text/plain".to_string());
        response
    }

    pub fn json(status_code: u16, body: impl Into<String>) -> Self {
        let mut response = Self::new(status_code, body);
        response.headers.insert("Content-Type".to_string(), "application/json".to_string());
        response
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut output = format!("HTTP/1.1 {} {}\r\n", self.status_code, self.reason);
        for (name, value) in &self.headers {
            output.push_str(&format!("{name}: {value}\r\n"));
        }
        output.push_str("\r\n");
        output.push_str(&self.body);
        output.into_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_plain_text_response() {
        let response = Response::text(200, "hello");
        let bytes = response.to_bytes();
        let output = String::from_utf8(bytes).unwrap();

        assert!(output.starts_with("HTTP/1.1 200 OK\r\n"));
        assert!(output.contains("Content-Type: text/plain"));
    }
}
