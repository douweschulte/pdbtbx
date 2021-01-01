#![allow(dead_code)]
use crate::reference_tables;

#[derive(Debug)]
pub struct Symmetry {
    space_group: String,
    z: usize,
    index: usize,
}

impl Symmetry {
    pub fn new(space_group: String, z: usize) -> Option<Self> {
        match reference_tables::get_index_for_symbol(space_group.as_str()) {
            Some(i) => Some(Symmetry {
                space_group: space_group,
                z: z,
                index: i,
            }),
            None => return None,
        }
    }

    pub fn from_index(index: usize) -> Option<Self> {
        match reference_tables::get_symbol_for_index(index) {
            Some(s) => Some(Symmetry {
                space_group: s.to_string(),
                z: reference_tables::get_transformation(index).unwrap().len(),
                index: index,
            }),
            None => return None,
        }
    }

    pub fn space_group(&self) -> &str {
        self.space_group.as_str()
    }

    pub fn z(&self) -> usize {
        self.z
    }
}

impl Clone for Symmetry {
    fn clone(&self) -> Self {
        Symmetry {
            space_group: self.space_group.clone(),
            z: self.z,
            index: self.index,
        }
    }
}
