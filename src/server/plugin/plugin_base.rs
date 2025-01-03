// src/server/plugin/plugin_base.rs

use std::io::Result;
use std::net::TcpStream;

use crate::server::structs::structs_request::Request;

/// The `Plugin` trait defines the necessary methods that all plugins must implement.
pub trait Plugin: Send + Sync + 'static {
    /// Initializes the plugin. Called when the plugin is added to the server.
    fn init(&mut self) -> Result<()>;

    /// Determines if the plugin should handle the current request.
    /// Returns `true` if the plugin will handle the request.
    fn catch(&self, request: &Request) -> bool;

    /// Serves the request by writing the response to the stream.
    fn serve(&self, stream: &mut TcpStream, request: &Request) -> Result<()>;
}
