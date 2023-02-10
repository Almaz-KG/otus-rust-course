use std::borrow::BorrowMut;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

use crate::clients::UdpClient;
use crate::commands::ClientCommand;
use crate::TcpClient;
use tui::widgets::TableState;

#[derive(Clone, Copy)]
pub enum SelectedTable {
    Homes,
    Rooms,
    Devices,
}

impl SelectedTable {
    pub fn next(&self) -> Self {
        match self {
            SelectedTable::Homes => SelectedTable::Rooms,
            SelectedTable::Rooms => SelectedTable::Devices,
            SelectedTable::Devices => SelectedTable::Homes,
        }
    }
}

pub struct ApplicationState {
    tcp_client: TcpClient,
    udp_client: UdpClient,
    pub current_command: String,
    pub executed_commands: Vec<String>,
    pub last_response: Vec<String>,
    pub last_info: Vec<String>,
    pub homes: Vec<String>,
    pub rooms: Vec<String>,
    pub devices: Vec<String>,
    pub current_selected_table: SelectedTable,
    pub homes_table_select_state: TableState,
    pub rooms_table_select_state: TableState,
    pub devices_table_select_state: TableState,
}

impl ApplicationState {
    pub fn new(tcp_client: TcpClient, udp_client: UdpClient) -> ApplicationState {
        let mut firs_element_selected = TableState::default();
        firs_element_selected.select(Some(0));

        ApplicationState {
            tcp_client,
            udp_client,
            current_command: "".to_string(),
            executed_commands: vec![],
            last_response: vec![],
            last_info: vec![],
            homes: vec![],
            rooms: vec![],
            devices: vec![],
            current_selected_table: SelectedTable::Homes,
            homes_table_select_state: firs_element_selected.clone(),
            rooms_table_select_state: firs_element_selected.clone(),
            devices_table_select_state: firs_element_selected.clone(),
        }
    }

    fn highlight_next_element(select_state: &mut TableState, items_len: usize) {
        let i = match select_state.selected() {
            Some(i) => {
                if i >= items_len - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        select_state.select(Some(i));
    }

    fn highlight_previous_element(select_state: &mut TableState, items_len: usize) {
        let i = match select_state.selected() {
            Some(i) => {
                if i == 0 {
                    items_len - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        select_state.select(Some(i));
    }

    pub fn highlight_next(&mut self) {
        match self.current_selected_table {
            SelectedTable::Homes => {
                ApplicationState::highlight_next_element(
                    self.homes_table_select_state.borrow_mut(),
                    self.homes.len(),
                );
            }
            SelectedTable::Rooms => ApplicationState::highlight_next_element(
                self.rooms_table_select_state.borrow_mut(),
                self.rooms.len(),
            ),
            SelectedTable::Devices => ApplicationState::highlight_next_element(
                self.devices_table_select_state.borrow_mut(),
                self.devices.len(),
            ),
        }
    }

    pub fn highlight_previous(&mut self) {
        match self.current_selected_table {
            SelectedTable::Homes => ApplicationState::highlight_previous_element(
                self.homes_table_select_state.borrow_mut(),
                self.homes.len(),
            ),
            SelectedTable::Rooms => ApplicationState::highlight_previous_element(
                self.rooms_table_select_state.borrow_mut(),
                self.rooms.len(),
            ),
            SelectedTable::Devices => ApplicationState::highlight_previous_element(
                self.devices_table_select_state.borrow_mut(),
                self.devices.len(),
            ),
        }
    }
}

pub struct ApplicationStateUpdater {
    app_state: Arc<Mutex<ApplicationState>>,
    events_receiver: Receiver<ClientCommand>,
}

impl ApplicationStateUpdater {
    pub fn new(
        app_state: Arc<Mutex<ApplicationState>>,
        events_receiver: Receiver<ClientCommand>,
    ) -> Self {
        Self {
            app_state,
            events_receiver,
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
                            let homes = ApplicationStateUpdater::handle_execute_command(
                                "list homes".to_string(),
                                &mut app_state,
                            );
                            app_state.homes = homes;
                        }
                        ClientCommand::GetAllRooms => {
                            let rooms = ApplicationStateUpdater::handle_execute_command(
                                "list rooms".to_string(),
                                &mut app_state,
                            );
                            app_state.rooms = rooms;
                        }
                        ClientCommand::GetAllDevices => {
                            let devices = ApplicationStateUpdater::handle_execute_command(
                                "list devices".to_string(),
                                &mut app_state,
                            );
                            app_state.devices = devices;
                        }

                        ClientCommand::GetHomeInfo => {
                            let index = app_state.homes_table_select_state.selected().unwrap_or(0);

                            let id = app_state.homes.get(index).unwrap();
                            let command = format!("status home -i {}", id);
                            let result = ApplicationStateUpdater::handle_execute_command(
                                command,
                                &mut app_state,
                            );
                            app_state.last_info = result;
                        }

                        ClientCommand::GetRoomInfo => {
                            let index = app_state.rooms_table_select_state.selected().unwrap_or(0);

                            let id = app_state.rooms.get(index).unwrap();
                            let command = format!("status room -i {}", id);
                            let result = ApplicationStateUpdater::handle_execute_command(
                                command,
                                &mut app_state,
                            );
                            app_state.last_info = result;
                        }
                        ClientCommand::GetDeviceInfo => {
                            let index =
                                app_state.devices_table_select_state.selected().unwrap_or(0);

                            let id = app_state.devices.get(index).unwrap();
                            let command = format!("status device -i {}", id);
                            let result = ApplicationStateUpdater::handle_execute_command(
                                command,
                                &mut app_state,
                            );
                            app_state.last_info = result;
                        }
                        ClientCommand::ExecuteCommand(cmd) => {
                            app_state.last_response =
                                ApplicationStateUpdater::handle_execute_command(
                                    cmd,
                                    &mut app_state,
                                );
                        }
                    }
                }
            }
        });
    }

    fn handle_execute_command(
        command: String,
        app_state: &mut MutexGuard<ApplicationState>,
    ) -> Vec<String> {
        let state = app_state;
        let result = state.tcp_client.command(command);

        result
            .map(|msg| {
                msg.split('\n')
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            })
            .map_err(|e| vec![format!("Error: {}", e)])
            .unwrap()
    }
}
