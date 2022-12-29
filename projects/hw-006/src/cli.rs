use crate::entities::devices::{Device, Socket, Thermometer};
use crate::entities::house::{Home, Room};
use clap::{Args, ArgGroup, Parser, Subcommand, ValueEnum};
use std::fs;
use std::io::BufReader;
use std::path::PathBuf;

const REPO_DIR: &str = ".smart-home";
const SMART_HOME_FILE: &str = "smart-home.json";

type SavedSmartHome = Option<Vec<Home>>;

#[derive(Args, Debug)]
struct MakeMeasure {
    /// Device id of the device where the measure will be proceeded
    #[arg(short, long, value_name = "device_id")]
    device_id: String,
}

#[derive(Args, Debug)]
struct CreateHome {
    /// The home name
    #[arg(short, long, value_name = "name")]
    name: String,
    /// The home description
    #[arg(short, long, value_name = "description")]
    description: Option<String>,
}

#[derive(Args, Debug)]
struct CreateRoom {
    /// The home where the room is located
    #[arg(long, value_name = "home_id")]
    home_id: String,

    /// The room name
    #[arg(short, long, value_name = "name")]
    name: String,

    /// The room description
    #[arg(short, long, value_name = "description")]
    description: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum DeviceType {
    Socket,
    Thermometer,
}

#[derive(Args, Debug)]
struct CreateDevice {
    /// The type of the creating device
    #[arg(short, long, value_name = "device_type")]
    r#type: DeviceType,

    /// The room where the device is located
    #[arg(long, value_name = "room_id")]
    room_id: String,

    /// The device name
    #[arg(short, long, value_name = "name")]
    name: String,

    /// The device description
    #[arg(short, long, value_name = "description")]
    description: Option<String>,
}

#[derive(Subcommand, Debug)]
enum CreateEntity {
    /// Create a home
    Home(CreateHome),

    /// Create a room
    Room(CreateRoom),

    /// Create a device
    Device(CreateDevice),
}

#[derive(Args, Debug)]
struct RemoveEntity {
    /// The id of the entity to be deleted
    #[arg(short, long, value_name = "id")]
    id: String
}

#[derive(Subcommand, Debug)]
enum RemoveEntityCommand {
    /// Remove a home
    Home(RemoveEntity),

    /// Remove a room
    Room(RemoveEntity),

    /// Remove a device
    Device(RemoveEntity),
}

#[derive(Args, Debug)]
struct EntityCreateCommandWrapper {
    #[command(subcommand)]
    command: CreateEntity,
}

#[derive(Args, Debug)]
struct EntityRemoveCommandWrapper {
    #[command(subcommand)]
    command: RemoveEntityCommand,
}

#[derive(Args, Debug)]
#[command(group(ArgGroup::new("args").required(true).args(["homes", "rooms", "devices"])))]
struct ListEntityCommand {
    /// List all homes
    #[arg(short = 'a', long)]
    homes: bool,

    /// List all rooms
    #[arg(short, long)]
    rooms: bool,

    /// List all devices
    #[arg(short, long)]
    devices: bool,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Initialize the storage for working with current smart-home setup. It must be the first
    /// step in any further activities
    Init,

    /// Prints the status of the smart-home
    Status,

    /// Subcommand for creating a new entity
    New(EntityCreateCommandWrapper),

    /// Subcommand for removing an entity
    Remove(EntityRemoveCommandWrapper),

    /// List all entities in the repository
    List(ListEntityCommand),

    /// Request measurement for specific device in the home
    Measure(MakeMeasure),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Arguments {
    #[command(subcommand)]
    command: Command,
}

pub struct Cli {}

impl Cli {
    pub fn process(args: Arguments) {
        match args.command {
            Command::Init => Cli::init(),
            Command::Status => Cli::status(),
            Command::New(command) => Cli::handle_new_command(command.command),
            Command::Remove(command) => Cli::handle_remove_command(command.command),
            Command::Measure(_) => {}
            Command::List(entity) => Cli::handle_list_command(entity),
        }
    }

    fn get_repo_dir() -> Result<PathBuf, String> {
        let mut current_dir = std::env::current_dir()
            .map_err(|e| format!("Unable determine the current dir {}", e))?;

        current_dir.push(REPO_DIR);
        Ok(current_dir)
    }

    fn get_state_file() -> Result<PathBuf, String> {
        let mut current_dir = std::env::current_dir()
            .map_err(|e| format!("Unable determine the current dir {}", e))?;

        current_dir.push(REPO_DIR);
        current_dir.push(SMART_HOME_FILE);
        Ok(current_dir)
    }

    fn find_home_by_id(id: &str) -> Option<Home> {
        match Cli::read_smart_home_status() {
            Ok(smart_home) => smart_home.and_then(|homes| homes.into_iter().find(|h| h.id == id)),
            Err(_) => None,
        }
    }

    fn find_home_by_room_id(room_id: &str) -> Option<Home> {
        match Cli::read_smart_home_status() {
            Ok(smart_home) => smart_home.and_then(|homes| {
                homes
                    .into_iter()
                    .find(|home| home.rooms.iter().any(|r| r.id == room_id))
            }),
            Err(_) => None,
        }
    }

    fn find_room_by_id(id: &str) -> Option<Room> {
        match Cli::read_smart_home_status() {
            Ok(smart_home) => smart_home.and_then(|homes| {
                for home in homes {
                    let option = home.rooms.into_iter().find(|room| room.id == id);

                    if option.is_some() {
                        return option;
                    }
                }
                None
            }),
            Err(_) => None,
        }
    }

    fn find_room_by_device_id(device_id: &str) -> Option<Room> {
        match Cli::read_smart_home_status() {
            Ok(smart_home) => smart_home.and_then(|homes| {
                for home in homes {
                    for room in home.rooms.into_iter() {
                        let option = room.devices
                            .iter()
                            .find(|d| d.id() == device_id);

                        if option.is_some() {
                            return Some(room);
                        }
                    }
                }
                None
            }),
            Err(_) => None,
        }
    }

    fn is_smart_home_repo_exists() -> bool {
        match Cli::get_repo_dir() {
            Ok(path) => path.exists(),
            _ => false,
        }
    }

    fn read_smart_home_status() -> Result<SavedSmartHome, String> {
        let mut current_dir = std::env::current_dir().expect("Unable determine the current dir");

        current_dir.push(REPO_DIR);
        current_dir.push(SMART_HOME_FILE);
        let path = current_dir.as_path();
        let file =
            fs::File::open(path).map_err(|e| format!("Unable open smart-home state file {}", e))?;
        let reader = BufReader::new(file);

        let state: SavedSmartHome = serde_json::from_reader(reader)
            .map_err(|e| format!("{}", e))
            .expect("Unable deserialize the smart-home state");

        Ok(state)
    }

    fn update_state(home: SavedSmartHome) -> Result<(), String> {
        if Cli::is_smart_home_repo_exists() {
            let file = Cli::get_state_file()?;
            let content = serde_json::to_string(&home).unwrap();
            fs::write(file, content).expect("Unable write initial data");
            Ok(())
        } else {
            Err("No repository found. Consider to init repository first".to_string())
        }
    }

    fn update_home_state(home: Home) -> Result<(), String> {
        let status = Cli::read_smart_home_status()?;

        match status {
            None => {
                let new_state: SavedSmartHome = Some(vec![home]);
                Cli::update_state(new_state)
            }
            Some(old_home) => {
                let mut homes: Vec<Home> = old_home
                    .iter()
                    .filter(|h| h.id != home.id)
                    .cloned()
                    .collect();

                homes.push(home);
                Cli::update_state(Some(homes))
            }
        }
    }

    fn init() {
        let mut current_dir = std::env::current_dir().expect("Unable determine the current dir");

        current_dir.push(REPO_DIR);
        let path = current_dir.as_path();

        if !path.exists() {
            println!("Initializing a new repo");
            fs::create_dir(path).expect("Unable create repository");

            current_dir.push(SMART_HOME_FILE);
            let init: SavedSmartHome = None;
            let content = serde_json::to_string(&init).unwrap();

            fs::write(current_dir, content).expect("Unable write initial data");
        } else {
            println!("Repository already exists")
        }
    }

    fn status() {
        if Cli::is_smart_home_repo_exists() {
            match Cli::read_smart_home_status() {
                Ok(state) => match state {
                    Some(homes) => {
                        for home in homes {
                            println!("{}", home);
                        }
                    }
                    None => {
                        println!(
                            "Smart home is not initialized. \
                           Please create smart home instance first"
                        )
                    }
                },
                Err(msg) => {
                    eprintln!("{}", msg)
                }
            }
        } else {
            eprintln!("No repository found. Consider to init repository first")
        }
    }

    fn handle_create_home_command(create_home: CreateHome) {
        let home = if let Some(ref description) = create_home.description {
            Home::build()
                .with_name(&create_home.name)
                .with_description(description)
                .build()
        } else {
            Home::build().with_name(&create_home.name).build()
        }
        .expect("Unable create a home");

        if let Err(msg) = Cli::update_home_state(home) {
            eprintln!("Unable to save changes: {}", msg)
        }
    }

    fn handle_create_room_command(room: CreateRoom) {
        match Cli::find_home_by_id(&room.home_id) {
            Some(mut home) => {
                let new_room = if let Some(ref description) = room.description {
                    Room::build()
                        .with_name(&room.name)
                        .with_description(description)
                        .build()
                } else {
                    Room::build().with_name(&room.name).build()
                }
                .expect("Unable create a home");

                home.rooms.push(new_room);

                if let Err(msg) = Cli::update_home_state(home) {
                    eprintln!("Unable to save changes: {}", msg)
                }
            }
            _ => {
                eprintln!("Home with id: {} not found", room.home_id)
            }
        }
    }

    fn handle_create_device_command(device: CreateDevice) {
        fn create_socket(device: &CreateDevice) -> Device {
            let socket = match device.description.as_ref() {
                None => Socket::new(&device.name),
                Some(dsc) => Socket::new_with_description(&device.name, dsc),
            };

            Device::Socket(socket)
        }

        fn create_thermometer(device: &CreateDevice) -> Device {
            let thermometer = match device.description.as_ref() {
                None => Thermometer::new(&device.name),
                Some(dsc) => Thermometer::new_with_description(&device.name, dsc),
            };

            Device::Thermometer(thermometer)
        }

        match Cli::find_room_by_id(&device.room_id) {
            Some(mut room) => {
                let device = match device.r#type {
                    DeviceType::Socket => create_socket(&device),
                    DeviceType::Thermometer => create_thermometer(&device),
                };

                room.devices.push(device);
                let mut home =
                    Cli::find_home_by_room_id(&room.id).expect("Unable find home by room_id");

                let mut rooms: Vec<Room> = home
                    .rooms
                    .iter()
                    .filter(|r| r.id != room.id)
                    .cloned()
                    .collect();

                rooms.push(room);
                home.rooms = rooms;

                if let Err(msg) = Cli::update_home_state(home) {
                    eprintln!("Unable to save changes: {}", msg)
                }
            }
            _ => {
                eprintln!("Room with id: {} not found", &device.room_id)
            }
        }
    }

    fn handle_new_command(new: CreateEntity) {
        match new {
            CreateEntity::Home(home) => Cli::handle_create_home_command(home),
            CreateEntity::Room(room) => Cli::handle_create_room_command(room),
            CreateEntity::Device(device) => Cli::handle_create_device_command(device),
        }
    }

    fn remove_home_by_id(id: &str){
        let current_state = Cli::read_smart_home_status();

        match current_state {
            Ok(state) => {
                if let Some(homes) = state {
                    let new_state: Vec<Home> = homes
                        .into_iter()
                        .filter(|h| h.id != id)
                        .collect();

                    if let Err(msg) = Cli::update_state(Some(new_state)) {
                        eprintln!("Unable to save changes: {}", msg)
                    }
                }
            }
            Err(msg) => {
                eprintln!("Unable remove home: {}", msg)
            }
        }
    }

    fn remove_room_by_id(id: &str){
        let current_state = Cli::read_smart_home_status();

        match current_state {
            Ok(state) => {
                if let Some(homes) = state {
                    if let Some(mut home) = Cli::find_home_by_room_id(id) {
                        let rooms: Vec<Room> = home.rooms
                            .into_iter()
                            .filter(|r| r.id != id)
                            .collect();

                        home.rooms = rooms;

                        let mut new_state: Vec<Home> = homes
                            .into_iter()
                            .filter(|h| h.id != home.id)
                            .collect();

                        new_state.push(home);

                        if let Err(msg) = Cli::update_state(Some(new_state)) {
                            eprintln!("Unable to save changes: {}", msg)
                        }
                    }
                }
            }
            Err(msg) => {
                eprintln!("Unable remove home: {}", msg)
            }
        }

    }

    fn remove_device_by_id(id: &str){
        if let Some(mut room) = Cli::find_room_by_device_id(id) {
            let new_devices = room
                .devices
                .into_iter()
                .filter(|d| d.id() != id)
                .collect();

            room.devices = new_devices;
            if let Some(mut home) = Cli::find_home_by_room_id(&room.id) {
                let mut new_rooms: Vec<Room> = home.rooms.iter()
                    .filter(|r| r.id != room.id)
                    .cloned()
                    .collect();

                new_rooms.push(room);
                home.rooms = new_rooms;

                if let Err(msg) = Cli::update_home_state(home) {
                    eprintln!("Unable to save changes: {}", msg)
                }
            } else {
                eprintln!("Unable find associated home for room: {}", room.id)
            }
        } else {
            eprintln!("Unable find associated room for device: {}", id)
        }
    }

    fn handle_remove_command(command: RemoveEntityCommand) {
        match command {
            RemoveEntityCommand::Home(home) => Cli::remove_home_by_id(&home.id),
            RemoveEntityCommand::Room(room) => Cli::remove_room_by_id(&room.id),
            RemoveEntityCommand::Device(device) => Cli::remove_device_by_id(&device.id),
        }
    }

    fn handle_list_command(command: ListEntityCommand){
        fn print_home_ids(){
            match Cli::read_smart_home_status() {
                Ok(state) => {
                    if let Some(homes) = state {
                        for home in homes {
                            println!("{}", home.id)
                        }
                    }
                }
                Err(msg) => {eprintln!("{}", msg)}
            }
        }

        fn print_room_ids(){
            match Cli::read_smart_home_status() {
                Ok(state) => {
                    if let Some(homes) = state {
                        for home in homes {
                            for room in home.rooms {
                                println!("{}", room.id)
                            }
                        }
                    }
                }
                Err(msg) => {eprintln!("{}", msg)}
            }
        }

        fn print_device_ids(){
            match Cli::read_smart_home_status() {
                Ok(state) => {
                    if let Some(homes) = state {
                        for home in homes {
                            for room in home.rooms {
                                for device in room.devices {
                                    println!("{}", device.id())
                                }
                            }
                        }
                    }
                }
                Err(msg) => {eprintln!("{}", msg)}
            }
        }

        if command.homes {
            print_home_ids()
        }
        if command.rooms {
            print_room_ids()
        }
        if command.devices {
            print_device_ids()
        }
    }
}
