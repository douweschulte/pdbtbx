#![allow(dead_code)]
use crate::reference_tables;
use crate::transformation::*;

#[derive(Debug)]
/// A Space group of a crystal
pub struct Symmetry {
    /// The fully qualified Herman Mauguin symbol for the space group
    symbol: String,
    /// The index of this symbol in Int. Crys. Handbook Vol A 2016
    index: usize,
}

impl Symmetry {
    /// Create a new Symmetry based on a fully qualified Herman Mauguin symbol
    pub fn new(symbol: &str) -> Option<Self> {
        match reference_tables::get_index_for_symbol(symbol.trim()) {
            Some(i) => Some(Symmetry {
                symbol: symbol.trim().to_string(),
                index: i,
            }),
            None => return None,
        }
    }

    /// Create a new Symmetry based on the index of a symbol in Int. Crys. Handbook Vol A 2016
    pub fn from_index(index: usize) -> Option<Self> {
        match reference_tables::get_symbol_for_index(index) {
            Some(s) => Some(Symmetry {
                symbol: s.to_string(),
                index: index,
            }),
            None => return None,
        }
    }

    /// Get the fully qualified Herman Mauguin symbol for the space group
    pub fn symbol(&self) -> &str {
        self.symbol.as_str()
    }

    /// Get the Z value, the number of polymeric sub units in a unit cell, for this space group
    pub fn z(&self) -> usize {
        reference_tables::get_transformation(self.index)
            .unwrap()
            .len()
    }

    /// Get the index of this space group in Int. Crys. Handbook Vol A 2016
    pub fn index(&self) -> usize {
        self.index
    }

    /// Get the transformations for this space group needed to fill the unit cell
    /// The first transformation is always an identity transformation
    pub fn transformations(&self) -> Vec<TransformationMatrix> {
        let matrices = reference_tables::get_transformation(self.index).unwrap();
        let mut output = Vec::with_capacity(matrices.len() + 1);
        output.push(TransformationMatrix::identity());
        for matrix in matrices {
            output.push(TransformationMatrix::from_matrix(matrix.clone()));
        }
        output
    }
}

impl Clone for Symmetry {
    fn clone(&self) -> Self {
        Symmetry {
            symbol: self.symbol.clone(),
            index: self.index,
        }
    }
}

impl PartialEq for Symmetry {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}
impl Eq for Symmetry {}

#[cfg(test)]
mod tests {
    use super::Symmetry;

    #[test]
    fn both_creations() {
        let a = Symmetry::new("P 21 21 21");
        let b = Symmetry::from_index(19);
        assert_eq!(a, b);
    }

    #[test]
    fn symbol_invariant() {
        let a = Symmetry::new("P 21 21 21").unwrap();
        assert_eq!(a.symbol(), "P 21 21 21")
    }
}
