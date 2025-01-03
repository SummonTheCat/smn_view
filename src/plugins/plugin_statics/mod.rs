use std::io::{Result, Write};
use std::net::TcpStream;

use crate::server::plugin::plugin_base::Plugin;
use crate::server::structs::structs_mime::Mime;
use crate::server::structs::structs_request::Request;
use crate::server::structs::structs_response::Response;
use crate::util::logging::{logln, Color};

pub struct PluginStatics{
    pub path_statics: String,
}

impl PluginStatics {
    /// Creates a new instance of `PluginStatics`.
    pub fn new(path_statics: String) -> Self {
        Self {
            path_statics,
        }
    }
}

impl Plugin for PluginStatics {
    fn init(&mut self) -> Result<()> {
        logln(&format!("{} {}", Color::BrightBlack.paint("Plugin initialized: "), Color::BrightBlue.paint("PluginStatics")));
        Ok(())
    }

    fn catch(&self, request: &Request) -> bool {
        request.path == "/" && request.method == "GET" 
    }

    fn serve(&self, stream: &mut TcpStream, request: &Request) -> Result<()> {
        // Response: Path: {path},\n Method: {method}
        let msg = format!("Path: {},\nMethod: {} Static: {}", request.path, request.method, self.path_statics);
        let body = msg.as_bytes().to_vec();

        let mime = Mime::TextPlain;

        let mut response = Response::response_ok(body, mime);
        response.body = msg.as_bytes().to_vec();

        stream.write_all(&response.to_bytes())?;
        stream.flush()?;
        Ok(())
    }
}
