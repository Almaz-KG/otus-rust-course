use torrentino::cli::{Arguments, Cli};

use clap::Parser;

fn main() -> Result<(), String> {
    let arguments = Arguments::parse();
    let cli = Cli::new(arguments);
    cli.process()
}