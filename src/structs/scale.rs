#![allow(dead_code)]
use crate::transformation::*;

#[derive(Debug, Clone)]
/// A scale transformation of a crystal, to get from standard orthogonal coordinates to fractional coordinates
pub struct Scale {
    /// The transformation from standard orthogonal coordinates to fractional coordinates
    transformation: TransformationMatrix,
    /// For validation, only if all rows are set this scale is valid
    rows_set: [bool; 3],
}

impl Scale {
    /// Create an empty transformation (identity)
    pub fn new() -> Self {
        Scale {
            transformation: TransformationMatrix::identity(),
            rows_set: [true, true, true],
        }
    }
    /// Get the transformation from standard orthogonal coordinates to fractional coordinates
    pub fn transformation(&self) -> &TransformationMatrix {
        &self.transformation
    }
    /// Set the transformation from standard orthogonal coordinates to fractional coordinates
    pub fn set_transformation(&mut self, transformation: TransformationMatrix) {
        self.transformation = transformation;
    }
    /// Set a row to the given data, this invalidates all other rows if the scale was valid before
    /// otherwise it validates the row given.
    /// To have a valid Scale all rows have to be set.
    ///
    /// ## Arguments
    /// * `row` - 0-based row to fill the data into
    /// * `data` - the row of data
    ///
    /// ## Example
    ///
    /// ```
    /// use rust_pdb::TransformationMatrix;
    /// use rust_pdb::Scale;
    /// let mut example = Scale::new();
    /// example.set_row(1, [0.0, 1.0, 0.0, 0.0]);
    /// example.set_row(2, [0.0, 0.0, 1.0, 0.0]);
    /// example.set_row(0, [1.0, 0.0, 0.0, 0.0]); // The order does not matter
    /// assert_eq!(example.transformation(), &TransformationMatrix::identity());
    /// assert!(example.valid());
    /// ```
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
    /// Checks if this Scale is valid, for this all rows have to be set (also see `set_row`).
    /// Mainly used to validate a structure after parsing.
    pub fn valid(&self) -> bool {
        self.rows_set == [true, true, true]
    }
}

impl PartialEq for Scale {
    fn eq(&self, other: &Self) -> bool {
        self.transformation == other.transformation
    }
}

impl Default for Scale {
    fn default() -> Self {
        Self::new()
    }
}
