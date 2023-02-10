use std::fmt::{Display, Formatter, Result};

pub trait Reportable: Display {
    fn report(&self) -> String;
}

#[derive(Debug)]
pub enum SocketStatus {
    Enabled,
    Disabled,
}

impl Display for SocketStatus {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "{}", format!("{self:?}").to_lowercase())
    }
}

#[derive(Debug)]
pub struct Socket {
    name: String,
    description: Option<String>,
    power_consumption: f32,
    status: SocketStatus,
}

impl Socket {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: Some(description.to_string()),
            ..Self::default()
        }
    }

    pub fn get_description(&self) -> Option<String> {
        self.description.as_ref().cloned()
    }

    pub fn enable(&mut self) {
        self.status = SocketStatus::Enabled
    }

    pub fn disable(&mut self) {
        self.status = SocketStatus::Disabled
    }

    pub fn get_current_power_consumption(&self) -> f32 {
        rand::random()
    }
}

impl Default for Socket {
    fn default() -> Self {
        Self {
            name: "Default socket".to_string(),
            description: None,
            power_consumption: 0.0,
            status: SocketStatus::Disabled,
        }
    }
}

impl Display for Socket {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        let txt = format!(
            "Socket: {}, {}, {}, {}",
            self.name,
            self.description.clone().unwrap_or_default(),
            self.power_consumption,
            self.status
        );

        write!(formatter, "{txt}")
    }
}

impl Reportable for Socket {
    fn report(&self) -> String {
        format!("Socket: {}, Status: {}", self.name, self.status)
    }
}

#[derive(Debug)]
pub struct Thermometer {
    name: String,
    description: Option<String>,
}

impl Thermometer {
    pub fn measure(&self) -> f32 {
        rand::random()
    }

    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: Some(description.to_string()),
        }
    }
}

impl Display for Thermometer {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let txt = format!(
            "Thermometer: {}, {}",
            self.name,
            self.description.clone().unwrap_or_default()
        );

        write!(formatter, "{txt}")
    }
}

impl Reportable for Thermometer {
    fn report(&self) -> String {
        format!("Thermometer: {}, Measure: {}", self.name, self.measure())
    }
}
