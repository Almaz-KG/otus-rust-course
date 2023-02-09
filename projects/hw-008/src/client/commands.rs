type EntityId = String;
type Command = String;

#[derive(Debug)]
pub enum ClientCommand {
    GetHomeInfo(EntityId),
    GetRoomInfo(EntityId),
    GetDeviceInfo(EntityId),
    ExecuteCommand(Command),
}