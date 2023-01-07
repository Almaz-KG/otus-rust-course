mod args;
mod command_handler;

pub use args::*;
pub use command_handler::*;

use std::io;

pub struct Cli {}

impl Cli {
    pub fn run(args: Arguments) {
        let mut output = io::stdout();
        let mut handler = CommandHandler::new(&mut output);
        handler.process(args.command)
    }
}
