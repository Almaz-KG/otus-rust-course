#![allow(dead_code)]
#![allow(unused_imports)]

use torrentino::cli::{Arguments, Cli};

use clap::Parser;
use std::net::UdpSocket;
use std::thread;

fn main() -> Result<(), String> {
    let arguments = Arguments::parse();
    let cli = Cli::new(arguments);
    cli.process()
}
