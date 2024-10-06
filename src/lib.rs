use std::error::Error;
use crate::config::Config;

mod cell_map;
pub mod config;
mod rle;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    /* Temporary code
    // File
    if !file_name.contains(".rle") {
        return Err("The input file must be a Run Length Encoded (.rle) file");
    }
    let content = fs::read_to_string(file_name).unwrap_or_else(|e| {
        return e.to_string()
    });
    // RLE
    let rle = RLE::parse(content)?;
    Ok(rle.to_cell_map()?)
     */
    panic!("TODO: Not implemented")
}