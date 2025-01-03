// src/server/structs/structs_response.rs

use std::collections::HashMap;
use std::fmt;

use crate::util::logging::logln;

use super::structs_header::StatusCode;
use super::structs_mime::Mime;

/// Represents an HTTP response.
#[derive(Default, Debug)]
pub struct Response {
    pub status_code: u16,
    pub status_message: String,
    pub header_fields: HashMap<String, String>,
    pub body: Vec<u8>,
}

/// Custom error type for Response operations.
#[derive(Debug)]
pub enum ResponseError {
    InvalidStatusCode(u16),
    InvalidHeader(String),
}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResponseError::InvalidStatusCode(code) => write!(f, "Invalid status code: {}", code),
            ResponseError::InvalidHeader(header) => write!(f, "Invalid header: {}", header),
        }
    }
}

impl std::error::Error for ResponseError {}

impl Response {
    /// Creates a new `Response` with the given status code and message.
    pub fn new(status_code: u16, status_message: &str) -> Self {
        Self {
            status_code,
            status_message: status_message.to_string(),
            header_fields: HashMap::new(),
            body: Vec::new(),
        }
    }

    /// Sets a header field, replacing any existing value.
    
    pub fn set_header(&mut self, key: &str, value: &str) {
        self.header_fields.insert(key.to_string(), value.to_string());
    }

    /// Adds a header field. If the header already exists, appends the value separated by a comma.
   
    pub fn add_header(&mut self, key: &str, value: &str) {
        let key_lower = key.to_lowercase();
        self.header_fields
            .entry(key_lower)
            .and_modify(|existing| {
                existing.push_str(", ");
                existing.push_str(value);
            })
            .or_insert_with(|| value.to_string());
    }

    /// Sets the body of the response with raw bytes.
    pub fn set_body(&mut self, body: Vec<u8>) {
        self.body = body;
    }

    /// Sets the body of the response from a string slice.
    pub fn set_body_from_str(&mut self, body: &str) {
        self.body = body.as_bytes().to_vec();
    }

    /// Serializes the `Response` into a byte vector suitable for sending over a network.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut response = Vec::new();

        // Status line
        let status_line = format!("HTTP/1.1 {} {}\r\n", self.status_code, self.status_message);
        response.extend_from_slice(status_line.as_bytes());

        // Headers
        for (key, value) in &self.header_fields {
            let header_line = format!("{}: {}\r\n", key, value);
            response.extend_from_slice(header_line.as_bytes());
        }

        // Blank line to indicate end of headers
        response.extend_from_slice(b"\r\n");

        // Body
        response.extend_from_slice(&self.body);

        response
    }
}


impl Response {
    // Response presets
    pub fn response_ok(body: Vec<u8>, mime: Mime) -> Self {
        let code = StatusCode::Ok.to_code();
        let msg = StatusCode::Ok.to_msg();
        let mut response = Self::new(code, msg);
        response.set_header("Content-Type", mime.to_string());

        response.set_body(body);
        response
    }

    pub fn response_error (error: String, code: StatusCode) -> Self {
        let mut response = Self::new(code.to_code(), code.to_msg());
        response.set_header("Content-Type", Mime::TextPlain.to_string());

        response.set_body(error.to_string().as_bytes().to_vec());
        response
    }
}