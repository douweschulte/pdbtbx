#![allow(dead_code)]
use crate::transformation::*;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
/// A transformation expressing non-crystallographic symmetry, used when transformations are required to generate the whole asymmetric subunit
pub struct MtriX {
    /// The serial number of this transformation
    pub serial_number: usize,
    /// The transformation
    pub transformation: TransformationMatrix,
    /// If the coordinates of the molecule are contained in the entry this is true
    pub contained: bool,
}

impl MtriX {
    /// Create a new MtriX with the given arguments
    pub fn new(
        serial_number: usize,
        transformation: TransformationMatrix,
        contained: bool,
    ) -> Self {
        MtriX {
            serial_number,
            transformation,
            contained,
        }
    }
}

impl PartialEq for MtriX {
    fn eq(&self, other: &Self) -> bool {
        self.transformation == other.transformation
            && self.serial_number == other.serial_number
            && self.contained == other.contained
    }
}

impl PartialOrd for MtriX {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.serial_number.cmp(&other.serial_number))
    }
}

impl Default for MtriX {
    fn default() -> Self {
        Self::new(0, TransformationMatrix::identity(), false)
    }
}
