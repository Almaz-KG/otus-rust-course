use crate::entities::devices::{Device, DeviceId};
use crate::entities::house::{Home, HomeId, Room, RoomId};
use crate::entities::manager::smart_home::SmartHomeManager;

pub trait FindFunctions {
    fn find_home_by_id(&self, id: &HomeId) -> Option<Home>;
    fn find_home_by_room_id(&self, id: &RoomId) -> Option<Home>;
    fn find_room_by_id(&self, id: &RoomId) -> Option<Room>;
    fn find_device_by_id(&self, id: &DeviceId) -> Option<Device>;
    fn find_room_by_device_id(&self, id: &DeviceId) -> Option<Room>;
}

impl FindFunctions for SmartHomeManager {
    fn find_home_by_id(&self, id: &HomeId) -> Option<Home> {
        match self.read_smart_home_status() {
            Ok(smart_home) => smart_home.and_then(|homes| homes.into_iter().find(|h| h.id == *id)),
            Err(_) => None,
        }
    }

    fn find_home_by_room_id(&self, id: &RoomId) -> Option<Home> {
        match self.read_smart_home_status() {
            Ok(smart_home) => smart_home.and_then(|homes| {
                homes
                    .into_iter()
                    .find(|home| home.rooms.iter().any(|r| r.id == *id))
            }),
            Err(_) => None,
        }
    }

    fn find_room_by_id(&self, id: &RoomId) -> Option<Room> {
        match self.read_smart_home_status() {
            Ok(smart_home) => smart_home.and_then(|homes| {
                for home in homes {
                    let option = home.rooms.into_iter().find(|room| room.id == *id);

                    if option.is_some() {
                        return option;
                    }
                }
                None
            }),
            Err(_) => None,
        }
    }

    fn find_device_by_id(&self, id: &DeviceId) -> Option<Device> {
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

    fn find_room_by_device_id(&self, id: &DeviceId) -> Option<Room> {
        match self.read_smart_home_status() {
            Ok(smart_home) => smart_home.and_then(|homes| {
                for home in homes {
                    for room in home.rooms.into_iter() {
                        let option = room.devices.iter().find(|d| d.id() == id);

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
}
