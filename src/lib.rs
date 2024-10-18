use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use clap::{arg, Parser};
use chrono::Local;
use crate::img_cell::ImgCell;
use crate::rle::RLE;

mod cell_map;
mod rle;
mod img_cell;

/// Alife is a program that simulates the Conway's Game of Life.
/// It can read RLE files to generate images, GIF and RLE files.
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// RLE file to load for initial configuration
    #[arg(short, long)]
    file: std::path::PathBuf,
    /// Number of generations to simulate
    #[arg(short, long)]
    gen: u32,
    /// Display the result on standard output instead of writing to a file
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    output: bool,
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    if !args.file.is_file() {
        return Err(Box::<dyn Error>::from("value of file is invalid"))
    }
    if let Some(ext) = args.file.extension() {
        if ext != "rle" {
            return Err(Box::<dyn Error>::from("the file supplied must be a file with the .rle extension"))
        }
    } else {
        return Err(Box::<dyn Error>::from("the file extension must be provided"))
    }
    // Read file
    let content = fs::read_to_string(&args.file).unwrap_or_else(|e| {
        return e.to_string()
    });

    let rle = RLE::parse(content)?;
    let mut cell_map = rle.to_cell_map()?;

    eprintln!("Running simulation...");

    for _ in 0..args.gen {
        cell_map.generate_next();
    }

    let exported_content = RLE::cell_map_to_file(&cell_map, Some(&rle.comments));

    if !args.output {
        let mut exported_file_name_rle = String::new();
        let mut exported_file_name_png = String::new();
        if let Some(file_stem) = args.file.file_stem() {
            if let Some(file_stem_str) = file_stem.to_str() {
                exported_file_name_rle = format!("{}-gen{}.rle", file_stem_str, args.gen);
                exported_file_name_png = format!("{}-gen{}.png", file_stem_str, args.gen);
            }
        }
        if exported_file_name_rle.is_empty() || exported_file_name_png.is_empty() {
            exported_file_name_rle = "alife_export_".to_string() + &Local::now().format("%Y-%m-%d_%H-%M").to_string() + "" + ".rle";
            exported_file_name_rle = "alife_export_".to_string() + &Local::now().format("%Y-%m-%d_%H-%M").to_string() + "" + ".png";
        }

        let mut file = File::create(&exported_file_name_rle)?;
        let img_cell = ImgCell::from_cell_map(&cell_map, Some(true), Some(true));

        file.write_all(exported_content.as_bytes())?;
        img_cell.img.save(&exported_file_name_png)?;


        eprintln!("Successfully created {} and {}", &exported_file_name_rle, &exported_file_name_png);
    } else {
        eprintln!("Result of the simulation after {} generations:\n", args.gen);
        println!("{}", exported_content);
    }

    Ok(())
}