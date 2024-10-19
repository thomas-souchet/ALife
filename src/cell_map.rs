use std::cmp::PartialEq;
use std::mem;

enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

pub struct CellMap {
    pub w: u32,
    pub h: u32,
    pub actual_generation: Vec<Vec<bool>>,
    next_generation: Vec<Vec<bool>>,
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Direction::Top, Direction::Top) => true,
            (Direction::Right, Direction::Right) => true,
            (Direction::Bottom, Direction::Bottom) => true,
            (Direction::Left, Direction::Left) => true,
            _ => false,
        }
    }
}

impl CellMap {

    /// Add dead cells around the figure
    fn expand(&mut self, all_d: &Vec<Direction>) {
        if all_d.contains(&Direction::Top) {
            self.actual_generation.insert(0, vec![false; self.w as usize]);
            self.next_generation.insert(0, vec![false; self.w as usize]);
            self.h += 1;
        }
        if all_d.contains(&Direction::Bottom) {
            self.actual_generation.push(vec![false; self.w as usize]);
            self.next_generation.push(vec![false; self.w as usize]);
            self.h += 1;
        }
        for i in 0..self.actual_generation.len() {
            if all_d.contains(&Direction::Right) {
                self.actual_generation[i].push(false);
                self.next_generation[i].push(false);
                if i == 0 { self.w += 1 }
            }
            if all_d.contains(&Direction::Left) {
                self.actual_generation[i].insert(0, false);
                self.next_generation[i].insert(0, false);
                if i == 0 { self.w += 1 }
            }
        }
    }

    /// Remove empty lines at the start and end of the figure
    fn _trim(&mut self) {
        if self.actual_generation.len() > 0 {
            while !self.actual_generation[0].contains(&true) {
                self.actual_generation.remove(0);
                self.next_generation.remove(0);
            }

            let mut last =  self.actual_generation.len() - 1;

            while !self.actual_generation[last].contains(&true) {
                self.actual_generation.remove(last);
                self.next_generation.remove(last);
                last -= 1;
            }
        }

        self.h = self.actual_generation.len() as u32;
    }

    // ---------------------

    /// Reduce the figure to the minimum size removing all the empty columns and lines
    pub fn auto_crop(&self) -> CellMap {
        let (mut start_y, mut end_y): (usize, usize) = ((self.h - 1) as usize, 0);
        let (mut start_x, mut end_x): (usize, usize) = ((self.w - 1) as usize, 0);

        if let Some(index) = self.actual_generation.iter().position(|y| y.contains(&true)) {
            if index < start_y { start_y = index }
        }
        if let Some(index) = self.actual_generation.iter().rposition(|y| y.contains(&true)) {
            if index > end_y { end_y = index }
        }

        for i in start_y..=end_y {
            if let Some(index) = self.actual_generation[i].iter().position(|x| *x) {
                if index < start_x { start_x = index }
            }
            if let Some(index) = self.actual_generation[i].iter().rposition(|x| *x) {
                if index > end_x { end_x = index }
            }
        }

        // Copy only the cropped part
        let (w, h) = (end_x - start_x + 1, end_y - start_y + 1);

        let mut new_actual_generation = Vec::with_capacity(h);
        for i in start_y..=end_y {
            let mut line = Vec::with_capacity(w);
            for j in start_x..=end_x {
                line.push(self.actual_generation[i][j]);
            }
            new_actual_generation.push(line);
        }


        CellMap {
            w: w as u32,
            h: h as u32,
            actual_generation: new_actual_generation,
            next_generation: vec![vec![false; w]; h]
        }
    }

    /// Create a new instance of a CellMap from a two-dimensional vector of booleans
    pub fn new(source: Vec<Vec<bool>>) -> Result<CellMap, &'static str> {
        let col_size = source.len();
        if col_size == 0 {
            return Err("[CellMap creation] The source can't be empty")
        }
        let row_size = source[0].len();
        for r in source.iter() {
            if r.len() != row_size {
                return Err("[CellMap creation] All rows must have the same size")
            }
        }
        Ok (CellMap {
            w: row_size as u32,
            h: col_size as u32,
            actual_generation: source,
            next_generation: vec![vec![false; row_size]; col_size],
        })
    }

    /// Generate the next generation following the rule of the game of life
    pub fn generate_next(&mut self) {
        // Detect if expand is necessary
        let mut all_directions = vec![];
        if self.actual_generation[0].contains(&true) {
            all_directions.push(Direction::Top);
        }
        if self.actual_generation.last().map_or(false, |row| row.contains(&true)) {
            all_directions.push(Direction::Bottom);
        }
        if self.actual_generation.iter().any(|row| row.first() == Some(&true)) {
            all_directions.push(Direction::Left);
        }
        if self.actual_generation.iter().any(|row| row.last() == Some(&true)) {
            all_directions.push(Direction::Right);
        }
        self.expand(&all_directions);


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
    }
}


// --------
// Tests
// --------

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
            Err(e) => assert_eq!(e, "[CellMap creation] The source can't be empty"),
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
            Err(e) => assert_eq!(e, "[CellMap creation] All rows must have the same size"),
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

        c.generate_next();

        assert_eq!(c.actual_generation, vec![
            vec![false, false, false, false, false],
            vec![false, true, true, false, false],
            vec![false, true, false, false, false],
            vec![false, true, true, true, false],
            vec![false, false, false, false, false],
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

        c.generate_next();

        assert_eq!(c.actual_generation, vec![
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

        c.generate_next();

        assert_eq!(c.actual_generation, vec![
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

        c.generate_next();

        assert_eq!(c.actual_generation, vec![
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

        c.generate_next();

        assert_eq!(c.actual_generation, vec![
            vec![false, false, false, false, false, false],
            vec![false, false, false, true, false, false],
            vec![false, true, false, false, true, false],
            vec![false, true, false, false, true, false],
            vec![false, false, true, false, false, false],
            vec![false, false, false, false, false, false]
        ]);
        Ok(())
    }

    // Test CellMap.auto_crop

    #[test]
    fn test_auto_crop() {
        let c = CellMap::new(vec![
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, true, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, true, false],
            vec![false, false, false, false, false],
            vec![false, true, true, true, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
        ]).unwrap();

        let cropped_c = c.auto_crop();

        assert_eq!(cropped_c.actual_generation,
                   vec![
                       vec![false, true, false],
                       vec![false, false, false],
                       vec![false, false, true],
                       vec![false, false, false],
                       vec![true, true, true],
                   ]
        );
        assert_eq!(cropped_c.w, 3);
        assert_eq!(cropped_c.h, 5);
        assert_eq!(cropped_c.next_generation.len(), 5);
        assert_eq!(cropped_c.next_generation[0].len(), 3);
    }

    // Test CellMap.expand

    #[test]
    fn test_expand_1() {
        let mut c = CellMap::new(vec![
            vec![false, true, false],
            vec![false, false, true],
            vec![true, true, true],
        ]).unwrap();

        c.expand(&vec![Direction::Left]);

        assert_eq!(c.w, 4);
        assert_eq!(c.h, 3);
        assert_eq!(c.next_generation.len(), 3);
        assert_eq!(c.next_generation[0].len(), 4);
        assert_eq!(c.actual_generation, vec![
            vec![false, false, true, false],
            vec![false, false, false, true],
            vec![false, true, true, true],
        ])
    }
}