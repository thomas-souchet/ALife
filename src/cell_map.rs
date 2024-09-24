use std::error::Error;

struct CellMap {
    w: u32,
    h: u32,
    actual_generation: Vec<Vec<bool>>,
    next_generation: Vec<Vec<bool>>,
}

impl CellMap {
    fn new(source: Vec<Vec<bool>>) -> CellMap {
        panic!("TODO: Not implemented")
    }

    fn from_rel(file_name: &str) -> CellMap {
        panic!("TODO: Not implemented")
    }

    fn to_rel(&self, file_name: &str) -> Result<(), Box<dyn Error>> {
        panic!("TODO: Not implemented")
    }

    fn generate_next() {
        panic!("TODO: Not implemented")
    }
}