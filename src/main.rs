use std::process;
use clap::Parser;

use alife::Args;


fn main() {
    let args = Args::parse();

    if let Err(e) = alife::run(args) {
        eprintln!("Fatal error, {}", e);
        process::exit(1);
    }
}
