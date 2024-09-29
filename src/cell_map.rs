use std::error::Error;
use std::{fs, mem};
use crate::rle::RLE;

pub struct CellMap {
    pub w: u32,
    pub h: u32,
    pub actual_generation: Vec<Vec<bool>>,
    next_generation: Vec<Vec<bool>>,
}

impl CellMap {
    pub fn new(source: Vec<Vec<bool>>) -> Result<CellMap, &'static str> {
        let col_size = source.len();
        if col_size == 0 {
            return Err("The source can't be empty")
        }
        let row_size = source[0].len();
        for r in source.iter() {
            if r.len() != row_size {
                return Err("All rows must have the same size")
            }
        }
        Ok (CellMap {
            w: row_size as u32,
            h: col_size as u32,
            actual_generation: source,
            next_generation: vec![vec![false; row_size]; col_size],
        })
    }

    fn from_rle(file_name: &str) -> Result<CellMap, &'static str> {
        // File
        if !file_name.contains(".rle") {
            return Err("The input file must be a Run Length Encoded (.rle) file");
        }
        let content = fs::read_to_string(file_name).unwrap_or_else(|e| {
            return e.to_string()
        });
        // RLE
        let rle = RLE::parse(content)?;
        Ok(rle.to_cell_map())
    }

    fn to_rle(&self, file_name: &str) -> Result<(), Box<dyn Error>> {
        panic!("TODO: Not implemented")
    }

    fn generate_next(&mut self) -> &Vec<Vec<bool>> {
        for i in 0..self.actual_generation.len() {
            for j in 0..self.actual_generation[i].len() {
                let (i, j) = (i as i32, j as i32);
                // Count alive cells
                let mut alive = 0;
                let coord = [(i-1, j-1), (i-1, j), (i-1, j+1), (i, j-1), (i, j+1), (i+1, j-1), (i+1, j), (i+1, j+1)];
                for (y, x) in coord {
                    if let Some(row) = self.actual_generation.get(y as usize) {
                        if let Some(v) = row.get(x as usize) {
                            if *v { alive += 1 }
                        }
                    }
                }

                // Apply game rules
                let (i, j) = (i as usize, j as usize);
                self.next_generation[i][j] = match (self.actual_generation[i][j], alive) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                }
            }
        }
        // Swap pointers
        mem::swap(&mut self.actual_generation, &mut self.next_generation);

        &self.actual_generation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test CellMap::new
    #[test]
    fn test_new_1() -> Result<(), &'static str> {
        let c = CellMap::new(
            vec![
                vec![false, true, false],
                vec![true, true, false],
                vec![false, true, true]]
        )?;

        assert_eq!(c.h, 3);
        assert_eq!(c.w, 3);
        assert_eq!(c.actual_generation, vec![
            vec![false, true, false],
            vec![true, true, false],
            vec![false, true, true]]);
        Ok(())
    }

    #[test]
    fn test_new_2() {
        match CellMap::new(vec![]) {
            Ok(_) => panic!("The result should not be Ok"),
            Err(e) => assert_eq!(e, "The source can't be empty"),
        }
    }

    #[test]
    fn test_new_3() {
        match CellMap::new(vec![
            vec![false, true],
            vec![true, false, true],
            vec![true, false],
        ]) {
            Ok(_) => panic!("The result should not be Ok"),
            Err(e) => assert_eq!(e, "All rows must have the same size"),
        }
    }

    // Test CellMap.generate_next
    #[test]
    fn test_generate_next_figure() -> Result<(), String> {
        let mut c = CellMap::new(
            vec![
                vec![false, true, false],
                vec![true, true, false],
                vec![false, true, true]]
        )?;

        let result = c.generate_next();

        assert_eq!(result, &vec![
            vec![true, true, false],
            vec![true, false, false],
            vec![true, true, true]
        ]);
        Ok(())
    }

    #[test]
    fn test_generate_next_blinker() -> Result<(), String> {
        // Test for a blinker pattern
        let mut c = CellMap::new(
            vec![
                vec![false, false, false, false, false],
                vec![false, false, true, false, false],
                vec![false, false, true, false, false],
                vec![false, false, true, false, false],
                vec![false, false, false, false, false]]
        )?;

        let result = c.generate_next();

        assert_eq!(result, &vec![
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, true, true, true, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false]
        ]);
        Ok(())
    }

    #[test]
    fn test_generate_next_glider() -> Result<(), String> {
        // Test for a glider pattern
        let mut c = CellMap::new(
            vec![
                vec![false, false, false, false, false],
                vec![false, false, true, false, false],
                vec![false, false, false, true, false],
                vec![false, true, true, true, false],
                vec![false, false, false, false, false]]
        )?;

        let result = c.generate_next();

        assert_eq!(result, &vec![
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, true, false, true, false],
            vec![false, false, true, true, false],
            vec![false, false, true, false, false]
        ]);
        Ok(())
    }

    #[test]
    fn test_generate_next_beehive() -> Result<(), String> {
        // Test for a beehive pattern (stable)
        let mut c = CellMap::new(
            vec![
                vec![false, false, false, false, false, false],
                vec![false, false, true, true, false, false],
                vec![false, true, false, false, true, false],
                vec![false, false, true, true, false, false],
                vec![false, false, false, false, false, false]]
        )?;

        let result = c.generate_next();

        assert_eq!(result, &vec![
            vec![false, false, false, false, false, false],
            vec![false, false, true, true, false, false],
            vec![false, true, false, false, true, false],
            vec![false, false, true, true, false, false],
            vec![false, false, false, false, false, false]
        ]);
        Ok(())
    }

    #[test]
    fn test_generate_next_toad() -> Result<(), String> {
        // Test for a toad pattern (period 2 oscillator)
        let mut c = CellMap::new(
            vec![
                vec![false, false, false, false, false, false],
                vec![false, false, false, false, false, false],
                vec![false, false, true, true, true, false],
                vec![false, true, true, true, false, false],
                vec![false, false, false, false, false, false],
                vec![false, false, false, false, false, false]]
        )?;

        let result = c.generate_next();

        assert_eq!(result, &vec![
            vec![false, false, false, false, false, false],
            vec![false, false, false, true, false, false],
            vec![false, true, false, false, true, false],
            vec![false, true, false, false, true, false],
            vec![false, false, true, false, false, false],
            vec![false, false, false, false, false, false]
        ]);
        Ok(())
    }
}