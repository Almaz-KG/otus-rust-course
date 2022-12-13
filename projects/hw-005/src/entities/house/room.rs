use crate::entities::devices::Device;
use crate::entities::reportable::{ReportError, Reportable};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// A [Room] entity represents a room in the Home. The home might have many rooms. Each room
/// instance represents with the `name`, `description` and the list of the devices located in the
/// room.
pub struct Room {
    pub name: String,
    pub description: Option<String>,
    pub devices: Vec<Device>,
}

impl Room {
    /// Returns an [RoomBuilder] struct instance for easy building of the actual Room instance.
    /// All the logic of building an instance is hidden in the builder struct and it's methods
    pub fn build() -> RoomBuilder {
        RoomBuilder::default()
    }
}

/// A straight forward and simple implementation of the [Display] trait. It writes the detailed
/// information about Room to the provided output. The detailed information contains the room's
/// `name`, and `description`, as well as the full description of the devices located in this
/// room
impl Display for Room {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        let devices_report: Vec<String> = self.devices.iter().map(|d| format!("{}", d)).collect();

        let txt = format!(
            "Room: {}, Description: {}\n\t\t{}",
            self.name,
            self.description
                .clone()
                .unwrap_or_else(|| "[No description]".to_string()),
            devices_report.join("\n\t\t")
        );

        write!(formatter, "{}", txt)
    }
}

/// An implentation of [Reportable] for Room struct provides the short and fast report about
/// current status of the Room instance
impl Reportable for Room {
    fn report(&self) -> Result<String, ReportError> {
        let devices_report: Vec<String> = self
            .devices
            .iter()
            .map(|d| {
                let report = d.report();

                match report {
                    Ok(report) => report,
                    Err(err) => format!("Error occurred: {}", err),
                }
            })
            .collect();

        Ok(format!(
            "Room: {},\n\t\t{}",
            self.name,
            devices_report.join("\n\t\t")
        ))
    }
}

/// A helper struct for implementing builder pattern over the [Room] struct. It encapsulates an
/// internal structure of the Room
#[derive(Default)]
pub struct RoomBuilder {
    name: Option<String>,
    description: Option<String>,
    devices: Option<Vec<Device>>,
}

impl RoomBuilder {
    /// This method expected to be called at the end of the building process. It will unwrap all
    /// provided fields, and populate these values to the final Room instance. It requires an
    /// exclusive ownership for the self instance.
    ///
    /// It will return [Err] in case, when the `name` for the home is not provided. In any other
    /// case, it should return a valid home instance
    ///
    /// ```
    /// use hw_005::entities::house::RoomBuilder;
    /// let builder = RoomBuilder::default();
    /// let room = builder.build();
    ///
    /// assert!(room.is_err());
    /// ```
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

    /// An owned method which will return a copy of the RoomBuilder with name of Room set. It
    /// will clone the provided `name` parameter. In the future implementation it might be
    /// changed for lean and effective implementation
    pub fn with_name(self, name: &str) -> RoomBuilder {
        RoomBuilder {
            name: Some(name.to_string()),
            description: self.description,
            devices: self.devices,
        }
    }

    /// An owned method which will return a copy of the RoomBuilder with description of Room set.
    /// It will clone the provided `description` parameter. In the future implementation it might
    /// be changed for lean and effective implementation
    pub fn with_description(self, description: &str) -> RoomBuilder {
        RoomBuilder {
            name: self.name,
            description: Some(description.to_string()),
            devices: self.devices,
        }
    }

    /// An owned method which will return a copy of the RoomBuilder with the device inserted to
    /// the list of existing Room devices. It will take ownership of the provided `device`
    /// parameter.
    pub fn with_device(self, device: Device) -> RoomBuilder {
        let mut devices = self.devices.unwrap_or_default();
        devices.push(device);

        RoomBuilder {
            name: self.name,
            description: self.description,
            devices: Some(devices),
        }
    }

    /// An owned method which will return a copy of the RoomBuilder with the list of devices
    /// inserted to the Room's devices field. It will take ownership of the provided `devices`
    /// vector parameter. It might be useful to use this method instead of `with_device`
    /// in case when the user wants to deleted some of the already provided device.
    pub fn with_devices(self, devices: Vec<Device>) -> RoomBuilder {
        RoomBuilder {
            name: self.name,
            description: self.description,
            devices: Some(devices),
        }
    }
}
