use clap::Parser;
use hw_006::cli::{Arguments, Cli};

fn main() {
    let args = Arguments::parse();
    Cli::process(args);
}
