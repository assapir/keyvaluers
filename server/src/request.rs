use json::JsonValue;
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::net::TcpStream;

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
}

impl ParseError {
    fn new(message: &str) -> Self {
        Self {
            message: String::from(message),
        }
    }
}

pub struct Request {
    pub content: RequestContent,
    stream: TcpStream,
}

pub struct RequestContent {
    pub method: String,
    pub uri: String,
    pub headers: HashMap<String, String>,
    pub body: Option<JsonValue>,
}

impl RequestContent {
    fn parse(request: &str) -> Result<Self, ParseError> {
        let lines: Vec<&str> = request.lines().collect();
        if lines.len() < 1 {
            return Err(ParseError::new("First line is empty"));
        }
        let (method, uri) = match parse_first_line(lines[0]) {
            Ok(value) => value,
            Err(err) => return Err(err),
        };

        let mut headers = HashMap::new();
        for &line in lines.iter().skip(1) {
            if line.is_empty() {
                // Done with headers
                break;
            }
            if let Ok((key, value)) = parse_header(&line) {
                headers.insert(key, value);
            }
        }
        // We need to skip the first line + headers
        let rest = lines[headers.len() + 1..].join("\n");
        let body = match json::parse(&rest) {
            Ok(value) => Some(value),
            Err(_) => None,
        };

        Ok(RequestContent {
            method,
            uri,
            headers,
            body,
        })
    }
}

impl Request {
    pub fn new(mut stream: TcpStream) -> Result<Self, ParseError> {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let request = String::from_utf8_lossy(&buffer[..]);

        let content = match RequestContent::parse(&request) {
            Ok(value) => value,
            Err(value) => return Err(value),
        };

        Ok(Self { stream, content })
    }

    pub fn write(&mut self, message: &str) -> Result<usize, io::Error> {
        self.stream.write(message.as_bytes())
    }

    pub fn end(&mut self) -> Result<(), io::Error> {
        self.stream.flush()
    }
}

fn parse_first_line(first_line: &str) -> Result<(String, String), ParseError> {
    let mut split = first_line.split_whitespace();
    let method = match split.next() {
        Some(method) => String::from(method).to_uppercase(),
        None => return Err(ParseError::new("Unable to parse method")),
    };
    let uri = match split.next() {
        Some(method) => String::from(method).to_lowercase(),
        None => return Err(ParseError::new("Unable to parse uri")),
    };
    Ok((method, uri))
}

fn parse_header(line: &str) -> Result<(String, String), ParseError> {
    let (key, value) = match line.split_once(':') {
        Some(parts) => parts,
        None => return Err(ParseError::new("Unable to parse header")),
    };
    Ok((String::from(key.to_lowercase().trim()), String::from(value.trim())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_with_body() {
        let request_str = r#"PUT / HTTP/1.1
Host: localhost:7878
User-Agent: curl/7.64.1
Accept: */*
Content-Length: 2
Content-Type: application/x-www-form-urlencoded

{
    "name": "foo"
}"#;

        let content = RequestContent::parse(&request_str).unwrap();
        let body = json::object!{
            name: "foo",
        };
        assert_eq!("PUT", content.method);
        assert_eq!("/", content.uri);
        assert_eq!(content.headers.len(), 5);
        assert_eq!(content.headers.get("host").unwrap(), r#"localhost:7878"#);
        assert_eq!(content.headers.get("user-agent").unwrap(), r#"curl/7.64.1"#);
        assert_eq!(content.body.unwrap(), body);
    }
}
