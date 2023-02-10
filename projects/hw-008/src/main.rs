use clap::Parser;
use hw_008::cli::{Arguments, Cli};

fn main() {
    let args = Arguments::parse();
    Cli::run(args);
}
