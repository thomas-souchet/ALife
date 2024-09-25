use std::error::Error;

struct CellMap {
    w: u32,
    h: u32,
    actual_generation: Vec<Vec<bool>>,
    next_generation: Vec<Vec<bool>>,
}

impl CellMap {
    fn new(source: Vec<Vec<bool>>) -> Result<CellMap, &'static str> {
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

    fn from_rle(file_name: &str) -> CellMap {
        panic!("TODO: Not implemented")
    }

    fn to_rle(&self, file_name: &str) -> Result<(), Box<dyn Error>> {
        panic!("TODO: Not implemented")
    }

    fn generate_next(&self) -> &Vec<Vec<bool>> {
        panic!("TODO: Not implemented");
        &self.actual_generation
    }

    fn actual_generation(&self) -> &Vec<Vec<bool>> {
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
    fn test_generate_next_1() -> Result<(), String> {
        let c = CellMap::new(
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
}