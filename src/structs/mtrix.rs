#![allow(dead_code)]
use crate::transformation::TransformationMatrix;
use std::cmp::Ordering;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
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
    #[must_use]
    pub const fn new(
        serial_number: usize,
        transformation: TransformationMatrix,
        contained: bool,
    ) -> Self {
        Self {
            serial_number,
            transformation,
            contained,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equality() {
        let a = MtriX::default();
        let b = MtriX::new(0, TransformationMatrix::identity(), false);
        let c = MtriX::new(1, TransformationMatrix::identity(), true);
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(b, c);
        assert!(a < c);
    }

    #[test]
    fn test_accessors() {
        let a = MtriX::default();
        assert!(!a.contained);
        assert_eq!(a.serial_number, 0);
        assert_eq!(a.transformation, TransformationMatrix::identity());
    }
}
