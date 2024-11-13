use std::fs;
use std::process::{Command, Stdio};
use std::io::{Cursor, Write};
use image::{DynamicImage, Rgb, RgbImage};
use crate::cell_map::CellMap;
use crate::img_cell::ImgCell;

pub struct VideoCell {

}

impl VideoCell {
    // WIP
    fn  generate() -> std::io::Result<()> {
        let mut ffmpeg = Command::new("ffmpeg")
            .arg("-y")
            .arg("-f").arg("image2pipe")
            .arg("-framerate").arg("2")
            .arg("-i").arg("-")
            .arg("-pix_fmt").arg("yuv420p")
            .arg("-vf").arg("scale=1080:1080,pad=1920:1080:(ow-iw)/2:(oh-ih)/2")
            .arg("tests/samples/output.mp4")
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .spawn()?;

        //let c = CellMap::new(vec![vec![true; 1080]; 1080]).unwrap();
        let c = CellMap::new(vec![
            vec![false, true, false],
            vec![false, false, true],
            vec![true, true, true]
        ]).unwrap();

        if let Some(mut stdin) = ffmpeg.stdin.take() {
            for i in 0..10 {

                let mut img = ImgCell::from_cell_map(&c, Some(true), None).img;

                let buffer = Vec::new();
                let mut writer = Cursor::new(buffer);
                img.write_to(&mut writer, image::ImageFormat::Png)
                    .expect("Failed to write image to FFmpeg");


                stdin.write_all(writer.get_ref())
                    .expect("Failed to write image to FFmpeg");
            }

            // Fermer stdin pour signaler la fin de l'entrée
            drop(stdin);
        }

        // Attendre que la commande se termine et obtenir le statut
        let status = ffmpeg.wait()?;
        if !status.success() {
            eprintln!("FFmpeg a échoué avec le statut : {:?}", status);
        }
        println!("Statut de sortie : {}", status);

        Ok(())
    }
}


// --------
// Tests
// --------

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use super::*;

    const PATH: &str = "tests/samples";

    /// Test init before each
    fn initialize(path: &str) {
        let path = Path::new(path);
        if !path.exists() {
            fs::create_dir_all(path).expect("Failed to create directories");
        } else if !path.is_dir() {
            panic!("Path already exists but it's not a directory");
        }
    }

    // Test VideoCell::generate

    #[test]
    fn test_generate_1() {
        initialize(PATH);
        VideoCell::generate().unwrap()
    }
}