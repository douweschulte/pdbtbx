#![allow(dead_code)]
use crate::transformation::*;

#[derive(Debug)]
pub struct MtriX {
    pub serial_number: usize,
    transformation: TransformationMatrix,
    pub contained: bool,
    rows_set: [bool; 3],
}

impl MtriX {
    pub fn new() -> Self {
        MtriX {
            serial_number: 0,
            transformation: TransformationMatrix::identity(),
            contained: false,
            rows_set: [true, true, true],
        }
    }
    pub fn transformation(&self) -> &TransformationMatrix {
        &self.transformation
    }
    pub fn set_row(&mut self, row: usize, data: [f64; 4]) {
        if row > 2 {
            panic!(format!(
                "Row in MtriX.set_row is too big (max 2, value: {})",
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

impl Clone for MtriX {
    fn clone(&self) -> Self {
        let mut orig = MtriX::new();

        orig.transformation = self.transformation.clone();

        orig
    }
}
