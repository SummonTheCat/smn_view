// src/server/plugin/plugin_manager.rs



use std::io::Result;
use std::sync::Arc;

use crate::server::structs::structs_request::Request;

use super::plugin_base::Plugin;

/// Manages a collection of plugins.
pub struct PluginManager {
    plugins: Vec<Arc<dyn Plugin + Send + Sync>>,
}

impl PluginManager {
    /// Creates a new `PluginManager` instance.
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    /// Applies a new plugin to the manager.
    pub fn apply_plugin(&mut self, mut plugin: Box<dyn Plugin + Send + Sync>) -> Result<()> {
        plugin.init()?;

        // Convert `Box<dyn Plugin + Send + Sync>` into `Arc<dyn Plugin + Send + Sync>`
        let arc_plugin: Arc<dyn Plugin + Send + Sync> = Arc::from(plugin);

        self.plugins.push(arc_plugin);
        Ok(())
    }

    /// Finds a plugin that can handle the given request.
    pub fn find_plugin(&self, request: &Request) -> Option<Arc<dyn Plugin + Send + Sync>> {
        for plugin in &self.plugins {
            if plugin.catch(request) {
                return Some(Arc::clone(plugin));
            }
        }
        None
    }
}
