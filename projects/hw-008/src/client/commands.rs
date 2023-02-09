type Command = String;

#[derive(Debug)]
pub enum ClientCommand {
    GetAllHomes,
    GetAllRooms,
    GetAllDevices,
    GetHomeInfo,
    GetRoomInfo,
    GetDeviceInfo,
    ExecuteCommand(Command),
}