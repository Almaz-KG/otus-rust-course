use std::io;
use crate::cli::{Arguments, CommandHandler};

pub struct Cli {}

impl Cli {
    pub fn run(args: Arguments){
        let mut output = io::stdout();
        let mut handler = CommandHandler::new(Box::new(&mut output));
        handler.process(args.command)
    }
}
