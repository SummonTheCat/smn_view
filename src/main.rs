// src/main.rs

mod plugins;
mod server;
mod util;
mod window;
mod io;

use plugins::{plugin_statics::PluginStatics, plugin_ui::PluginUI};
use server::{plugin::plugin_manager::PluginManager, server_core::start_server};
use std::net::SocketAddr;
use util::logging::{log_line, log_line_header, logln, Color};
use window::{structs::struct_windowconfig::WindowConfig, window_core::start_window};

#[tokio::main]
async fn main() -> wry::Result<()> {
    let addr: SocketAddr = ([127, 0, 0, 1], 3030).into();
    let url = format!("http://{}/", addr);

    // Create the PluginManager and apply plugins
    logln("");
    log_line_header("Plugin Manager", Color::Cyan, 30);
    let mut plugin_manager = PluginManager::new();
    plugin_manager
        .apply_plugin(Box::new(PluginUI::new()))
        .expect("Failed to apply plugin");

    plugin_manager
        .apply_plugin(Box::new(PluginStatics::new("statics".to_string())))
        .expect("Failed to apply plugin");

    log_line(Color::Cyan, 30);
    logln("");

    // Start the server with the given address and PluginManager
    log_line_header("Server", Color::Cyan, 30);
    let mut server = start_server(addr, plugin_manager);
    server.await_ready();

    // Start the UI with the given URL
    // Once the window closes, trigger server shutdown
    let window_config = WindowConfig::default().set_title("SmnView").set_url(&url);

    start_window(window_config, || {
        server.shutdown();
    })?;

    server.await_shutdown();
    log_line(Color::Cyan, 30);

    Ok(())
}
