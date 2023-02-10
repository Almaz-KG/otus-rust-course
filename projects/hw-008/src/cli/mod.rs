mod args;
mod command_handler;

pub use args::*;
pub use command_handler::*;

use std::{env, io};

pub struct Cli {}

impl Cli {
    pub fn run(args: Arguments) {
        let mut output = io::stdout();
        let path = env::current_dir().unwrap();
        let mut handler = CommandHandler::new(&mut output, path);
        handler.process(args.command)
    }
}
