use crate::clients::UdpClient;
use crate::TcpClient;

pub struct App {
    tcp_client: TcpClient,
    udp_client: UdpClient,
    commands: Vec<String>,
    last_result: Vec<String>,
    homes: Vec<String>,
    rooms: Vec<String>,
    devices: Vec<String>,
}

impl App {
    pub fn new(tcp_client: TcpClient, udp_client: UdpClient) -> App {
        App {
            tcp_client,
            udp_client,
            commands: vec![],
            last_result: vec![],
            homes: vec![],
            rooms: vec![],
            devices: vec![],
        }
    }

    pub fn on_tick(&mut self) {
        // let value = self.data.pop().unwrap();
        // self.data.insert(0, value);
    }
}
