type EntityId = String;
type Command = String;

#[derive(Debug)]
pub enum ClientCommand {
    GetAllHomes,
    GetAllRooms,
    GetAllDevices,
    GetHomeInfo(EntityId),
    GetRoomInfo(EntityId),
    GetDeviceInfo(EntityId),
    ExecuteCommand(Command),
}