use crate::entities::devices::Device;
use crate::entities::house::room::Room;
use crate::entities::reportable::Reportable;
use crate::entities::{generate_id, ReportError};
use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// A [Home] struct represents the Smart House wrapper. Each home must contain a `name`, an
/// optional `description`, and a list of the [Room]s. All these nested fields of the struct
/// might be used for the reporting purpose. As for now, all these fields are used for full
/// reporting, and the only `name` field is used for short reporting.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Home {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub rooms: Vec<Room>,
}

/// An implementation for [Home] struct
impl Home {
    /// A builder function for easy build of the Home instance. It returns a [HomeBuilder] struct
    /// which wraps all internal logic for building an instance. It implements commonly used in
    /// Rust `builder` pattern
    pub fn build() -> HomeBuilder {
        HomeBuilder::default()
    }

    /// A public function which returns a shared reference for the internal list of Rooms. In
    /// theory, it might return an `Iterator<Item=Room>`, but for time being, I've decided to
    /// return a vector without any redundant complexity
    pub fn get_rooms(&self) -> &Vec<Room> {
        &self.rooms
    }

    /// An unobvious function, which returns an option of Vector of [Reportable]s. It tries to
    /// find devices for the `room` in the belonging list of [Room]s in the Home. It will return
    /// [None] in case, there is no room in the Home or the list of rooms doesn't contain the
    /// room with the provided room name. The signature of this function looks ugly, but hopefully
    /// it will be refactored soon
    pub fn get_devices(&self, room: &Room) -> Option<&Vec<Device>> {
        self.rooms
            .iter()
            .find(|r| r.name == room.name)
            .map(|r| &r.devices)
    }
}

/// An obvious and simple implementation of the [Display] trait for [Home] struct. It writes the
/// full description of the instance to the formatter. The full description consists of Home `name`,
/// `description` (or empty string if it is None) and the full description of all children entities.
impl Display for Home {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        let rooms: Vec<String> = self.rooms.iter().map(|r| r.id.clone()).collect();

        let txt = format!(
            "House: {},\nId: {},\nDescription: {},\nRooms: [{}]",
            self.name,
            self.id,
            self.description
                .clone()
                .unwrap_or_else(|| "[No description]".to_string()),
            rooms.join(",")
        );
        write!(formatter, "{txt}")
    }
}

/// Reporting for the Home struct means printing out the short info about the Home as well as
/// short info about nested fields
impl Reportable for Home {
    fn report(&self) -> Result<String, ReportError> {
        let rooms_report: Vec<String> = self
            .rooms
            .iter()
            .map(|d| {
                let report = d.report();

                match report {
                    Ok(report) => report,
                    Err(err) => format!("Error occurred: {err}"),
                }
            })
            .collect();

        Ok(format!(
            "House: {},\n\t {}",
            self.name,
            rooms_report.join("\n\t")
        ))
    }
}

/// A builder struct for storing the intermediate state of building process of instance of [Home]
/// struct. All the fields from Home moved to this struct as an [Option]al, so in the final call of
/// function `build` each field will be interspersed to the instance of Home struct
#[derive(Default)]
pub struct HomeBuilder {
    name: Option<String>,
    description: Option<String>,
    rooms: Option<Vec<Room>>,
}

impl HomeBuilder {
    /// This method expected to be called at the end of the building process. It will unwrap all
    /// provided fields, and populate these values to the final Home instance. It requires an
    /// exclusive ownership for the self instance.
    ///
    /// It will return [Err] in case, when the `name` for the home is not provided. In any other
    /// case, it should return a valid home instance
    ///
    /// ```
    /// use hw_007::entities::house::HomeBuilder;
    /// let builder = HomeBuilder::default();
    /// let home = builder.build();
    ///
    /// assert!(home.is_err());
    /// ```
    pub fn build(self) -> Result<Home, String> {
        if self.name.is_none() {
            return Err("Please, provide house name".to_string());
        }

        let h = Home {
            id: generate_id("home"),
            name: self.name.unwrap(),
            description: self.description,
            rooms: self.rooms.unwrap_or_default(),
        };

        Ok(h)
    }

    /// An owned method which will return a copy of the HomeBuilder with name of Home set. It
    /// will clone the provided `name` parameter. In the future implementation it might be
    /// changed for lean and effective implementation
    pub fn with_name(self, name: &str) -> HomeBuilder {
        HomeBuilder {
            name: Some(name.to_string()),
            description: self.description,
            rooms: self.rooms,
        }
    }

    /// An owned method which will return a copy of the HomeBuilder with description of Home set.
    /// It will clone the provided `description` parameter. In the future implementation it might
    /// be changed for lean and effective implementation
    pub fn with_description(self, description: &str) -> HomeBuilder {
        HomeBuilder {
            name: self.name,
            description: Some(description.to_string()),
            rooms: self.rooms,
        }
    }

    /// An owned method which will return a copy of the [HomeBuilder] with the room inserted to the
    /// list of existing rooms. It will take ownership of the provided `room` parameter.
    pub fn with_room(self, room: Room) -> HomeBuilder {
        let mut rooms = self.rooms.unwrap_or_default();
        rooms.push(room);

        HomeBuilder {
            name: self.name,
            description: self.description,
            rooms: Some(rooms),
        }
    }

    /// An owned method which will return a copy of the HomeBuilder with the list of rooms
    /// inserted to the Home rooms field. It will take ownership of the provided `room` vector
    /// parameter. It might be usefull to use this method instead of `with_room` in case when the
    /// user wants to deleted some of the provided room.
    pub fn with_rooms(self, rooms: Vec<Room>) -> HomeBuilder {
        HomeBuilder {
            name: self.name,
            description: self.description,
            rooms: Some(rooms),
        }
    }
}
