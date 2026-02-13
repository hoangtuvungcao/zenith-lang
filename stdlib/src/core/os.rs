//! Zenith Standard OS Module
//!
//! This module provides functions for interacting with the operating system.

use std::env;

pub fn get_env(name: &str) -> Option<String> {
    env::var(name).ok()
}

pub fn set_env(name: &str, value: &str) {
    env::set_var(name, value);
}

pub fn args() -> Vec<String> {
    env::args().collect()
}

pub fn exit(code: i32) -> ! {
    std::process::exit(code)
}

pub fn current_dir() -> Result<String, String> {
    env::current_dir()
        .map(|path| path.to_string_lossy().to_string())
        .map_err(|e| e.to_string())
}
