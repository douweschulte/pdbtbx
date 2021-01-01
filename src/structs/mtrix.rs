#![allow(dead_code)]
use crate::transformation::*;

#[derive(Debug)]
/// A transformation expressing non-crystallographic symmetry, used when transformations are required to generate the whole asymmetric subunit
pub struct MtriX {
    /// The serial number of this transformation
    pub serial_number: usize,
    /// The transformation
    transformation: TransformationMatrix,
    /// If the coordinates of the molecule are contained in the entry this is true
    pub contained: bool,
    /// For validation, only if all rows are set this scale is valid
    rows_set: [bool; 3],
}

impl MtriX {
    /// Create an empty transformation (identity), not contained, and with a serial number of 0.
    pub fn new() -> Self {
        MtriX {
            serial_number: 0,
            transformation: TransformationMatrix::identity(),
            contained: false,
            rows_set: [true, true, true],
        }
    }
    /// The serial number of this transformation
    pub fn serial_number(&self) -> usize {
        self.serial_number
    }
    /// Set the serial number of this transformation
    pub fn set_serial_number(&mut self, new_value: usize) {
        self.serial_number = new_value;
    }
    /// The 'contained' status, see wwPDB 3.30 MTRIXn p. 184. If the coordinates of the molecule are contained in the entry this is true.
    pub fn contained(&self) -> bool {
        self.contained
    }
    /// Set the contained status
    pub fn set_contained(&mut self, new_value: bool) {
        self.contained = new_value;
    }
    /// The transformation
    pub fn transformation(&self) -> &TransformationMatrix {
        &self.transformation
    }
    /// Set the transformation
    pub fn set_transformation(&mut self, transformation: TransformationMatrix) {
        self.transformation = transformation;
    }
    /// Set a row to the given data, this invalidates all other rows if the MtriX was valid before
    /// otherwise it validates the row given.
    /// To have a valid MtriX all rows have to be set.
    ///
    /// ## Arguments
    /// * `row` - 0-based row to fill the data into
    /// * `data` - the row of data
    ///
    /// ## Example
    ///
    /// ```
    /// use rust_pdb::TransformationMatrix;
    /// use rust_pdb::MtriX;
    /// let mut example = MtriX::new();
    /// example.set_row(1, [0.0, 1.0, 0.0, 0.0]);
    /// example.set_row(2, [0.0, 0.0, 1.0, 0.0]);
    /// example.set_row(0, [1.0, 0.0, 0.0, 0.0]); // The order does not matter
    /// assert_eq!(example.transformation(), &TransformationMatrix::identity());
    /// assert!(example.valid());
    /// ```
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
    /// Checks if this MtriX is valid, for this all rows have to be set (also see `set_row`).
    /// Mainly used to validate a structure after parsing.
    pub fn valid(&self) -> bool {
        self.rows_set == [true, true, true]
    }
}

impl Clone for MtriX {
    fn clone(&self) -> Self {
        let mut mtrix = MtriX::new();

        mtrix.transformation = self.transformation.clone();
        mtrix.serial_number = self.serial_number;
        mtrix.contained = self.contained;
        mtrix.rows_set = self.rows_set.clone();

        mtrix
    }
}
