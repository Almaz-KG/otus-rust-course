use anyhow::{anyhow, Result};

use crate::cli::DeviceType;
use crate::entities::devices::{Device, DeviceId, Socket, Thermometer};
use crate::entities::house::{Home, HomeId, Room, RoomId};
use crate::entities::manager::smart_home::SmartHomeManager;
use crate::entities::manager::{FindFunctions, UpdateFunctions};

pub trait CreateFunctions {
    fn create_home(&self, name: String, description: Option<String>) -> Result<HomeId>;

    fn create_room(
        &self,
        home_id: String,
        name: String,
        description: Option<RoomId>,
    ) -> Result<String>;

    fn create_device(
        &self,
        device_type: DeviceType,
        room_id: String,
        name: String,
        description: Option<DeviceId>,
    ) -> Result<String>;
}

impl CreateFunctions for SmartHomeManager {
    fn create_home(&self, name: String, description: Option<String>) -> Result<HomeId> {
        let home = if let Some(ref description) = description {
            Home::build()
                .with_name(&name)
                .with_description(description)
                .build()
        } else {
            Home::build().with_name(&name).build()
        }
        .map_err(|msg| anyhow!(msg))?;

        let id = home.id.clone();

        match self.update_home_state(home) {
            Ok(_) => Ok(id),
            Err(msg) => Err(anyhow!("Unable to create home: {}", msg)),
        }
    }

    fn create_room(
        &self,
        home_id: String,
        name: String,
        description: Option<RoomId>,
    ) -> Result<String> {
        match self.find_home_by_id(&home_id) {
            Some(mut home) => {
                let new_room = if let Some(ref description) = description {
                    Room::build()
                        .with_name(&name)
                        .with_description(description)
                        .build()
                } else {
                    Room::build().with_name(&name).build()
                }
                .map_err(|msg| anyhow!(msg))?;

                let id = new_room.id.clone();
                home.rooms.push(new_room);
                self.update_home_state(home)?;
                Ok(id)
            }
            _ => Err(anyhow!(format!("Home with id: {} not found", home_id))),
        }
    }

    fn create_device(
        &self,
        device_type: DeviceType,
        room_id: String,
        name: String,
        description: Option<DeviceId>,
    ) -> Result<String> {
        fn create_socket(name: &str, description: &Option<String>) -> Device {
            let socket = match description.as_ref() {
                None => Socket::new(name),
                Some(dsc) => Socket::new_with_description(name, dsc),
            };

            Device::Socket(socket)
        }

        fn create_thermometer(name: &str, description: &Option<String>) -> Device {
            let thermometer = match description.as_ref() {
                None => Thermometer::new(name),
                Some(dsc) => Thermometer::new_with_description(name, dsc),
            };

            Device::Thermometer(thermometer)
        }

        match self.find_room_by_id(&room_id) {
            Some(mut room) => {
                let device = match device_type {
                    DeviceType::Socket => create_socket(&name, &description),
                    DeviceType::Thermometer => create_thermometer(&name, &description),
                };
                let id = device.id().clone();
                room.devices.push(device);

                match self.find_home_by_room_id(&room.id) {
                    None => Err(anyhow!(
                        "Unable to find associated home to {} room",
                        room_id
                    )),
                    Some(mut home) => {
                        let mut rooms: Vec<Room> = home
                            .rooms
                            .iter()
                            .filter(|r| r.id != room.id)
                            .cloned()
                            .collect();

                        rooms.push(room);
                        home.rooms = rooms;

                        match self.update_home_state(home) {
                            Ok(_) => Ok(id),
                            Err(msg) => Err(anyhow!("Unable to save changes: {}", msg)),
                        }
                    }
                }
            }
            None => Err(anyhow!(format!("Room with id: {} not found", room_id))),
        }
    }
}
