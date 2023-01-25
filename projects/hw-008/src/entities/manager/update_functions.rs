use anyhow::{anyhow, Result};
use std::fs;

use crate::entities::devices::{Device, DeviceStatus, SocketStatus};
use crate::entities::house::Home;
use crate::entities::manager::smart_home::{SavedSmartHome, SmartHomeManager};
use crate::entities::manager::FindFunctions;

pub trait UpdateFunctions {
    fn change_device_status(&mut self, device_id: &str, status: DeviceStatus) -> Result<()>;
    fn update_state(&self, home: SavedSmartHome) -> Result<()>;
    fn update_home_state(&self, home: Home) -> Result<()>;
}

impl UpdateFunctions for SmartHomeManager {
    fn change_device_status(&mut self, device_id: &str, status: DeviceStatus) -> Result<()> {
        if let Some(room) = self.find_room_by_device_id(&device_id.to_string()) {
            // It's safe to unwrap the result, because otherwise `find_room_by_device_id`
            // will return None
            let device = self.find_device_by_id(&device_id.to_string()).unwrap();

            match device {
                Device::Socket(mut socket) => {
                    socket.status = SocketStatus::from_bool(status);
                }
                Device::Thermometer(_) => {}
            }

            if let Some(mut home) = self.find_home_by_room_id(&room.id) {
                home.rooms.retain(|r| r.id != room.id);
                home.rooms.push(room);

                match self.update_home_state(home) {
                    Ok(_) => Ok(()),
                    Err(msg) => Err(anyhow!(format!("{}", msg))),
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
                device_id
            )))
        }
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
}
