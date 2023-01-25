use crate::entities::devices::DeviceId;
use anyhow::{anyhow, Result};

use crate::entities::house::{Home, HomeId, RoomId};
use crate::entities::manager::smart_home::SmartHomeManager;
use crate::entities::manager::{FindFunctions, UpdateFunctions};

pub trait RemoveFunctions {
    fn remove_home(&self, id: &HomeId) -> Result<HomeId>;

    fn remove_room(&self, id: &RoomId) -> Result<RoomId>;

    fn remove_device(&self, id: &DeviceId) -> Result<DeviceId>;
}

impl RemoveFunctions for SmartHomeManager {
    fn remove_home(&self, id: &HomeId) -> Result<HomeId> {
        let current_state = self.read_smart_home_status();

        match current_state {
            Ok(state) => {
                if let Some(homes) = state {
                    let new_state: Vec<Home> = homes.into_iter().filter(|h| h.id != *id).collect();
                    match self.update_state(Some(new_state)) {
                        Ok(_) => Ok(id.clone()),
                        Err(msg) => Err(anyhow!(format!("Unable to save changes: {}", msg))),
                    }
                } else {
                    Err(anyhow!(format!("Home {} not found", id)))
                }
            }
            Err(msg) => Err(anyhow!(format!("Unable remove home: {}", msg))),
        }
    }

    fn remove_room(&self, id: &RoomId) -> Result<RoomId> {
        match self.find_home_by_room_id(id) {
            None => Err(anyhow!(format!("Home not found for room {}", id))),
            Some(mut home) => {
                home.rooms.retain(|r| r.id != *id);

                match self.update_home_state(home) {
                    Ok(_) => Ok(id.clone()),
                    Err(msg) => Err(anyhow!(format!("Unable to save changes: {}", msg))),
                }
            }
        }
    }

    fn remove_device(&self, id: &DeviceId) -> Result<DeviceId> {
        if let Some(mut room) = self.find_room_by_device_id(id) {
            room.devices.retain(|d| d.id() != id);

            if let Some(mut home) = self.find_home_by_room_id(&room.id) {
                home.rooms.retain(|r| r.id != room.id);
                home.rooms.push(room);

                match self.update_home_state(home) {
                    Ok(_) => Ok(id.clone()),
                    Err(msg) => Err(anyhow!(format!("Unable to save changes: {}", msg))),
                }
            } else {
                Err(anyhow!(format!(
                    "Unable find associated home for room: {}",
                    room.id
                )))
            }
        } else {
            Err(anyhow!(format!(
                "Unable find associated room for device: {}",
                id
            )))
        }
    }
}
