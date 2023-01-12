/// An additional sub module devoted the smart socket device. It contains the socket struct, as
/// well as some helper and utility structs.
mod socket;
pub use socket::{Socket, SocketStatus};

/// I'm tired to write stub text here, hopefully in  the production code I will not be so boiled
/// with such kind of dummy documentation writing part. Again, as the name of the module states -
/// it contains [thermometer#Thermometer] struct and the helper struct
mod thermometer;
pub use thermometer::Thermometer;

/// This is a module stores a common enum [Device], which will handle the variety of devices in
/// the project. Current implementation of this enum contains only two elements inside the enum,
/// but in the future it may have more.
mod device;
pub use device::Device;
