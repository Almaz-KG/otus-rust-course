use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// An interface for any object, which want to declare that it can measure the surrounding
/// environment. Current implementation is relativelly simple and dummy, but in the future
/// implementations it might be changed.
pub trait CanMeasure<T> {
    /// Make a measure. It's simple function for making measure of the env. It's a blocking
    /// version, in the future updates it might be changed to the `async` version.
    fn measure(&self) -> Result<Option<T>, MeasureError>;
}

/// A list of errors, which may happen during the measurement. It's again super simple and dummy
/// implementation. Probably, it will not cover all possible scenarios.
#[derive(Debug)]
pub enum MeasureError {
    /// This error type describes a wrong internal state of the measurement device. One of the
    /// possible wrong state is the `Device Not Ready`.
    WrongDeviceStateError(String),
    /// Error for the case when the device is turned off, and it can't make any measurements.
    DeviceIsOff,
    /// Option for the devices located in the completely different place, and the communication
    /// is happening through the Internet. In case, then the communication is broken this error
    /// type should be returned
    DeviceIsUnreachable,
    /// Another internal error type indicating error happening in the process of making
    /// measurement. It's more comprehensive error type
    MeasurementError(String),

    /// Error type for all other possible scenarios. This type should be returned in cases where
    /// other above listed types are not align with the actual situation
    UnknownError(String),
}

impl Error for MeasureError {}

impl Display for MeasureError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match self {
            MeasureError::WrongDeviceStateError(msg) => {
                formatter.write_str(&format!("WrongDeviceStateError: {}", msg))
            }
            MeasureError::DeviceIsOff => formatter.write_str("DeviceIsOff"),
            MeasureError::DeviceIsUnreachable => formatter.write_str("DeviceIsUnreachable"),
            MeasureError::MeasurementError(msg) => {
                formatter.write_str(&format!("MeasurementError: {}", msg))
            }
            MeasureError::UnknownError(msg) => {
                formatter.write_str(&format!("UnknownError: {}", msg))
            }
        }
    }
}
