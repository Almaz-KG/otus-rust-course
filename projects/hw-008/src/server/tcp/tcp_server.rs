use crate::server::TcpSession;
use std::net::TcpListener;
use std::thread;
use std::thread::JoinHandle;

/// A TCP Server for smart home project. The internal logic of the connection is hidden by
/// [TcpSession] struct.
#[allow(unused)]
pub struct TcpServer {
    host: String,
    port: u16,
}

impl TcpServer {
    pub fn start(host: String, port: u16) -> JoinHandle<()> {
        let listener = TcpListener::bind((host, port)).unwrap();
        let addr = listener.local_addr().unwrap();

        thread::spawn(move || {
            println!("Running Tcp server on {}", addr);

            for stream in listener.incoming() {
                thread::spawn(move || {
                    let stream = stream.unwrap();
                    println!("[TcpServer] Connected with {:?}", stream.local_addr().unwrap());
                    let result = TcpSession::run(stream);
                    if result.is_err() {
                        println!("[TcpServer] Error: {}", result.err().unwrap())
                    }

                    println!("[TcpServer] Connection closed")
                });
            }
        })
    }
}
