#![allow(dead_code)]
use super::unit_cell::UnitCell;
use crate::reference_tables;
use crate::transformation::TransformationMatrix;
use std::cmp::Ordering;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
/// A Space group of a crystal
pub struct Symmetry {
    /// The index of this symbol in Int. Crys. Handbook Vol A 2016
    index: usize,
}

impl Symmetry {
    /// Create a new `Symmetry` based on a fully qualified Herman Mauguin or Hall symbol
    #[must_use]
    pub fn new(symbol: impl AsRef<str>) -> Option<Self> {
        reference_tables::get_index_for_symbol(symbol.as_ref().trim()).map(|index| Self { index })
    }

    /// Create a new `Symmetry` based on the index of a symbol in Int. Crys. Handbook Vol A 2016
    #[must_use]
    pub fn from_index(index: usize) -> Option<Self> {
        reference_tables::get_herman_mauguin_symbol_for_index(index).map(|_| Self { index })
    }

    /// Get the fully qualified Herman Mauguin symbol for the space group
    #[must_use]
    pub fn herman_mauguin_symbol(&self) -> &str {
        reference_tables::get_herman_mauguin_symbol_for_index(self.index)
            .expect("An invalid index was present in the definition of this symmetry")
    }

    /// Get the fully qualified Hall symbol for the space group
    #[must_use]
    pub fn hall_symbol(&self) -> &str {
        reference_tables::get_hall_symbol_for_index(self.index)
            .expect("An invalid index was present in the definition of this symmetry")
    }

    /// Get the Z value, the number of polymeric sub units in a unit cell, for this space group
    #[allow(clippy::unwrap_used)]
    #[must_use]
    pub fn z(&self) -> usize {
        reference_tables::get_transformation(self.index)
            .unwrap()
            .len()
            + 1
    }

    /// Get the index of this space group in Int. Crys. Handbook Vol A 2016
    #[must_use]
    pub const fn index(&self) -> usize {
        self.index
    }

    /// Get the transformations for this space group needed to fill the unit cell.
    /// The first transformation is always an identity transformation.
    /// The translation is fractional to the unit cell size.
    #[allow(clippy::unwrap_used)]
    #[must_use]
    pub fn transformations(&self) -> Vec<TransformationMatrix> {
        let matrices = reference_tables::get_transformation(self.index).unwrap();
        let mut output = Vec::with_capacity(matrices.len() + 1);
        output.push(TransformationMatrix::identity());
        for matrix in matrices {
            output.push(TransformationMatrix::from_matrix(*matrix));
        }
        output
    }

    /// Get the transformations for this space group needed to fill the unit cell.
    /// The first transformation is always an identity transformation.
    /// The translation is in Å.
    #[allow(clippy::unwrap_used)]
    #[must_use]
    pub fn transformations_absolute(&self, unit_cell: &UnitCell) -> Vec<TransformationMatrix> {
        let matrices = reference_tables::get_transformation(self.index).unwrap();
        let mut output = Vec::with_capacity(matrices.len() + 1);
        output.push(TransformationMatrix::identity());
        for matrix in matrices {
            let mut ma = TransformationMatrix::from_matrix(*matrix);
            ma.multiply_translation(unit_cell.size());
            output.push(ma);
        }
        output
    }
}

impl PartialEq for Symmetry {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for Symmetry {}

impl Ord for Symmetry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

impl PartialOrd for Symmetry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::Symmetry;

    #[test]
    #[allow(clippy::unwrap_used)]
    fn both_creations() {
        let a = Symmetry::new("P 21 21 21").unwrap();
        let b = Symmetry::from_index(19).unwrap();
        assert_eq!(a, b);
        assert_eq!(a.z(), a.transformations().len());
        assert_eq!(
            4,
            a.transformations_absolute(&crate::UnitCell::new(1.0, 1.0, 1.0, 90.0, 90.0, 90.0))
                .len()
        );
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn symbol_invariant() {
        let a = Symmetry::new("P 21 21 21").unwrap();
        assert_eq!(a.herman_mauguin_symbol(), "P 21 21 21");
        assert_eq!(a.hall_symbol(), "P 2ac 2ab");
        assert_eq!(a.index(), 19);
    }
}
