use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Other(String),
}

impl Default for Method {
    fn default() -> Self {
        Self::Get
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::Get => write!(f, "GET"),
            Method::Post => write!(f, "POST"),
            Method::Put => write!(f, "PUT"),
            Method::Patch => write!(f, "PATCH"),
            Method::Delete => write!(f, "DELETE"),
            Method::Other(value) => write!(f, "{value}"),
        }
    }
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value.to_ascii_uppercase().as_str() {
            "GET" => Method::Get,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "PATCH" => Method::Patch,
            "DELETE" => Method::Delete,
            _ => Method::Other(value.to_string()),
        }
    }
}

#[derive(Debug, Default)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub params: HashMap<String, String>,
}

impl Request {
    pub fn path_without_query(&self) -> &str {
        self.path.split('?').next().unwrap_or(&self.path)
    }
}

pub fn parse_request(buffer: &[u8]) -> Result<Request, String> {
    let text = String::from_utf8_lossy(buffer);
    let mut lines = text.split("\r\n");
    let request_line = lines.next().ok_or("missing request line")?;

    let mut parts = request_line.split_whitespace();
    let method = parts.next().ok_or("missing HTTP method")?;
    let path = parts.next().ok_or("missing request path")?;
    let version = parts.next().ok_or("missing HTTP version")?;

    let mut headers = HashMap::new();
    let mut body = String::new();

    let mut saw_blank_line = false;
    for line in lines {
        if !saw_blank_line && line.is_empty() {
            saw_blank_line = true;
            continue;
        }

        if saw_blank_line {
            body.push_str(line);
            body.push('\n');
            continue;
        }

        if let Some((name, value)) = line.split_once(':') {
            headers.insert(name.trim().to_lowercase(), value.trim().to_string());
        }
    }

    Ok(Request {
        method: Method::from(method),
        path: path.to_string(),
        version: version.to_string(),
        headers,
        body: body.trim_end().to_string(),
        params: HashMap::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_simple_get_request() {
        let bytes = b"GET /hello HTTP/1.1\r\nHost: example.com\r\n\r\n";
        let request = parse_request(bytes).unwrap();

        assert_eq!(request.method, Method::Get);
        assert_eq!(request.path, "/hello");
        assert_eq!(request.headers.get("host"), Some(&"example.com".to_string()));
    }
}
