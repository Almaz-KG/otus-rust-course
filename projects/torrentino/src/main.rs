#![allow(dead_code)]
#![allow(unused_imports)]
use torrentino::cli::Arguments;

use clap::Parser;

fn main() {

    let arguments = Arguments::parse();
    println!("{:?}", arguments);
}