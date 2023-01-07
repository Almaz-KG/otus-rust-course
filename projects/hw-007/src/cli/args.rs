use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Args, Debug)]
pub struct StartServerConf {
    /// An optional server host. If no id provided it will be considered as localhost
    #[arg(long, value_name = "host")]
    pub host: Option<String>,

    /// An optional server port. If no port provided it will be generated randomly
    #[arg(short, long, value_name = "port")]
    pub port: Option<u16>,
}

#[derive(Subcommand, Debug)]
pub enum ServerCommand {
    /// Start a new TCP server
    Start(StartServerConf),
}

#[derive(Args, Debug)]
pub struct ServerCommandWrapper {
    #[command(subcommand)]
    pub command: ServerCommand,
}

#[derive(Args, Debug)]
pub struct MakeMeasure {
    /// Device id of the device where the measure will be proceeded
    #[arg(short, long, value_name = "device_id")]
    pub device_id: String,
}

#[derive(Args, Debug)]
pub struct CreateHome {
    /// The home name
    #[arg(short, long, value_name = "name")]
    pub name: String,
    /// The home description
    #[arg(short, long, value_name = "description")]
    pub description: Option<String>,
}

#[derive(Args, Debug)]
pub struct CreateRoom {
    /// The home where the room is located
    #[arg(long, value_name = "home_id")]
    pub home_id: String,

    /// The room name
    #[arg(short, long, value_name = "name")]
    pub name: String,

    /// The room description
    #[arg(short, long, value_name = "description")]
    pub description: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum DeviceType {
    Socket,
    Thermometer,
}

#[derive(Args, Debug)]
pub struct CreateDevice {
    /// The type of the creating device
    #[arg(short, long, value_name = "device_type")]
    pub r#type: DeviceType,

    /// The room where the device is located
    #[arg(long, value_name = "room_id")]
    pub room_id: String,

    /// The device name
    #[arg(short, long, value_name = "name")]
    pub name: String,

    /// The device description
    #[arg(short, long, value_name = "description")]
    pub description: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum CreateEntity {
    /// Create a home
    Home(CreateHome),

    /// Create a room
    Room(CreateRoom),

    /// Create a device
    Device(CreateDevice),
}

#[derive(Args, Debug)]
pub struct EntityId {
    /// The id of the entity to be deleted
    #[arg(short, long, value_name = "id")]
    pub id: String,
}

#[derive(Subcommand, Debug)]
pub enum RemoveEntityCommand {
    /// Remove a home
    Home(EntityId),

    /// Remove a room
    Room(EntityId),

    /// Remove a device
    Device(EntityId),
}

#[derive(Args, Debug)]
pub struct EntityCreateCommandWrapper {
    #[command(subcommand)]
    pub command: CreateEntity,
}

pub enum DeviceStatusAction {
    Enable,
    Disable,
}

#[derive(Args, Debug)]
pub struct DeviceStatusCommand {
    /// The id of the entity to be deleted
    #[arg(short = 'i', long, value_name = "id")]
    pub device_id: String,

    /// Enable given device
    #[arg(short = 'e', long)]
    pub enable: Option<bool>,

    /// Disable given device
    #[arg(short = 'd', long)]
    pub disable: Option<bool>,
}

#[derive(Subcommand, Debug)]
pub enum StatusCommand {
    /// Status of the smart home
    Home(EntityId),

    /// Status of the room in smart home
    Room(EntityId),

    /// Status of the device in smart home
    Device(DeviceStatusCommand),
}

#[derive(Args, Debug)]
pub struct StatusCommandWrapper {
    #[command(subcommand)]
    pub command: StatusCommand,
}

#[derive(Args, Debug)]
pub struct EntityRemoveCommandWrapper {
    #[command(subcommand)]
    pub command: RemoveEntityCommand,
}

#[derive(Subcommand, Debug)]
pub enum ListEntityCommand {
    /// List all homes
    Homes,

    /// List all rooms
    Rooms,

    /// List all devices
    Devices,
}

#[derive(Args, Debug)]
pub struct ListEntityCommandWrapper {
    #[command(subcommand)]
    pub command: ListEntityCommand,
}

#[derive(Subcommand, Debug)]
#[non_exhaustive]
pub enum Command {
    /// Initialize the storage for working with current smart-home setup. It must be the first
    /// step in any further activities
    Init,

    /// The status of the smart-home entities
    Status(StatusCommandWrapper),

    /// Subcommand for creating a new entity
    New(EntityCreateCommandWrapper),

    /// Subcommand for removing an entity
    Remove(EntityRemoveCommandWrapper),

    /// List all entities in the repository
    List(ListEntityCommandWrapper),

    /// Request measurement for specific device in the home
    Measure(MakeMeasure),

    /// TCP server for serving remove smart home features
    Server(ServerCommandWrapper),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Command,
}
