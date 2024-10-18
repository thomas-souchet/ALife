use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing;
use imageproc::rect::Rect;
use crate::cell_map::CellMap;

pub struct ImgCell {
    pub img: RgbImage
}

impl ImgCell {
    const  MIN_CELL_SIZE: u32 = 5;
    const MAX_CELL_SIZE: u32 = 15;
    const LIMIT_MIN : u32 = 5;
    const LIMIT_MAX : u32 = 100;
    const GRID_LIMIT : u32 = 300;

    fn calculate_cell_size(w: u32, h: u32) -> u32 {
        let v = w.max(h);

        match v {
            v if v <= Self::LIMIT_MIN => Self::MAX_CELL_SIZE,
            v if v <= Self::LIMIT_MAX => {
                let coef = (Self::MIN_CELL_SIZE as f64 - Self::MAX_CELL_SIZE as f64) / (Self::LIMIT_MAX - Self::LIMIT_MIN) as f64;
                (coef * v as f64 + (Self::LIMIT_MIN as f64 * -coef + Self::MAX_CELL_SIZE as f64)) as u32
            },
            _ => Self::MIN_CELL_SIZE,
        }
    }

    /// Create an image from a cell map
    pub fn from_cell_map(c: &CellMap, inverted: Option<bool>, cropped: Option<bool>) -> ImgCell {
        let mut c = c;
        let grid_color = Rgb([90, 90, 90]);

        let inverted = inverted.unwrap_or(false);
        let cropped = cropped.unwrap_or(true);
        let cropped_source: CellMap;

        if cropped {
            cropped_source = c.auto_crop();
            c = &cropped_source;
        }

        let mut cell_size: u32 = Self::calculate_cell_size(c.w, c.h);
        let display_grid = c.w.max(c.h) <= Self::GRID_LIMIT;
        if !display_grid { cell_size = 1 }

        let mut width = c.w * cell_size;
        let mut height = c.h * cell_size;
        if display_grid {
            width += 1;
            height += 1;
        }

        let mut image: RgbImage = ImageBuffer::new(width, height);
        if display_grid {
            drawing::draw_hollow_rect_mut(
                &mut image,
                Rect::at(0, 0).of_size(width, height),
                grid_color);
        }

        // Dessiner le quadrillage
        for i in 0..c.actual_generation.len() {
            for j in 0..c.actual_generation[i].len() {
                if display_grid {
                    let border = Rect::at(j as i32 * cell_size as i32, i as i32 * cell_size as i32).of_size(cell_size, cell_size);
                    drawing::draw_hollow_rect_mut(&mut image, border, grid_color);
                }

                let mut x = j as i32 * cell_size as i32;
                let mut y = i as i32 * cell_size as i32;
                let mut cell_width = cell_size;
                let mut cell_height = cell_size;
                if display_grid {
                    x += 1;
                    y += 1;
                    cell_width -= 1;
                    cell_height -= 1;
                }
                let rect = Rect::at(x, y).of_size(cell_width, cell_height);
                let color = if c.actual_generation[i][j] == inverted {
                    Rgb([255, 255, 255])
                } else {
                    Rgb([0, 0, 0])
                };
                drawing::draw_filled_rect_mut(&mut image, rect, color);
            }
        }

        ImgCell { img: image }
    }
}


// --------
// Tests
// --------

#[cfg(test)]
mod tests {
    use super::*;

    // Test ImgCell::from_cell_map

    fn generate_vec(n: u32, pos: bool) -> Vec<bool> {
        let mut v = Vec::new();
        for i in 0..n {
            if i%3 == 0 {
                v.push(pos)
            } else {
                v.push(!pos)
            }
        }
        v
    }

    fn generate_map(n: u32) -> Vec<Vec<bool>> {
        let mut v = Vec::new();
        for i in 0..n {
            if i%2 == 0 {
                v.push(generate_vec(n, true))
            } else {
                v.push(generate_vec(n, false))
            }
        }
        v
    }

    #[test]
    fn test_from_cell_map_1() {
        let c = CellMap::new(
            vec![
                vec![false, true, false],
                vec![false, false, true],
                vec![true, true, true]]
        ).unwrap();

        let i = ImgCell::from_cell_map(&c, None, None);
        i.img.save("test_1.png").unwrap()
    }

    #[test]
    fn test_from_cell_map_2() {
        let c = CellMap::new(generate_map(5)).unwrap();

        let i = ImgCell::from_cell_map(&c, None, None);
        i.img.save("test_2.png").unwrap()
    }

    #[test]
    fn test_from_cell_map_3() {
        let c = CellMap::new(generate_map(47)).unwrap();

        let i = ImgCell::from_cell_map(&c, None, None);
        i.img.save("test_3.png").unwrap()
    }

    #[test]
    fn test_from_cell_map_4() {
        let c = CellMap::new(generate_map(100)).unwrap();

        let i = ImgCell::from_cell_map(&c, None, None);
        i.img.save("test_4.png").unwrap()
    }

    #[test]
    fn test_from_cell_map_5() {
        let c = CellMap::new(generate_map(300)).unwrap();

        let i = ImgCell::from_cell_map(&c, None, None);
        i.img.save("test_5.png").unwrap()
    }

    #[test]
    fn test_from_cell_map_6() {
        let c = CellMap::new(generate_map(600)).unwrap();

        let i = ImgCell::from_cell_map(&c, None, None);
        i.img.save("test_6.png").unwrap()
    }
}