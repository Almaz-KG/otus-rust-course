use std::sync::{Arc, Mutex, MutexGuard};
use std::sync::mpsc::Receiver;
use std::thread;
use clap::builder::Str;
use crate::clients::UdpClient;
use crate::commands::ClientCommand;
use crate::{ServerResponse, TcpClient};

pub struct ApplicationState {
    tcp_client: TcpClient,
    udp_client: UdpClient,
    pub current_command: String,
    pub commands: Vec<String>,
    pub last_result: Vec<String>,
    pub homes: Vec<String>,
    pub rooms: Vec<String>,
    pub devices: Vec<String>,
}

impl ApplicationState {
    pub fn new(tcp_client: TcpClient, udp_client: UdpClient) -> ApplicationState {
        ApplicationState {
            tcp_client,
            udp_client,
            current_command: "".to_string(),
            commands: vec![],
            last_result: vec![],
            homes: vec![],
            rooms: vec![],
            devices: vec![],
        }
    }

    pub fn get_device_info(&self) -> Vec<String>{
        vec!["TEST DATA".to_string()]
    }
}

pub struct ApplicationStateUpdater {
    app_state: Arc<Mutex<ApplicationState>>,
    events_receiver: Receiver<ClientCommand>
}

impl ApplicationStateUpdater {
    pub fn new(app_state: Arc<Mutex<ApplicationState>>, events_receiver: Receiver<ClientCommand>) -> Self {
        Self {
            app_state,
            events_receiver
        }
    }

    pub fn start(self) {
        thread::spawn(move || {
            let mut app_state = self.app_state.lock().unwrap();
            assert!(app_state.tcp_client.connect().is_ok());
            drop(app_state);

            loop {
                if let Ok(command) = self.events_receiver.recv() {
                    let mut app_state = self.app_state.lock().unwrap();

                    match command {
                        ClientCommand::GetAllHomes => {
                            let homes = ApplicationStateUpdater::handle_execute_command
                                ("list homes".to_string(), &mut app_state);
                            app_state.homes = homes;
                        },
                        ClientCommand::GetAllRooms => {
                            let rooms = ApplicationStateUpdater::handle_execute_command
                                ("list rooms".to_string(), &mut app_state);
                            app_state.rooms = rooms;
                        },
                        ClientCommand::GetAllDevices => {
                            let devices = ApplicationStateUpdater::handle_execute_command
                                ("list devices".to_string(), &mut app_state);
                            app_state.devices = devices;
                        },

                        ClientCommand::GetHomeInfo(id) => {
                            ApplicationStateUpdater::get_info("home", id, app_state);
                        }
                        ClientCommand::GetRoomInfo(id) => {
                            ApplicationStateUpdater::get_info("room", id, app_state);
                        }
                        ClientCommand::GetDeviceInfo(id) => {
                            ApplicationStateUpdater::get_info("device", id, app_state);
                        }
                        ClientCommand::ExecuteCommand(cmd) => {
                            app_state.last_result =
                                ApplicationStateUpdater::handle_execute_command(cmd, &mut app_state);
                        }
                    }
                }
            }
        });
    }

    fn handle_execute_command(command: String,
                              app_state: &mut MutexGuard<ApplicationState>) -> Vec<String> {
        let mut state = app_state;
        let result = state.tcp_client.command(command);

        result
            .map(|msg| {msg
                .split("\n")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()})
            .map_err(|e| vec![format!("Error: {}", e)])
            .unwrap()
    }

    fn get_all_info(entity_type: &str, app_state: MutexGuard<ApplicationState>) {
        // let mut state = app_state;
        // let result = state.tcp_client.
        //
        // state.last_result = result
        //     .map(|msg| {msg
        //         .split("\n")
        //         .map(|s| s.to_string())
        //         .collect::<Vec<String>>()})
        //     .map_err(|e| vec![format!("Error: {}", e)])
        //     .unwrap();
    }

    fn get_info(entity_type: &str, entity_id: String, app_state: MutexGuard<ApplicationState>) {
        todo!()
    }

}
