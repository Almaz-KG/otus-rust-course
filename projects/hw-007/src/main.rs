use clap::Parser;
use hw_007::cli::{Arguments, Cli};

fn main() {
    let args = Arguments::parse();
    Cli::run(args);
}
