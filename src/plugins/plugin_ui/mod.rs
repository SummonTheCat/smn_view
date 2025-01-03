use std::io::{Result, Write};
use std::net::TcpStream;

use crate::io::io_file::file_read_bytes;
use crate::io::io_path::{get_extension, path_get_root};
use crate::server::plugin::plugin_base::Plugin;
use crate::server::structs::structs_header::StatusCode;
use crate::server::structs::structs_mime::Mime;
use crate::server::structs::structs_request::Request;
use crate::server::structs::structs_response::Response;
use crate::util::logging::{logln, Color};

pub struct PluginUI;

impl PluginUI {
    /// Creates a new instance of `PluginUI`.
    pub fn new() -> Self {
        Self {}
    }
}

impl Plugin for PluginUI {
    fn init(&mut self) -> Result<()> {
        logln(&format!(
            "{} {}",
            Color::BrightBlack.paint("Plugin initialized: "),
            Color::BrightBlue.paint("PluginUI")
        ));
        Ok(())
    }

    fn catch(&self, request: &Request) -> bool {
        request.path == "/" && request.method == "GET"
    }

    fn serve(&self, stream: &mut TcpStream, _request: &Request) -> Result<()> {
        let path_landing = path_get_root().join("statics/index.html");

        logln(&format!(
            "{} {}",
            Color::BrightBlack.paint("Serving: "),
            Color::BrightBlue.paint(path_landing.to_str().unwrap())
        ));

        let content = match file_read_bytes(path_landing.as_path()) {
            Ok(content) => content,
            Err(_) => {
                let response = Response::response_error("File not found".to_owned(), StatusCode::NotFound);
                stream.write_all(&response.to_bytes())?;
                return Ok(());
            }
        };

        let extension = get_extension(&path_landing).unwrap_or("html");
        let mime = Mime::from_extension(extension);

        let response = Response::response_ok(content, mime);

        stream.write_all(&response.to_bytes())?;
        stream.flush()?;
        Ok(())
    }
}
