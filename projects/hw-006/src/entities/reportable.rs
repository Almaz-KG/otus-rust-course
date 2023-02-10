use crate::entities::MeasureError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// A trait for making possible some entity to report.
/// Any entity, which want to provide internal status must
/// implement this trait as well as [Display] trait.
/// Display trait must be implemented in order to be able
/// provide the full description of the entity. Whereas,
/// the [Reportable] trait devoted for the short, fast as small
/// current status reporting
pub trait Reportable: Display {
    /// Main function for reporting. For the first implementation,
    /// it returns [String], but in the future implementation
    /// it may be refactored to return some lean and complex type
    fn report(&self) -> Result<String, ReportError>;
}

/// An enum describing the error happening during the call of the of `report` function of the
/// Reportable trait. Currently, this is mock enum, with super simple and dummy listed enum values.
/// In some production ready libraries, this enums might have super different values list.
#[derive(Debug)]
pub enum ReportError {
    /// Let's pretend reportable objects might go through the internet and make some updates from
    /// it. It might be helpful to tell end user that the error related to the internet
    /// connection issues, rather than just throwing generic error
    NetworkError(String),
    /// An internal error highlights errors happening when the object is trying to make report
    /// content. It might be related to logic of the report building, and not related to the
    /// other involved part of the reporting process
    InternalError(String),
    /// Many reportable objects are devices which can `measure` the surrounding environment.
    /// This error type addressed for those objects
    MeasureError(String),
    /// Here I'm trying to simulate the case when second error happens during handling first one.
    /// The first parameter is the massage from the error handling logic, and the second
    /// parameter is the root cause of the first error.
    NestedError(String, Box<ReportError>),
    /// A common error type for all other errors, which are not covered with above types
    UnknownError(String),
}

impl Display for ReportError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match self {
            ReportError::NetworkError(msg) => formatter.write_str(&format!("NetworkError: {msg}")),
            ReportError::InternalError(msg) => {
                formatter.write_str(&format!("InternalError: {msg}"))
            }
            ReportError::MeasureError(msg) => formatter.write_str(&format!("MeasureError: {msg}")),
            ReportError::NestedError(msg, root_cause) => formatter.write_str(&format!(
                "Another error happened during handing first one. \
                    Handler message: {msg}. The root cause message: {root_cause}"
            )),
            ReportError::UnknownError(msg) => formatter.write_str(&format!("UnknownError: {msg}")),
        }
    }
}

impl Error for ReportError {}

impl From<MeasureError> for ReportError {
    fn from(measure: MeasureError) -> Self {
        match measure {
            MeasureError::WrongDeviceStateError(msg) => {
                ReportError::InternalError(format!("Wrong device state: {msg}"))
            }
            MeasureError::DeviceIsOff => ReportError::InternalError("Device is OFF".to_string()),
            MeasureError::DeviceIsUnreachable => {
                ReportError::NetworkError("Device is unreachable".to_string())
            }
            MeasureError::MeasurementError(msg) => ReportError::MeasureError(msg),
            MeasureError::UnknownError(msg) => ReportError::UnknownError(msg),
        }
    }
}
