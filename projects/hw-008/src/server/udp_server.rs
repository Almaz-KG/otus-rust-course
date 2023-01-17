use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::entities::devices::Device;
use crate::entities::Measure;

/// An Udp Server which serves for the devices which might use UDP protocol.
#[allow(unused)]
pub struct UdpServer {
    host: String,
    port: u16,
    devices: Vec<Device>,
    active_connections: Vec<SocketAddr>,
}

impl UdpServer {
    fn listen(server: Arc<Mutex<UdpServer>>, socket: UdpSocket) {
        thread::spawn(move || {
            let mut buf: [u8; 20] = [0; 20];
            let mut result: Vec<u8> = Vec::new();
            match socket.recv_from(&mut buf) {
                Ok((number_of_bytes, src_addr)) => {
                    result = Vec::from(&buf[0..number_of_bytes]);
                    let mut server = server.lock().unwrap();
                    server.active_connections.push(src_addr);
                }
                Err(fail) => println!("failed listening {:?}", fail)
            }

            let display_result = result.clone();
            let result_str = String::from_utf8(display_result).unwrap();
            println!("received message: {:?}", result_str);
        });
    }

    fn send_datagram(addr: &SocketAddr, value: &str) -> Result<usize, String> {
        let content = value.as_bytes();
        let host = format!("{}:{}", addr.ip(), addr.port());

        let socket = UdpSocket::bind(host).unwrap();
        socket.set_write_timeout(Some(Duration::from_secs(2))).unwrap();
        socket.send(&content)
            .map_err(|e| format!("{}", e))
    }

    fn send_updates(server: Arc<Mutex<UdpServer>>) {
        let thread = thread::spawn(move || {
            loop {
                let mut server = server.lock().unwrap();

                let mut dropped_connections = vec![];

                for device in server.devices.iter()  {
                    match device {
                        Device::Socket(_) => {}
                        Device::Thermometer(therm) => {
                            for addr in server.active_connections.iter() {
                                let id = therm.id.clone();
                                let value = therm.measure().unwrap().unwrap();
                                let measure = format!("{}|{}", id, value);
                                let send_r = UdpServer::send_datagram(addr, &measure);

                                match send_r {
                                    Ok(_) => {} // do nothing
                                    Err(_) => { // remove from the list of active connections
                                        dropped_connections.push(addr);
                                    }
                                }
                            }
                        }
                    }
                }
                let mut connections = server.active_connections.clone();
                connections
                    .retain(|ac| dropped_connections.iter().any(|c| ac == *c));

                server.active_connections = connections;

                thread::sleep(Duration::from_secs(2))
            }
        });
    }

    pub fn start(host: String, port: u16, devices: Vec<Device>) {
        println!("C");
        let udp_socket_result = UdpSocket::bind((host.clone(), port));
        println!("D");

        match udp_socket_result {
            Ok(socket) => {
                println!("E");
                let mut server = UdpServer {
                    host,
                    port,
                    devices,
                    active_connections: vec![],
                };

                let server = Arc::new(Mutex::new(server));
                UdpServer::listen(server.clone(), socket);
                println!("E");
                UdpServer::send_updates(server.clone());
                println!("Running Udp server...")
            }
            Err(_) => {
                eprintln!("Unable to start Udp Server...")
            }
        }
    }
}
