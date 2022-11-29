use crate::device::{Displayable, Reportable};

pub struct Room {
    name: String,
    description: Option<String>,
    devices: Vec<Box<dyn Reportable>>,
}

impl Room {
    pub fn build() -> RoomBuilder {
        RoomBuilder::default()
    }
}

impl Displayable for Room {
    fn display(&self) -> String {
        let devices_report: Vec<String> = self.devices.iter().map(|d| d.display()).collect();

        format!(
            "Room: {}, Description: {}\n\t\t{}",
            self.name,
            self.description.clone().unwrap_or_default(),
            devices_report.join("\n\t\t")
        )
    }
}

impl Reportable for Room {
    fn report(&self) -> String {
        let devices_report: Vec<String> = self.devices.iter().map(|d| d.report()).collect();

        format!(
            "Room: {},\n\t\t{}",
            self.name,
            devices_report.join("\n\t\t")
        )
    }
}

#[derive(Default)]
pub struct RoomBuilder {
    name: Option<String>,
    description: Option<String>,
    devices: Option<Vec<Box<dyn Reportable>>>,
}

impl RoomBuilder {
    pub fn build(self) -> Result<Room, String> {
        if self.name.is_none() {
            return Err("Please, provide room name".to_string());
        }

        let r = Room {
            name: self.name.unwrap(),
            description: self.description,
            devices: self.devices.unwrap_or_default(),
        };

        Ok(r)
    }

    pub fn with_name(self, name: &str) -> RoomBuilder {
        RoomBuilder {
            name: Some(name.to_string()),
            description: self.description,
            devices: self.devices,
        }
    }

    pub fn with_description(self, description: &str) -> RoomBuilder {
        RoomBuilder {
            name: self.name,
            description: Some(description.to_string()),
            devices: self.devices,
        }
    }

    pub fn with_device(self, device: Box<dyn Reportable>) -> RoomBuilder {
        let mut devices = self.devices.unwrap_or_default();
        devices.push(device);

        RoomBuilder {
            name: self.name,
            description: self.description,
            devices: Some(devices),
        }
    }

    pub fn with_devices(self, devices: Vec<Box<dyn Reportable>>) -> RoomBuilder {
        RoomBuilder {
            name: self.name,
            description: self.description,
            devices: Some(devices),
        }
    }
}

pub struct House {
    name: String,
    description: Option<String>,
    rooms: Vec<Room>,
}

impl House {
    pub fn build() -> HouseBuilder {
        HouseBuilder::default()
    }

    pub fn get_rooms(&self) -> &Vec<Room> {
        &self.rooms
    }

    pub fn get_devices(&self, room: &Room) -> Option<&Vec<Box<dyn Reportable>>> {
        self.rooms
            .iter()
            .find(|r| r.name == room.name)
            .map(|r| &r.devices)
    }
}

impl Displayable for House {
    fn display(&self) -> String {
        let rooms_report: Vec<String> = self.rooms.iter().map(|d| d.display()).collect();

        format!(
            "House: {}, Description: {}\n\t {}",
            self.name,
            self.description.clone().unwrap_or_default(),
            rooms_report.join("\n\t")
        )
    }
}

impl Reportable for House {
    fn report(&self) -> String {
        let rooms_report: Vec<String> = self.rooms.iter().map(|d| d.report()).collect();

        format!("House: {},\n\t {}", self.name, rooms_report.join("\n\t"))
    }
}

#[derive(Default)]
pub struct HouseBuilder {
    name: Option<String>,
    description: Option<String>,
    rooms: Option<Vec<Room>>,
}

impl HouseBuilder {
    pub fn build(self) -> Result<House, String> {
        if self.name.is_none() {
            return Err("Please, provide house name".to_string());
        }

        let h = House {
            name: self.name.unwrap(),
            description: self.description,
            rooms: self.rooms.unwrap_or_default(),
        };

        Ok(h)
    }

    pub fn with_name(self, name: &str) -> HouseBuilder {
        HouseBuilder {
            name: Some(name.to_string()),
            description: self.description,
            rooms: self.rooms,
        }
    }

    pub fn with_description(self, description: &str) -> HouseBuilder {
        HouseBuilder {
            name: self.name,
            description: Some(description.to_string()),
            rooms: self.rooms,
        }
    }

    pub fn with_room(self, room: Room) -> HouseBuilder {
        let mut rooms = self.rooms.unwrap_or_default();
        rooms.push(room);

        HouseBuilder {
            name: self.name,
            description: self.description,
            rooms: Some(rooms),
        }
    }

    pub fn with_rooms(self, rooms: Vec<Room>) -> HouseBuilder {
        HouseBuilder {
            name: self.name,
            description: self.description,
            rooms: Some(rooms),
        }
    }
}
