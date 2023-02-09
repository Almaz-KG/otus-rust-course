use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::{anyhow, Result};

use crate::entities::devices::{Device, DeviceId};
use crate::entities::house::{Home, Room};
use crate::entities::manager::FindFunctions;
use crate::entities::Measure;

const SMART_HOME_FILE: &str = "smart-home.json";

const REPO_DIR: &str = ".smart-home";

pub(crate) type SavedSmartHome = Option<Vec<Home>>;

pub struct SmartHomeManager {
    path: PathBuf,
}

impl SmartHomeManager {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn initialize_smart_home(&self) -> Result<()> {
        if !self.path.exists() {
            fs::create_dir(self.path.clone()).expect("Unable create repository");

            let mut path = self.path.clone();
            path.push(REPO_DIR);
            path.push(SMART_HOME_FILE);

            let init: SavedSmartHome = None;
            let content = serde_json::to_string(&init).unwrap();

            fs::write(path, content).expect("Unable write initial data");
            Ok(())
        } else {
            Err(anyhow!(format!(
                "Path: {} is not empty",
                self.path.display()
            )))
        }
    }

    pub fn get_state_file(&self) -> Result<PathBuf> {
        let mut current_dir = self.path.clone();
        current_dir.push(REPO_DIR);
        current_dir.push(SMART_HOME_FILE);
        Ok(current_dir)
    }

    pub fn is_smart_home_repo_exists(&self) -> bool {
        self.path.exists()
    }

    pub fn read_smart_home_status(&self) -> Result<SavedSmartHome> {
        let mut current_dir = self.path.clone();
        println!("{:?}", current_dir);

        current_dir.push(REPO_DIR);
        current_dir.push(SMART_HOME_FILE);

        let path = current_dir.as_path();
        println!("{:?}", path);

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let state: SavedSmartHome = serde_json::from_reader(reader)
            .map_err(|e| format!("{}", e))
            .expect("Unable deserialize the smart-home state");

        Ok(state)
    }

    pub fn make_measure(&self, device_id: &DeviceId) -> Result<String> {
        match self.find_device_by_id(device_id) {
            None => Err(anyhow!("Not found")),
            Some(device) => match device {
                Device::Socket(_) => Err(anyhow!("Not supported")),
                Device::Thermometer(th) => match th.measure() {
                    Ok(measurement) => match measurement {
                        None => Err(anyhow!("N/A")),
                        Some(v) => Ok(v.to_string()),
                    },
                    Err(msg) => Err(anyhow!(format!("{}", msg))),
                },
            },
        }
    }

    pub fn list_all_devices(&self) -> Result<Vec<Device>> {
        match self.read_smart_home_status() {
            Ok(state) => {
                let mut devices: Vec<Device> = vec![];
                if let Some(homes) = state {
                    for home in homes {
                        for room in home.rooms {
                            for device in room.devices {
                                devices.push(device.clone())
                            }
                        }
                    }
                }
                Ok(devices)
            }
            Err(msg) => Err(anyhow!(format!("Unable list of devices: {}", msg))),
        }
    }

    pub fn list_all_rooms(&self) -> Result<Vec<Room>> {
        match self.read_smart_home_status() {
            Ok(state) => {
                let mut rooms: Vec<Room> = vec![];
                if let Some(homes) = state {
                    for home in homes {
                        for room in home.rooms {
                            rooms.push(room.clone())
                        }
                    }
                }
                Ok(rooms)
            }
            Err(msg) => Err(anyhow!(format!("Unable list of rooms: {}", msg))),
        }
    }

    pub fn list_all_homes(&self) -> Result<Vec<Home>> {
        match self.read_smart_home_status() {
            Ok(state) => match state {
                None => Ok(vec![]),
                Some(homes) => Ok(homes),
            },
            Err(msg) => Err(anyhow!(format!("Unable list of rooms: {}", msg))),
        }
    }
}
