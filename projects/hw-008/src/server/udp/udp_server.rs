use crate::entities::devices::Device;
use crate::entities::manager::SmartHomeManager;
use crate::entities::Measure;
use std::net::{SocketAddr, UdpSocket};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub const SEND_INTERVAL: u64 = 2;

/// An Udp Server which serves for the devices which might use UDP protocol.
#[allow(unused)]
pub struct UdpServer {
    active_connections: Mutex<Vec<SocketAddr>>,
    socket: Mutex<UdpSocket>,
    manager: SmartHomeManager,
}

impl UdpServer {
    fn listen(server: Arc<UdpServer>) {
        thread::spawn(move || loop {
            let mut buf: [u8; 20] = [0; 20];
            let socket = server.socket.lock().unwrap();

            match socket.recv_from(&mut buf) {
                Ok((_, src_addr)) => {
                    println!("[UdpServer] Connected with {}", src_addr);
                    server.active_connections.lock().unwrap().push(src_addr);
                }
                Err(_) => {
                    drop(socket);
                    thread::sleep(Duration::from_secs(SEND_INTERVAL));
                }
            }
        });
    }

    fn send_updates(server: Arc<UdpServer>) {
        let _thread = thread::spawn(move || loop {
            let devices = server.manager.list_all_devices().unwrap();
            let connections = server.active_connections.lock().unwrap();

            if devices.is_empty() || connections.is_empty() {
                thread::sleep(Duration::from_secs(SEND_INTERVAL));
                drop(connections);
                continue;
            }

            for device in devices.iter() {
                match device {
                    Device::Socket(_) => {}
                    Device::Thermometer(therm) => {
                        let socket = server.socket.lock().unwrap();
                        for addr in connections.iter() {
                            let id = therm.id.clone();
                            let value = therm.measure().unwrap().unwrap();
                            let measure = format!("[{}]: {}\n", id, value);

                            socket.send_to(measure.as_bytes(), addr).unwrap();
                        }
                    }
                }
            }
            drop(connections);
            thread::sleep(Duration::from_secs(SEND_INTERVAL));
        });
    }

    pub fn start(host: String, port: u16, repo: PathBuf) {
        let udp_socket_result = UdpSocket::bind((host, port));
        match udp_socket_result {
            Ok(socket) => {
                let addr = socket.local_addr().unwrap();
                let manager = SmartHomeManager::new(repo);
                socket.set_nonblocking(true).unwrap();

                let server = UdpServer {
                    socket: Mutex::new(socket),
                    active_connections: Mutex::new(vec![]),
                    manager,
                };

                let server = Arc::new(server);
                UdpServer::listen(server.clone());
                UdpServer::send_updates(server);

                println!("Running Udp server on {}", addr);
            }
            Err(_) => {
                eprintln!("Unable to start Udp Server...")
            }
        };
    }
}
