use crate::entities::house::{Home, Room};
use clap::{Args, Parser, Subcommand, ValueEnum};
use std::fs;
use std::io::BufReader;
use std::path::PathBuf;

const REPO_DIR: &str = ".smart-home";
const SMART_HOME_FILE: &str = "smart-home.json";

type SavedSmartHome = Option<Vec<Home>>;

#[derive(Args, Debug)]
pub struct Describe {
    /// Description of the specific room
    #[arg(short, long, value_name = "room_id")]
    room: Option<String>,

    /// Description of the specific apartment
    #[arg(short, long, value_name = "home_id")]
    apartment: Option<String>,

    /// Description of the specific device
    #[arg(short, long, value_name = "device_id")]
    device: Option<String>,

    /// Description of the all entities
    #[arg(long, value_name = "all")]
    all: Option<bool>,
}

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

// #[derive(Subcommand, Debug)]
// enum EntitySubCommand {
//     /// Remove entity by guid
//     // #[command(subcommand)]
//     Remove,
//
//     /// Create a new entity
//     // #[command(subcommand)]
//     New(CreateEntity),
// }
//

#[derive(Args, Debug)]
struct EntityCreateCommandWrapper {
    #[command(subcommand)]
    command: CreateEntity,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Initialize the storage for working with current smart-home setup. It must be the first
    /// step in any further activities
    Init,
    /// Prints the status of the smart-home
    Status,
    /// Prints the description of the specified entity. Requires additional parameters
    Describe(Describe),

    /// Subcommand for creating a new entity
    New(EntityCreateCommandWrapper),

    /// Subcommand for interacting with home entity
    Home,

    /// Subcommand for interacting with room entity
    Room,

    /// Subcommand for interacting with device entity
    Device,

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
            Command::Describe(_) => {}
            Command::New(command) => Cli::handle_new_command(command.command),
            Command::Home => {},
            Command::Room => { },
            Command::Device => {}
            Command::Measure(_) => {}
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
            Ok(smart_home) => {
                match smart_home {
                    None => None,
                    Some(homes) => {
                        for home in homes {
                            if home.id == id {
                                return Some(home);
                            }
                        }
                        return None
                    }
                }
                // let result: Option<&Home> =
                //     .iter()
                //     .flat_map(|homes| homes.iter().find(|h| h.id == id))
                //     .collect();
                // result
            }
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
                    Some(home) => {
                        println!("{:?}", home)
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
                .with_description(&description)
                .build()
        } else {
            Home::build().with_name(&create_home.name).build()
        }
        .expect("Unable create a home");

        match Cli::update_state(Some(vec![home])) {
            Err(msg) => {
                eprintln!("Unable to save changes: {}", msg)
            }
            _ => {}
        }
    }

    fn handle_create_room_command(room: CreateRoom) {
        match Cli::find_home_by_id(&room.home_id) {
            Some(mut home) => {
                let new_room = if let Some(ref description) = room.description {
                    Room::build()
                        .with_name(&room.name)
                        .with_description(&description)
                        .build()
                } else {
                    Room::build().with_name(&room.name).build()
                }
                    .expect("Unable create a home");

                home.rooms.push(new_room);

                match Cli::update_state(Some(vec![home])) {
                    Err(msg) => {
                        eprintln!("Unable to save changes: {}", msg)
                    }
                    _ => {}
                }
            }
            _ => {
                eprintln!("Home with id: {} not found", room.home_id)
            }
        }
    }

    fn handle_new_command(new: CreateEntity) {
        match new {
            CreateEntity::Home(home) => Cli::handle_create_home_command(home),
            CreateEntity::Room(room) => Cli::handle_create_room_command(room),
            _ => {
                eprintln!("Illegal state")
            }
        }
    }

}
