use std::io::Write;
use std::path::PathBuf;

use anyhow::Result;

use crate::cli::*;
use crate::entities::manager::*;

pub struct CommandHandler<'a> {
    output: &'a mut dyn Write,
    smart_home_manager: SmartHomeManager,
}

impl<'a> CommandHandler<'a> {
    pub fn new(output: &'a mut dyn Write, path: PathBuf) -> Self {
        let smart_home_manager = SmartHomeManager::new(path);

        Self {
            output,
            smart_home_manager,
        }
    }

    pub fn process(&mut self, command: Command) {
        match command {
            Command::Init => self.initialize_smart_home(),
            Command::Status(wrapper) => self.status_command(wrapper.command),
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

    fn initialize_smart_home(&mut self) {
        self.write_response("Initializing a new repo").unwrap();

        let current_dir = env::current_dir().expect("Unable determine the current dir");
        let path = current_dir.as_path();

        if !path.exists() {
            let smart_home_manager = SmartHomeManager::new(path.to_path_buf());
            match smart_home_manager.initialize_smart_home() {
                Ok(_) => self.smart_home_manager = smart_home_manager,
                Err(msg) => self
                    .write_response(&format!("Unable initialize repository {}", msg))
                    .unwrap(),
            };
        } else {
            self.write_response("Repository already exists").unwrap();
        }
    }

    fn print_home_status(&mut self, id: String) {
        match self.smart_home_manager.read_smart_home_status() {
            Ok(state) => match state {
                Some(mut homes) => {
                    homes.retain(|h| h.id == id);

                    if homes.is_empty() {
                        self.write_response("Not found").unwrap()
                    } else {
                        for home in homes {
                            self.write_response(&format!("{}", home)).unwrap();
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
                self.write_response(&format!("{}", msg)).unwrap();
            }
        }
    }

    fn print_room_status(&mut self, id: String) {
        match self.smart_home_manager.read_smart_home_status() {
            Ok(state) => {
                if let Some(homes) = state {
                    let mut found = false;
                    for home in homes {
                        let mut rooms = home.rooms;
                        rooms.retain(|r| r.id == id);

                        for room in rooms {
                            found = true;
                            self.write_response(&format!("{}", room)).unwrap();
                        }
                    }

                    if !found {
                        self.write_response("Not found").unwrap()
                    }
                }
            }
            Err(msg) => {
                self.write_response(&format!("{}", msg)).unwrap();
            }
        }
    }

    fn change_device_status(&mut self, device_id: &str, status: bool) {
        match self
            .smart_home_manager
            .change_device_status(device_id, status)
        {
            Ok(_) => {
                self.write_response(device_id).unwrap();
            }
            Err(msg) => {
                self.write_response(&format!("{}", msg)).unwrap();
            }
        }
    }

    fn handle_device_status_command(&mut self, command: DeviceStatusCommand) {
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
            (_, _) => match self.smart_home_manager.find_device_by_id(device_id) {
                None => self.write_response("Not found").unwrap(),
                Some(device) => {
                    self.write_response(&format!("{}", device)).unwrap();
                }
            },
        }
    }

    fn status_command(&mut self, command: StatusCommand) {
        match command {
            StatusCommand::Home(id) => self.print_home_status(id.id),
            StatusCommand::Room(id) => self.print_room_status(id.id),
            StatusCommand::Device(command) => self.handle_device_status_command(command),
        }
    }

    fn handle_create_home_command(&mut self, create_home: CreateHome) {
        match self
            .smart_home_manager
            .create_home(create_home.name, create_home.description)
        {
            Ok(home_id) => self.write_response(&home_id).unwrap(),
            Err(msg) => self.write_response(&format!("{}", msg)).unwrap(),
        }
    }

    fn handle_create_room_command(&mut self, room: CreateRoom) {
        match self
            .smart_home_manager
            .create_room(room.home_id, room.name, room.description)
        {
            Ok(room_id) => self.write_response(&room_id).unwrap(),
            Err(msg) => self.write_response(&format!("{}", msg)).unwrap(),
        }
    }

    fn handle_create_device_command(&mut self, device: CreateDevice) {
        match self.smart_home_manager.create_device(
            device.r#type,
            device.room_id,
            device.name,
            device.description,
        ) {
            Ok(device_id) => self.write_response(&device_id).unwrap(),
            Err(msg) => self.write_response(&format!("{}", msg)).unwrap(),
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
        match self.smart_home_manager.remove_home(&id.to_string()) {
            Ok(id) => self.write_response(&id).unwrap(),
            Err(msg) => self.write_response(&format!("{}", msg)).unwrap(),
        }
    }

    fn remove_room_by_id(&mut self, id: &str) {
        match self.smart_home_manager.remove_room(&id.to_string()) {
            Ok(id) => self.write_response(&id).unwrap(),
            Err(msg) => self.write_response(&format!("{}", msg)).unwrap(),
        }
    }

    fn remove_device_by_id(&mut self, id: &str) {
        match self.smart_home_manager.remove_device(&id.to_string()) {
            Ok(id) => self.write_response(&id).unwrap(),
            Err(msg) => self.write_response(&format!("{}", msg)).unwrap(),
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
        match self.smart_home_manager.make_measure(&device_id.to_string()) {
            Ok(ms_result) => self.write_response(&ms_result).unwrap(),
            Err(msg) => self.write_response(&format!("{}", msg)).unwrap(),
        }
    }

    fn print_device_ids(&mut self) {
        match self.smart_home_manager.list_all_devices() {
            Ok(devices) => {
                for device in devices {
                    self.write_response(&format!("{}\n", device.id())).unwrap();
                }
            }
            Err(msg) => self.write_response(&format!("{}", msg)).unwrap(),
        }
    }

    fn print_room_ids(&mut self) {
        match self.smart_home_manager.list_all_rooms() {
            Ok(rooms) => {
                for room in rooms {
                    self.write_response(&format!("{}\n", room.id)).unwrap();
                }
            }
            Err(msg) => self.write_response(&format!("{}", msg)).unwrap(),
        }
    }

    fn print_home_ids(&mut self) {
        println!("AAAAA");
        match self.smart_home_manager.list_all_homes() {
            Ok(homes) => {
                for home in homes {
                    self.write_response(&format!("{}\n", home.id)).unwrap();
                }
            }
            Err(msg) => self.write_response(&format!("{}", msg)).unwrap(),
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
