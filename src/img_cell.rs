use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing;
use imageproc::rect::Rect;
use crate::cell_map::CellMap;

pub struct ImgCell {

}

impl ImgCell {

    /// Create an image from a cell map
    pub fn from_cell_map(c: &CellMap, file_name: &str, inverted: Option<bool>) {
        let cell_size: u32 = 20;
        let grid_color = Rgb([70, 70, 70]);

        let inverted = inverted.unwrap_or(true);
        let width = c.w * cell_size + 1;
        let height = c.h * cell_size + 1;

        let mut image: RgbImage = ImageBuffer::new(width, height);
        drawing::draw_hollow_rect_mut(
            &mut image,
            Rect::at(0, 0).of_size(width, height),
            grid_color);

        // Dessiner le quadrillage
        for i in 0..c.actual_generation.len() {
            for j in 0..c.actual_generation[i].len() {
                let border = Rect::at((j * 20) as i32, (i * 20) as i32).of_size(cell_size, cell_size);
                let rect = Rect::at((j * 20 + 1) as i32, (i * 20 + 1) as i32).of_size(cell_size-1, cell_size-1);
                let color = if c.actual_generation[i][j] == inverted {
                    Rgb([255, 255, 255])
                } else {
                    Rgb([0, 0, 0])
                };
                drawing::draw_hollow_rect_mut(&mut image, border, grid_color);
                drawing::draw_filled_rect_mut(&mut image, rect, color);
            }
        }

        image.save(file_name).unwrap();
    }
}


// --------
// Tests
// --------

#[cfg(test)]
mod tests {
    use super::*;

    // Test ImgCell::from_cell_map

    #[test]
    fn test_from_cell_map_1() {
        let c = CellMap::new(
            vec![
                vec![false, true, false],
                vec![false, false, true],
                vec![true, true, true]]
        ).unwrap();

        ImgCell::from_cell_map(&c, "test_1.png", None);
    }
}