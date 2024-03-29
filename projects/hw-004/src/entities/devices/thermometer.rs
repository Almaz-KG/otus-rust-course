use crate::entities::reportable::Reportable;
use std::fmt::{Display, Formatter, Result};

/// A super short and stupid wrapper around the Thermometer entity.
/// In theory and the future implementations, it should become more
/// robust and meaningful entity with variety internal states and
/// functionality. As for now, it's just a dummy thermometer
#[derive(Debug)]
pub struct Thermometer {
    name: String,
    description: Option<String>,
}

/// A thermometer struct implementation, it mostly wrapper and dummy stub
/// logic inside each method.
impl Thermometer {
    /// A fake implementation of the `measure` function for the
    /// given thermometer instance. The current implementation gives
    /// a random number from [0.. 1.0). I don't expect that the logic
    /// behind this method will be changed in scope of this project
    /// at all. So, please consider to use it if you want to get some
    /// random number in your use case
    pub fn measure(&self) -> f32 {
        rand::random()
    }

    /// Creates a new instance of [Thermometer] by given name and description.
    /// It's a fake struct, because in the real life we don't need the name
    /// of our thermometer.
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: Some(description.to_string()),
        }
    }
}

/// A super straight forward implemntation of the [Display] trait
/// for Thermometer struct. It writes out the name and the description
/// of the instance with the `Thermometer` prefix.
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

/// A Reportable implementation for Thermometer struct fives back for
/// caller the String representation of the current status. It makes
/// measurement for building the status report.
impl Reportable for Thermometer {
    fn report(&self) -> String {
        format!("Thermometer: {}, Measure: {}", self.name, self.measure())
    }
}
