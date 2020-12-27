#![allow(dead_code)]

#[derive(Debug)]
pub struct Symmetry {
    space_group: char,
    symbols: Vec<usize>,
}

impl Symmetry {
    pub fn new(space_group: char, symbols: Vec<usize>) -> Symmetry {
        Symmetry {
            space_group: space_group,
            symbols: symbols,
        }
    }

    pub fn symbols(&self) -> &Vec<usize> {
        &self.symbols
    }

    pub fn space_group(&self) -> char {
        self.space_group
    }
}

impl Clone for Symmetry {
    fn clone(&self) -> Self {
        Symmetry::new(self.space_group, self.symbols.clone())
    }
}
