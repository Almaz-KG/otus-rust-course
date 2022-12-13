use super::socket::Socket;
use super::thermometer::Thermometer;
use crate::entities::{ReportError, Reportable};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// This is a common Trait representing variety of devices in Smart Home project. Currently it
/// has only two elements, but it the future implementation it may have more.
///
/// There are two approaches how to handle problem of storing dynamically typed elements in
/// one typed container. There is no any java-like `Object` classes, and runtime at all.
///
/// The first approach is to use `dyn MyTrait` type as element type of container. It means,
/// developer will be forced to implement `MyTrait` trait for each device. And the main drawback
/// of this approach is the type erasure. Users of the `dyn MyTrait` will not have an access for
/// the fields and methods for the type behind the `dyn MyTrait` type. Instead, all internal
/// features will be hidden by Trait public API, which will lead to down-cast the trait.
///
/// Another approach, and this is the implementation of this approach, is to have enum for
/// each [Device] in the project. It's much easier to deal with enum, which incapsulates each
/// device types within the element of Enum. A negative side of this approach is that this enum
/// does not support easy inheritance. If user want to introduce a new device type to the
/// project, he/she will forced to update this enum, as well as update all places where this
/// enum was used, it's not cool at all. A classical `composition` pattern might be considered as
/// one of possible solution for the `extension` limitation
#[derive(Debug)]
pub enum Device {
    Socket(Socket),
    Thermometer(Thermometer),
}

impl Display for Device {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match self {
            Device::Socket(s) => formatter.write_str(&format!("{s}")),

            Device::Thermometer(t) => formatter.write_str(&format!("{t}")),
        }
    }
}

impl Reportable for Device {
    fn report(&self) -> Result<String, ReportError> {
        match self {
            Device::Socket(s) => s.report(),
            Device::Thermometer(t) => t.report(),
        }
    }
}
