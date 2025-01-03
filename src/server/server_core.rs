// src/server/server_core.rs

use std::{net::SocketAddr, thread};
use tokio::{net::TcpListener, io::{AsyncReadExt, AsyncWriteExt}, sync::oneshot};
use crate::{server::structs::{structs_header::StatusCode, structs_request::Request, structs_response::Response}, util::logging::{logln, logln_color, Color}};
use std::sync::Arc;

use super::plugin::plugin_manager::PluginManager;

/// Represents the server.
pub struct Server {
    pub ready_rx: oneshot::Receiver<()>,
    pub handle: Option<thread::JoinHandle<()>>,
    pub _shutdown_tx: Option<oneshot::Sender<()>>,
}

impl Server {
    /// Sends a shutdown signal to the server.
    pub fn shutdown(&mut self) {
        if let Some(tx) = self._shutdown_tx.take() {
            let _ = tx.send(());
        } else {
            logln("Shutdown already triggered or not available.");
        }
    }

    /// Waits for the server to shut down.
    pub fn await_shutdown(&mut self) {
        if let Some(handle) = self.handle.take() {
            if let Err(err) = handle.join() {
                logln(&format!("Server thread panicked: {:?}", err));
            } else {
                logln_color("[Ended: Server]", Color::Green);
            }
        } else {
            logln("Server already shut down or join called multiple times.");
        }
    }

    /// Waits until the server is ready to accept connections.
    pub fn await_ready(&mut self) {
        if let Ok(_) = self.ready_rx.try_recv() {
            // Server is ready
        }
    }
}

/// Starts the server on the specified address with the provided `PluginManager`.
pub fn start_server(addr: SocketAddr, plugin_manager: PluginManager) -> Server {
    let (ready_tx, ready_rx) = oneshot::channel();
    let (shutdown_tx, mut shutdown_rx) = oneshot::channel::<()>();

    // Move `PluginManager` into the server thread
    let server_handle = thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
        rt.block_on(async move {
            let listener = TcpListener::bind(addr).await.expect("Failed to bind");

            logln_color("[Started: Server]", Color::Green);
            // Print the server address and http:// URL
            logln(&format!("{} {}", Color::BrightBlack.paint("Server listening on:"), Color::Blue.paint(&format!("http://{}", addr))));
            let _ = ready_tx.send(());

            // Wrap `PluginManager` in an `Arc` for shared ownership across tasks
            let plugin_manager = Arc::new(plugin_manager);

            loop {
                tokio::select! {
                    _ = &mut shutdown_rx => {
                        // Shutdown signal received
                        break;
                    }
                    accept_result = listener.accept() => {
                        match accept_result {
                            Ok((mut stream, _peer)) => {
                                // Clone the `PluginManager` Arc for the task
                                let plugin_manager = Arc::clone(&plugin_manager);
                                tokio::spawn(async move {
                                    // Read the request
                                    let mut buf = [0u8; 1024];
                                    let n = stream.read(&mut buf).await.unwrap_or(0);
                                    if n == 0 {
                                        // Connection closed
                                        return;
                                    }
                                    match Request::from_bytes(&buf[..n]) {
                                        Ok(request) => {
                                            // Find a plugin to handle the request
                                            if let Some(plugin) = plugin_manager.find_plugin(&request) {
                                                // Convert `tokio::net::TcpStream` to `std::net::TcpStream`
                                                let std_stream = match stream.into_std() {
                                                    Ok(s) => s,
                                                    Err(e) => {
                                                        logln(&format!("Failed to convert stream: {}", e));
                                                        return;
                                                    }
                                                };
                                                let mut std_stream = std_stream;
                                                if let Err(e) = plugin.serve(&mut std_stream, &request) {
                                                    logln(&format!("Plugin serve error: {}", e));
                                                }
                                            } else {
                                                // No plugin found, respond with No Content
                                                let response = Response::response_error("No Content".to_owned(), StatusCode::NotFound).to_bytes();
                                                let _ = stream.write_all(&response).await;
                                                let _ = stream.flush().await;
                                            }
                                        }
                                        Err(e) => {
                                            logln(&format!("Failed to parse request: {}", e));
                                            // Invalid request, respond with Bad Request
                                            let response = Response::response_error("Bad Request".to_owned(), StatusCode::BadRequest).to_bytes();
                                            let _ = stream.write_all(&response).await;
                                            let _ = stream.flush().await;
                                        }
                                    }
                                });
                            }
                            Err(e) => {
                                logln(&format!("Failed to accept connection: {}", e));
                            }
                        }
                    }
                }
            }
        });
    });

    Server {
        ready_rx,
        handle: Some(server_handle),
        _shutdown_tx: Some(shutdown_tx),
    }
}
