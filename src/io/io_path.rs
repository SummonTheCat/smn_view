use std::{env, path::{Path, PathBuf}};

pub fn path_get_root() -> PathBuf {
    // Get the environment SMNVIEW_ROOT
    let env_root = env::var("SMNVIEW_ROOT").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(env_root)
}

pub fn get_extension(path: &PathBuf) -> Option<&str> {
    path.extension().and_then(|ext| ext.to_str())
}