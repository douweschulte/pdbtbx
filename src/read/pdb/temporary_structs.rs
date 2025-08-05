use crate::TransformationMatrix;

/// To help build a a matrix from separate rows
pub(crate) struct BuildUpMatrix {
    /// First row
    pub row0: Option<[f64; 4]>,
    /// Second row
    pub row1: Option<[f64; 4]>,
    /// Third row
    pub row2: Option<[f64; 4]>,
}

impl BuildUpMatrix {
    /// Create an empty struct
    pub(crate) const fn empty() -> Self {
        Self {
            row0: None,
            row1: None,
            row2: None,
        }
    }
    /// Consume this struct and get the transformation matrix, if any row is not defined it returns None
    pub(crate) const fn get_matrix(&self) -> Option<TransformationMatrix> {
        match self {
            Self {
                row0: Some(r1),
                row1: Some(r2),
                row2: Some(r3),
            } => Some(TransformationMatrix::from_matrix([*r1, *r2, *r3])),
            _ => None,
        }
    }
    /// Determine if all rows are set
    pub(crate) const fn is_set(&self) -> bool {
        matches!(
            self,
            Self {
                row0: Some(_),
                row1: Some(_),
                row2: Some(_),
            }
        )
    }
    /// Determine if at least one row is set
    pub(crate) const fn is_partly_set(&self) -> bool {
        self.row0.is_some() || self.row1.is_some() || self.row2.is_some()
    }
    /// Set a specified row
    pub(crate) fn set_row(&mut self, row: usize, data: [f64; 4]) {
        match row {
            0 => self.row0 = Some(data),
            1 => self.row1 = Some(data),
            2 => self.row2 = Some(data),
            _ => panic!("Invalid value in 'set_row' on a BuildUpMatrix"),
        }
    }
}
