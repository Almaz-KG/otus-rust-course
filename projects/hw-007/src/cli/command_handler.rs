use crate::cli::*;
use crate::entities::devices::*;
use crate::entities::house::*;
use crate::entities::Measure;
use anyhow::{anyhow, Result};
use std::fs;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;

const REPO_DIR: &str = ".smart-home";
const SMART_HOME_FILE: &str = "smart-home.json";

type SavedSmartHome = Option<Vec<Home>>;

pub struct CommandHandler<'a> {
    output: &'a mut dyn Write,
}

impl<'a> CommandHandler<'a> {
    pub fn new(output: &'a mut dyn Write) -> Self {
        Self { output }
    }

    pub fn process(&mut self, command: Command) {
        match command {
            Command::Init => self.init(),
            Command::Status(wrapper) => self.status(wrapper.command),
            Command::New(wrapper) => self.handle_new_command(wrapper.command),
            Command::Remove(wrapper) => self.handle_remove_command(wrapper.command),
            Command::Measure(wrapper) => self.handle_measure_command(&wrapper.device_id),
            Command::List(entity) => self.handle_list_command(entity.command),
        }
    }

    fn write_response(&mut self, content: &str) -> Result<(), String> {
        let bytes = content.as_bytes();
        self.output
            .write_all(bytes)
            .map_err(|_| "Unable to write response")?;
        Ok(())
    }

    fn get_repo_dir(&self) -> Result<PathBuf> {
        let mut current_dir = std::env::current_dir()?;
        current_dir.push(REPO_DIR);
        Ok(current_dir)
    }

    fn get_state_file(&self) -> Result<PathBuf> {
        let mut current_dir = std::env::current_dir()?;

        current_dir.push(REPO_DIR);
        current_dir.push(SMART_HOME_FILE);
        Ok(current_dir)
    }

    fn find_home_by_id(&self, id: &str) -> Option<Home> {
        match self.read_smart_home_status() {
            Ok(smart_home) => smart_home.and_then(|homes| homes.into_iter().find(|h| h.id == id)),
            Err(_) => None,
        }
    }

    fn find_home_by_room_id(&self, room_id: &str) -> Option<Home> {
        match self.read_smart_home_status() {
            Ok(smart_home) => smart_home.and_then(|homes| {
                homes
                    .into_iter()
                    .find(|home| home.rooms.iter().any(|r| r.id == room_id))
            }),
            Err(_) => None,
        }
    }

    fn find_room_by_id(&self, id: &str) -> Option<Room> {
        match self.read_smart_home_status() {
            Ok(smart_home) => smart_home.and_then(|homes| {
                for home in homes {
                    let option = home.rooms.into_iter().find(|room| room.id == id);

                    if option.is_some() {
                        return option;
                    }
                }
                None
            }),
            Err(_) => None,
        }
    }

    fn find_device_by_id(&self, id: &str) -> Option<Device> {
        match self.read_smart_home_status() {
            Ok(smart_home) => smart_home.and_then(|homes| {
                let mut devices: Vec<Device> = vec![];
                for home in homes.into_iter() {
                    for mut room in home.rooms.into_iter() {
                        devices.append(&mut room.devices)
                    }
                }

                devices.into_iter().find(|d| d.id() == id)
            }),
            Err(_) => None,
        }
    }

    fn find_room_by_device_id(&self, device_id: &str) -> Option<Room> {
        match self.read_smart_home_status() {
            Ok(smart_home) => smart_home.and_then(|homes| {
                for home in homes {
                    for room in home.rooms.into_iter() {
                        let option = room.devices.iter().find(|d| d.id() == device_id);

                        if option.is_some() {
                            return Some(room);
                        }
                    }
                }
                None
            }),
            Err(_) => None,
        }
    }

    fn is_smart_home_repo_exists(&self) -> bool {
        match self.get_repo_dir() {
            Ok(path) => path.exists(),
            _ => false,
        }
    }

    fn read_smart_home_status(&self) -> Result<SavedSmartHome> {
        let mut current_dir = std::env::current_dir().expect("Unable determine the current dir");

        current_dir.push(REPO_DIR);
        current_dir.push(SMART_HOME_FILE);
        let path = current_dir.as_path();
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let state: SavedSmartHome = serde_json::from_reader(reader)
            .map_err(|e| e.to_string())
            .expect("Unable deserialize the smart-home state");

        Ok(state)
    }

    fn update_state(&self, home: SavedSmartHome) -> Result<()> {
        if self.is_smart_home_repo_exists() {
            let file = self.get_state_file()?;
            let content = serde_json::to_string(&home).unwrap();
            fs::write(file, content).expect("Unable write initial data");
            Ok(())
        } else {
            Err(anyhow!(
                "No repository found. Consider to init repository first"
            ))
        }
    }

    fn update_home_state(&self, home: Home) -> Result<()> {
        let status = self.read_smart_home_status()?;

        match status {
            None => {
                let new_state: SavedSmartHome = Some(vec![home]);
                self.update_state(new_state)
            }
            Some(old_home) => {
                let mut homes: Vec<Home> = old_home
                    .iter()
                    .filter(|h| h.id != home.id)
                    .cloned()
                    .collect();

                homes.push(home);
                self.update_state(Some(homes))
            }
        }
    }

    fn init(&mut self) {
        let mut current_dir = std::env::current_dir().expect("Unable determine the current dir");

        current_dir.push(REPO_DIR);
        let path = current_dir.as_path();

        if !path.exists() {
            self.write_response("Initializing a new repo").unwrap();
            fs::create_dir(path).expect("Unable create repository");

            current_dir.push(SMART_HOME_FILE);
            let init: SavedSmartHome = None;
            let content = serde_json::to_string(&init).unwrap();

            fs::write(current_dir, content).expect("Unable write initial data");
        } else {
            self.write_response("Repository already exists").unwrap();
        }
    }

    fn print_home_status(&mut self, id: String) {
        match self.read_smart_home_status() {
            Ok(state) => match state {
                Some(mut homes) => {
                    homes.retain(|h| h.id == id);

                    if homes.is_empty() {
                        self.write_response("Not found").unwrap()
                    } else {
                        for home in homes {
                            self.write_response(&home.to_string()).unwrap();
                        }
                    }
                }
                None => {
                    self.write_response(
                        "Smart home is not initialized. \
                           Please create smart home instance first",
                    )
                    .unwrap();
                }
            },
            Err(msg) => {
                self.write_response(&msg.to_string()).unwrap();
            }
        }
    }

    fn print_room_status(&mut self, id: String) {
        match self.read_smart_home_status() {
            Ok(state) => {
                if let Some(homes) = state {
                    let mut found = false;
                    for home in homes {
                        let mut rooms = home.rooms;
                        rooms.retain(|r| r.id == id);

                        for room in rooms {
                            found = true;
                            self.write_response(&room.to_string()).unwrap();
                        }
                    }

                    if !found {
                        self.write_response("Not found").unwrap()
                    }
                }
            }
            Err(msg) => {
                self.write_response(&msg.to_string()).unwrap();
            }
        }
    }

    fn change_device_status(&mut self, device_id: &str, status: bool) {
        if let Some(mut room) = self.find_room_by_device_id(device_id) {
            // It's safe to unwrap the result, because otherwise `find_room_by_device_id`
            // will return None
            let device = room
                .devices
                .iter_mut()
                .find(|d| d.id() == device_id)
                .unwrap();

            match device {
                Device::Socket(socket) => {
                    socket.status = SocketStatus::from_bool(status);
                }
                Device::Thermometer(_) => {}
            }
            self.write_response(device_id).unwrap();

            if let Some(mut home) = self.find_home_by_room_id(&room.id) {
                let mut new_rooms: Vec<Room> = home
                    .rooms
                    .iter()
                    .filter(|r| r.id != room.id)
                    .cloned()
                    .collect();

                new_rooms.push(room);
                home.rooms = new_rooms;

                if let Err(msg) = self.update_home_state(home) {
                    self.write_response(&format!("Unable to save changes: {msg}"))
                        .unwrap();
                }
            } else {
                self.write_response(&format!(
                    "Unable find associated home for room: {}",
                    room.id
                ))
                .unwrap();
            }
        } else {
            self.write_response(&format!(
                "Unable find associated room for device: {device_id}"
            ))
            .unwrap();
        }
    }

    fn handle_device_status(&mut self, command: DeviceStatusCommand) {
        let device_id = &command.device_id;

        match (command.disable, command.enable) {
            (Some(_), Some(_)) => self.write_response("Wrong command parameters").unwrap(),
            (None, Some(enable)) => {
                if !enable {
                    self.write_response(device_id).unwrap();
                    return;
                }
                self.change_device_status(device_id, enable);
            }
            (Some(disable), None) => {
                if !disable {
                    self.write_response(device_id).unwrap();
                    return;
                }
                self.change_device_status(device_id, !disable);
            }
            (_, _) => match self.read_smart_home_status() {
                Ok(state) => {
                    if let Some(homes) = state {
                        let mut found = false;
                        for home in homes {
                            for room in home.rooms {
                                let mut devices = room.devices;
                                devices.retain(|d| d.id() == device_id);

                                for device in devices {
                                    found = true;
                                    self.write_response(&device.to_string()).unwrap();
                                }
                            }
                        }

                        if !found {
                            self.write_response("Not found").unwrap()
                        }
                    }
                }
                Err(msg) => {
                    self.write_response(&msg.to_string()).unwrap();
                }
            },
        }
    }

    fn status(&mut self, command: StatusCommand) {
        if self.is_smart_home_repo_exists() {
            match command {
                StatusCommand::Home(id) => self.print_home_status(id.id),
                StatusCommand::Room(id) => self.print_room_status(id.id),
                StatusCommand::Device(command) => self.handle_device_status(command),
            }
        } else {
            self.write_response("No repository found. Consider to init repository first")
                .unwrap();
        }
    }

    fn handle_create_home_command(&mut self, create_home: CreateHome) {
        let home = if let Some(ref description) = create_home.description {
            Home::build()
                .with_name(&create_home.name)
                .with_description(description)
                .build()
        } else {
            Home::build().with_name(&create_home.name).build()
        }
        .expect("Unable create a home");

        let id = home.id.clone();
        if let Err(msg) = self.update_home_state(home) {
            self.write_response(&format!("Unable to save changes: {msg}"))
                .unwrap();
        } else {
            self.write_response(&id).unwrap();
        }
    }

    fn handle_create_room_command(&mut self, room: CreateRoom) {
        match self.find_home_by_id(&room.home_id) {
            Some(mut home) => {
                let new_room = if let Some(ref description) = room.description {
                    Room::build()
                        .with_name(&room.name)
                        .with_description(description)
                        .build()
                } else {
                    Room::build().with_name(&room.name).build()
                }
                .expect("Unable create a home");
                let id = new_room.id.clone();
                home.rooms.push(new_room);

                if let Err(msg) = self.update_home_state(home) {
                    self.write_response(&format!("Unable to save changes: {msg}"))
                        .unwrap();
                } else {
                    self.write_response(&id).unwrap();
                }
            }
            _ => {
                self.write_response(&format!("Home with id: {} not found", room.home_id))
                    .unwrap();
            }
        }
    }

    fn handle_create_device_command(&mut self, device: CreateDevice) {
        fn create_socket(device: &CreateDevice) -> Device {
            let socket = match device.description.as_ref() {
                None => Socket::new(&device.name),
                Some(dsc) => Socket::new_with_description(&device.name, dsc),
            };

            Device::Socket(socket)
        }

        fn create_thermometer(device: &CreateDevice) -> Device {
            let thermometer = match device.description.as_ref() {
                None => Thermometer::new(&device.name),
                Some(dsc) => Thermometer::new_with_description(&device.name, dsc),
            };

            Device::Thermometer(thermometer)
        }

        match self.find_room_by_id(&device.room_id) {
            Some(mut room) => {
                let device = match device.r#type {
                    DeviceType::Socket => create_socket(&device),
                    DeviceType::Thermometer => create_thermometer(&device),
                };
                let id = device.id().clone();

                room.devices.push(device);
                let mut home = self
                    .find_home_by_room_id(&room.id)
                    .expect("Unable find home by room_id");

                let mut rooms: Vec<Room> = home
                    .rooms
                    .iter()
                    .filter(|r| r.id != room.id)
                    .cloned()
                    .collect();

                rooms.push(room);
                home.rooms = rooms;

                if let Err(msg) = self.update_home_state(home) {
                    self.write_response(&format!("Unable to save changes: {msg}"))
                        .unwrap();
                } else {
                    self.write_response(&id).unwrap();
                }
            }
            _ => {
                self.write_response(&format!("Room with id: {} not found", &device.room_id))
                    .unwrap();
            }
        }
    }

    fn handle_new_command(&mut self, new: CreateEntity) {
        match new {
            CreateEntity::Home(home) => self.handle_create_home_command(home),
            CreateEntity::Room(room) => self.handle_create_room_command(room),
            CreateEntity::Device(device) => self.handle_create_device_command(device),
        }
    }

    fn remove_home_by_id(&mut self, id: &str) {
        let current_state = self.read_smart_home_status();

        match current_state {
            Ok(state) => {
                if let Some(homes) = state {
                    let new_state: Vec<Home> = homes.into_iter().filter(|h| h.id != id).collect();

                    if let Err(msg) = self.update_state(Some(new_state)) {
                        self.write_response(&format!("Unable to save changes: {msg}"))
                            .unwrap();
                    } else {
                        self.write_response(id).unwrap();
                    }
                }
            }
            Err(msg) => {
                self.write_response(&format!("Unable remove home: {msg}"))
                    .unwrap();
            }
        }
    }

    fn remove_room_by_id(&mut self, id: &str) {
        let current_state = self.read_smart_home_status();

        match current_state {
            Ok(state) => {
                if let Some(homes) = state {
                    if let Some(mut home) = self.find_home_by_room_id(id) {
                        let rooms: Vec<Room> =
                            home.rooms.into_iter().filter(|r| r.id != id).collect();

                        home.rooms = rooms;

                        let mut new_state: Vec<Home> =
                            homes.into_iter().filter(|h| h.id != home.id).collect();

                        new_state.push(home);

                        if let Err(msg) = self.update_state(Some(new_state)) {
                            self.write_response(&format!("Unable to save changes: {msg}"))
                                .unwrap();
                        } else {
                            self.write_response(id).unwrap();
                        }
                    }
                }
            }
            Err(msg) => {
                self.write_response(&format!("Unable remove home: {msg}"))
                    .unwrap();
            }
        }
    }

    fn remove_device_by_id(&mut self, id: &str) {
        if let Some(mut room) = self.find_room_by_device_id(id) {
            let new_devices = room.devices.into_iter().filter(|d| d.id() != id).collect();

            room.devices = new_devices;
            if let Some(mut home) = self.find_home_by_room_id(&room.id) {
                let mut new_rooms: Vec<Room> = home
                    .rooms
                    .iter()
                    .filter(|r| r.id != room.id)
                    .cloned()
                    .collect();

                new_rooms.push(room);
                home.rooms = new_rooms;

                if let Err(msg) = self.update_home_state(home) {
                    self.write_response(&format!("Unable to save changes: {msg}"))
                        .unwrap();
                } else {
                    self.write_response(id).unwrap();
                }
            } else {
                self.write_response(&format!(
                    "Unable find associated home for room: {}",
                    room.id
                ))
                .unwrap();
            }
        } else {
            self.write_response(&format!("Unable find associated room for device: {id}"))
                .unwrap();
        }
    }

    fn handle_remove_command(&mut self, command: RemoveEntityCommand) {
        match command {
            RemoveEntityCommand::Home(home) => self.remove_home_by_id(&home.id),
            RemoveEntityCommand::Room(room) => self.remove_room_by_id(&room.id),
            RemoveEntityCommand::Device(device) => self.remove_device_by_id(&device.id),
        }
    }

    fn handle_measure_command(&mut self, device_id: &str) {
        match self.find_device_by_id(device_id) {
            None => self.write_response("Not found").unwrap(),
            Some(device) => match device {
                Device::Socket(_) => self.write_response("Not supported").unwrap(),
                Device::Thermometer(th) => match th.measure() {
                    Ok(measurement) => match measurement {
                        None => self.write_response("None").unwrap(),
                        Some(v) => self.write_response(&v.to_string()).unwrap(),
                    },
                    Err(msg) => self.write_response(&msg.to_string()).unwrap(),
                },
            },
        }
    }

    fn print_device_ids(&mut self) {
        match self.read_smart_home_status() {
            Ok(state) => {
                let mut device_ids: Vec<String> = vec![];
                if let Some(homes) = state {
                    for home in homes {
                        for room in home.rooms {
                            for device in room.devices {
                                device_ids.push(device.id().clone())
                            }
                        }
                    }
                }
                self.write_response(&device_ids.join("\n")).unwrap();
            }
            Err(msg) => {
                self.write_response(&msg.to_string()).unwrap();
            }
        }
    }

    fn print_room_ids(&mut self) {
        match self.read_smart_home_status() {
            Ok(state) => {
                let mut room_ids: Vec<String> = vec![];
                if let Some(homes) = state {
                    for home in homes {
                        for room in home.rooms {
                            room_ids.push(room.id)
                        }
                    }
                }
                self.write_response(&room_ids.join("\n")).unwrap();
            }
            Err(msg) => {
                self.write_response(&msg.to_string()).unwrap();
            }
        }
    }

    fn print_home_ids(&mut self) {
        match self.read_smart_home_status() {
            Ok(state) => {
                let mut home_ids: Vec<String> = vec![];
                if let Some(homes) = state {
                    for home in homes {
                        home_ids.push(home.id)
                    }
                }
                self.write_response(&home_ids.join("\n")).unwrap();
            }
            Err(msg) => {
                self.write_response(&msg.to_string()).unwrap();
            }
        }
    }

    fn handle_list_command(&mut self, command: ListEntityCommand) {
        match command {
            ListEntityCommand::Homes => self.print_home_ids(),
            ListEntityCommand::Rooms => self.print_room_ids(),
            ListEntityCommand::Devices => self.print_device_ids(),
        }
    }
}
