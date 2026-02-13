//! Zenith Standard Networking Module
//!
//! This module provides functions for network communication.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub struct NetworkError(pub String);

pub fn connect_tcp(addr: &str) -> Result<TcpStream, NetworkError> {
    TcpStream::connect(addr).map_err(|e| NetworkError(e.to_string()))
}

pub fn listen_tcp(addr: &str) -> Result<TcpListener, NetworkError> {
    TcpListener::bind(addr).map_err(|e| NetworkError(e.to_string()))
}

pub fn send_data(mut stream: TcpStream, data: &[u8]) -> Result<(), NetworkError> {
    stream
        .write_all(data)
        .map_err(|e| NetworkError(e.to_string()))
}

pub fn receive_data(mut stream: TcpStream) -> Result<Vec<u8>, NetworkError> {
    let mut buffer = Vec::new();
    stream
        .read_to_end(&mut buffer)
        .map_err(|e| NetworkError(e.to_string()))?;
    Ok(buffer)
}
