use crate::entities::house::Home;
use clap::{Args, Parser, Subcommand};
use std::fs;

const REPO_DIR: &str = ".smart-home";
const SMART_HOME_FILE: &str = "smart-home.json";

type SavedSmartHome = Option<Home>;

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
struct EntityInteractionCommand {}

#[derive(Subcommand, Debug)]
enum Command {
    /// Initialize the storage for working with current smart-house setup. It must be the first
    /// step in any further activities
    Init,
    /// Prints the status of the smart-house
    Status,
    /// Prints the description of the specified entity. Requires additional parameters
    Describe(Describe),

    /// Subcommand for interacting with house entity
    Houses(EntityInteractionCommand),

    /// Subcommand for interacting with room entity
    Rooms(EntityInteractionCommand),

    /// Subcommand for interacting with device entity
    Devices(EntityInteractionCommand),

    /// Request measurement for specific device in the house
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
            Command::Status => {}
            Command::Describe(_) => {}
            Command::Houses(_) => {}
            Command::Rooms(_) => {}
            Command::Devices(_) => {}
            Command::Measure(_) => {}
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
}
