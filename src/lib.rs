use std::error::Error;
use crate::config::Config;

mod cell_map;
pub mod config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    panic!("TODO: Not implemented")
}