use std::net::TcpListener;
use std::thread;
use std::path::PathBuf;
use crate::server::TcpSession;

/// A TCP Server for smart home project. It allows to interact with the wide range of the clients
/// by TCP connection. The internal logic of the connection is hidden by [TcpSession] struct.
#[allow(unused)]
pub struct TcpServer {
    host: String,
    port: u16,
    repo: String,
}

impl TcpServer {
    pub fn start(host: String, port: u16, _repo: PathBuf) {
        let listener = TcpListener::bind((host, port)).unwrap();
        let addr = listener.local_addr().unwrap();

        println!("Running server on {}", addr);
        for stream in listener.incoming() {
            thread::spawn(move || {
                let stream = stream.unwrap();
                println!("[Server] Connected with {:?}", stream.local_addr().unwrap());
                TcpSession::run(stream).unwrap();
                println!("[Server] Connection closed")
            });
        }
    }
}