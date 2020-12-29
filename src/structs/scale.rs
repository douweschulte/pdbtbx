#![allow(dead_code)]
use crate::transformation::*;

#[derive(Debug)]
pub struct Scale {
    transformation: TransformationMatrix,
    rows_set: [bool; 3],
}

impl Scale {
    pub fn new() -> Self {
        Scale {
            transformation: TransformationMatrix::identity(),
            rows_set: [true, true, true],
        }
    }
    pub fn transformation(&self) -> &TransformationMatrix {
        &self.transformation
    }
    pub fn set_row(&mut self, row: usize, data: [f64; 4]) {
        if row > 2 {
            panic!(format!(
                "Row in Scale.set_row is too big (max 2, value: {})",
                row
            ));
        }
        let mut matrix = self.transformation.matrix();
        matrix[row] = data;
        self.transformation.set_matrix(matrix);
        if self.rows_set == [true, true, true] {
            self.rows_set = [false, false, false];
        }
        self.rows_set[row] = true;
    }
    pub fn valid(&self) -> bool {
        self.rows_set == [true, true, true]
    }
}

impl Clone for Scale {
    fn clone(&self) -> Self {
        let mut orig = Scale::new();

        orig.transformation = self.transformation.clone();

        orig
    }
}
