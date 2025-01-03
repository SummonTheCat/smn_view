// src/server/structs/structs_http.rs

use crate::util::logging::logln;
use std::collections::HashMap;

/// Represents an HTTP request with minimal parsing.
#[derive(Default)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub header_fields: HashMap<String, String>, // Explicit type annotations
    pub body: Vec<u8>, // New field to store the request body
}

/// Custom error type for Request operations.
#[derive(Debug)]
pub enum RequestError {
    HeaderNotFound(String),
    InvalidRequest(String),
}

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestError::HeaderNotFound(header) => write!(f, "Header '{}' not found", header),
            RequestError::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
        }
    }
}

impl std::error::Error for RequestError {}

impl Request {
    /// Parses a raw HTTP request string and returns a `Request` instance.
    pub fn from_string(request: &str) -> Result<Self, RequestError> {
        let request_bytes = request.as_bytes();
        Self::from_bytes(request_bytes)
    }

    /// Parses a raw HTTP request from bytes and returns a `Request` instance.
    pub fn from_bytes(request: &[u8]) -> Result<Self, RequestError> {
        let mut lines = request.split(|&b| b == b'\r' || b == b'\n');

        // Parse request line
        let request_line = lines
            .next()
            .ok_or_else(|| RequestError::InvalidRequest("Empty request".to_string()))?;
        let request_line_str = String::from_utf8_lossy(request_line);
        let mut parts = request_line_str.split_whitespace();

        let method = parts
            .next()
            .ok_or_else(|| RequestError::InvalidRequest("Missing HTTP method".to_string()))?
            .to_string();
        let path = parts
            .next()
            .ok_or_else(|| RequestError::InvalidRequest("Missing request path".to_string()))?
            .to_string();
        // Optionally, parse HTTP version if needed
        // let _version = parts.next().ok_or_else(|| RequestError::InvalidRequest("Missing HTTP version".to_string()))?.to_string();

        let mut header_fields: HashMap<String, String> = HashMap::new();
        let mut current_header = None;

        // Parse headers
        for line in &mut lines {
            if line.is_empty() {
                break; // End of headers
            }

            let line_str = String::from_utf8_lossy(line);
            if line_str.starts_with(' ') || line_str.starts_with('\t') {
                // Continuation of the previous header
                if let Some(ref header_name) = current_header {
                    if let Some(header_value) = header_fields.get_mut(header_name) {
                        header_value.push(' ');
                        header_value.push_str(line_str.trim());
                    }
                }
            } else {
                if let Some((key, value)) = line_str.split_once(':') {
                    let key = key.trim().to_lowercase(); // Normalize to lowercase for case-insensitive matching
                    let value = value.trim().to_string();
                    header_fields.insert(key.clone(), value);
                    current_header = Some(key);
                } else {
                    return Err(RequestError::InvalidRequest(format!(
                        "Invalid header line: {}",
                        line_str
                    )));
                }
            }
        }

        // Parse body based on Content-Length
        let body = if let Some(content_length_str) = header_fields.get("content-length") {
            let content_length: usize = content_length_str.parse().map_err(|_| {
                RequestError::InvalidRequest("Invalid Content-Length value".to_string())
            })?;

            logln(&format!("Content-Length: {}", content_length));

            let mut body = Vec::with_capacity(content_length);
            for _ in 0..content_length {
                if let Some(&byte) = request.get(lines.clone().count()) {
                    body.push(byte);
                }
            }

            // Since we've already split the request into lines, we'll reconstruct the body.
            // Alternatively, you can handle the body differently based on your server's implementation.
            // For simplicity, we'll assume the body starts after the first empty line.

            let body_start = request
                .windows(4)
                .position(|window| window == b"\r\n\r\n")
                .map(|pos| pos + 4)
                .unwrap_or(request.len());

            request[body_start..].to_vec()
        } else {
            Vec::new()
        };

        Ok(Self {
            method,
            path,
            header_fields,
            body,
        })
    }

    /// Retrieves the value of a specific header.
    ///
    /// # Arguments
    ///
    /// * `header_name` - The name of the header to retrieve.
    ///
    /// # Returns
    ///
    /// * `Ok(&str)` containing the header value if found.
    /// * `Err(RequestError)` with an error message if the header is not found.
    pub fn get_header_value<'a>(&'a self, header_name: &str) -> Result<&'a str, RequestError> {
        self.header_fields
            .get(&header_name.to_lowercase())
            .map(|value| value.as_str())
            .ok_or_else(|| RequestError::HeaderNotFound(header_name.to_string()))
    }

    /// Retrieves the value of a specific header or returns a default value if not found.
    ///
    /// # Arguments
    ///
    /// * `header_name` - The name of the header to retrieve.
    /// * `default` - The default value to return if the header is not found.
    ///
    /// # Returns
    ///
    /// * `&str` containing the header value if found, otherwise the default value.
    pub fn get_header_value_or_default<'a>(
        &'a self,
        header_name: &str,
        default: &'a str,
    ) -> &'a str {
        self.header_fields
            .get(&header_name.to_lowercase())
            .map(|value| value.as_str())
            .unwrap_or(default)
    }

    /// Retrieves the request body as a byte slice.
    ///
    /// # Returns
    ///
    /// * `&[u8]` containing the request body.
    pub fn get_body(&self) -> &[u8] {
        &self.body
    }

    /// Retrieves the request body as a string, if valid UTF-8.
    ///
    /// # Returns
    ///
    /// * `Ok(&str)` containing the body as a string.
    /// * `Err(std::str::Utf8Error)` if the body is not valid UTF-8.
    pub fn get_body_as_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.body)
    }
}

// Debug implementation for Request
impl std::fmt::Debug for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Request")
            .field("method", &self.method)
            .field("path", &self.path)
            .field("header_fields", &self.header_fields)
            .field("body", &format!("{:?}", self.body))
            .finish()
    }
}
