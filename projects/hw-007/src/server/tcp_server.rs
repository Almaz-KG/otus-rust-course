use crate::server::TcpSession;
use std::net::TcpListener;
use std::path::PathBuf;
use std::thread;

/// A TCP Server for smart home project. The internal logic of the connection is hidden by
/// [TcpSession] struct.
#[allow(unused)]
pub struct TcpServer {
    host: String,
    port: u16,
}

impl TcpServer {
    pub fn start(host: String, port: u16, _repo: PathBuf) {
        let listener = TcpListener::bind((host, port)).unwrap();
        let addr = listener.local_addr().unwrap();

        println!("Running server on {addr}");
        for stream in listener.incoming() {
            thread::spawn(move || {
                let stream = stream.unwrap();
                println!("[Server] Connected with {:?}", stream.local_addr().unwrap());
                let result = TcpSession::run(stream);
                if result.is_err() {
                    println!("[Server] Error: {}", result.err().unwrap())
                }

                println!("[Server] Connection closed")
            });
        }
    }
}
