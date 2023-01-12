use crate::entities::reportable::{ReportError, Reportable};
use crate::entities::{generate_id, Measure, MeasureError};
use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// A super short and stupid wrapper around the Thermometer entity. In theory and the future
/// implementations, it should become more robust and meaningful entity with variety internal
/// states and functionality. As for now, it's just a dummy thermometer
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Thermometer {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

/// A thermometer struct implementation, it mostly wrapper and dummy stub-logic inside each method.
impl Thermometer {
    /// Creates a new instance of [Thermometer] by given name. It's a fake struct, because in the
    /// real life we don't need the name of our thermometer.
    pub fn new(name: &str) -> Self {
        Self {
            id: generate_id("ther_"),
            name: name.to_string(),
            description: None,
        }
    }

    /// Creates a new instance of [Thermometer] by given name and description. It's a fake
    /// struct, because in the real life we don't need the name of our thermometer.
    pub fn new_with_description(name: &str, description: &str) -> Self {
        Self {
            id: generate_id("ther_"),
            name: name.to_string(),
            description: Some(description.to_string()),
        }
    }
}

/// A super straight forward implemntation of the [Display] trait for Thermometer struct. It
/// writes out the name and the description of the instance with the `Thermometer` prefix.
impl Display for Thermometer {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        let txt = format!(
            "Thermometer: {},\nId: {},\nDescription: {}",
            self.name,
            self.id,
            self.description
                .clone()
                .unwrap_or_else(|| "[No description]".to_string())
        );

        write!(formatter, "{}", txt)
    }
}

impl Measure<f32> for Thermometer {
    /// A fake implementation of the `measure` function for the given thermometer instance. The
    /// current implementation gives a random number from [0.. 1.0). I don't expect that the logic
    /// behind this method will be changed in scope of this project at all. So, please consider
    /// to use it if you want to get some random number in your use case
    fn measure(&self) -> Result<Option<f32>, MeasureError> {
        Ok(Some(rand::random()))
    }
}

/// A Reportable implementation for Thermometer struct fives back for caller the String
/// representation of the current status. It makes measurement for building the status report.
impl Reportable for Thermometer {
    fn report(&self) -> Result<String, ReportError> {
        match self.measure() {
            Ok(result) => match result {
                Some(value) => Ok(format!("Thermometer: {}, Measure: {}", self.name, value)),
                None => Ok(format!("Thermometer: {}, No measure value", self.name)),
            },
            Err(msg) => Err(msg.into()),
        }
    }
}
