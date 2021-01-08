#![allow(dead_code)]
use crate::transformation::*;

#[derive(Debug, Clone)]
/// A transformation of the orthogonal coordinates to submitted
pub struct OrigX {
    /// The transformation from orthogonal to submitted coordinates
    transformation: TransformationMatrix,
    /// For validation, only if all rows are set this origx is valid
    rows_set: [bool; 3],
}

impl OrigX {
    /// Create an empty transformation (identity)
    pub fn new() -> OrigX {
        OrigX {
            transformation: TransformationMatrix::identity(),
            rows_set: [true, true, true],
        }
    }
    /// Get the transformation from orthogonal to submitted coordinates
    pub fn transformation(&self) -> &TransformationMatrix {
        &self.transformation
    }
    /// Set the transformation from orthogonal to submitted coordinates
    pub fn set_transformation(&mut self, transformation: TransformationMatrix) {
        self.transformation = transformation;
    }
    /// Set a row to the given data, this invalidates all other rows if the origx was valid before
    /// otherwise it validates the row given.
    /// To have a valid origx all rows have to be set.
    ///
    /// ## Arguments
    /// * `row` - 0-based row to fill the data into
    /// * `data` - the row of data
    ///
    /// ## Example
    ///
    /// ```
    /// use pdbtbx::TransformationMatrix;
    /// use pdbtbx::OrigX;
    /// let mut example = OrigX::new();
    /// example.set_row(1, [0.0, 1.0, 0.0, 0.0]);
    /// example.set_row(2, [0.0, 0.0, 1.0, 0.0]);
    /// example.set_row(0, [1.0, 0.0, 0.0, 0.0]); // The order does not matter
    /// assert_eq!(example.transformation(), &TransformationMatrix::identity());
    /// assert!(example.valid());
    /// ```
    pub fn set_row(&mut self, row: usize, data: [f64; 4]) {
        if row > 2 {
            panic!(format!(
                "Row in OrigX.set_row is too big (max 2, value: {})",
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
    /// Checks if this OrigX is valid, for this all rows have to be set (also see `set_row`).
    /// Mainly used to validate a structure after parsing.
    pub fn valid(&self) -> bool {
        self.rows_set == [true, true, true]
    }
}

impl PartialEq for OrigX {
    fn eq(&self, other: &Self) -> bool {
        self.transformation == other.transformation
    }
}

impl Default for OrigX {
    fn default() -> Self {
        Self::new()
    }
}
