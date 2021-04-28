use super::*;
use rstar::{PointDistance, RTreeObject, AABB};

/// A structure containing references to the full hierarchy for a single atom
#[derive(Debug, Clone)]
pub struct AtomWithHierarchy<'a> {
    /// The Chain containing this Atom
    pub chain: &'a Chain,
    /// The Residue containing this Atom
    pub residue: &'a Residue,
    /// The Conformer containing this Atom
    pub conformer: &'a Conformer,
    /// This Atom
    pub atom: &'a Atom,
}

impl<'a> AtomWithHierarchy<'a> {
    /// Create an AtomWithHierarchy from a Tuple containing all needed references
    pub fn from_tuple(
        hierarchy: (&'a Chain, &'a Residue, &'a Conformer, &'a Atom),
    ) -> AtomWithHierarchy<'a> {
        AtomWithHierarchy {
            chain: hierarchy.0,
            residue: hierarchy.1,
            conformer: hierarchy.2,
            atom: hierarchy.3,
        }
    }
    /// Create an AtomWithHierarchy from all needed references
    pub fn new(
        chain: &'a Chain,
        residue: &'a Residue,
        conformer: &'a Conformer,
        atom: &'a Atom,
    ) -> AtomWithHierarchy<'a> {
        AtomWithHierarchy {
            chain,
            residue,
            conformer,
            atom,
        }
    }

    /// Tests if this atom is part of the protein backbone
    pub fn backbone(&self) -> bool {
        self.conformer.amino_acid() && self.atom.backbone()
    }

    /// Tests if this atom is part of a side chain of an amino acid
    pub fn side_chain(&self) -> bool {
        self.conformer.amino_acid() && !self.atom.hetero()
    }
}

impl<'a> RTreeObject for AtomWithHierarchy<'a> {
    type Envelope = AABB<[f64; 3]>;

    fn envelope(&self) -> Self::Envelope {
        self.atom.envelope()
    }
}

impl<'a> PointDistance for AtomWithHierarchy<'a> {
    fn distance_2(&self, other: &[f64; 3]) -> f64 {
        self.atom.distance_2(other)
    }
}
