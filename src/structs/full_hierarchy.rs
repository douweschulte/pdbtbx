use super::*;
use rstar::{PointDistance, RTreeObject, AABB};

/// A structure containing references to the full hierarchy for a single atom
#[derive(Debug)]
pub struct FullHierarchy<'a> {
    /// The Chain containing this Atom
    pub chain: &'a Chain,
    /// The Residue containing this Atom
    pub residue: &'a Residue,
    /// The Conformer containing this Atom
    pub conformer: &'a Conformer,
    /// This Atom
    pub atom: &'a Atom,
}

impl<'a> FullHierarchy<'a> {
    /// Create a FullHierarchy from a Tuple containing all needed references
    pub fn from_tuple(
        hierarchy: (&'a Chain, &'a Residue, &'a Conformer, &'a Atom),
    ) -> FullHierarchy<'a> {
        FullHierarchy {
            chain: hierarchy.0,
            residue: hierarchy.1,
            conformer: hierarchy.2,
            atom: hierarchy.3,
        }
    }
    /// Create a FullHierarchy from all needed references
    pub fn new(
        chain: &'a Chain,
        residue: &'a Residue,
        conformer: &'a Conformer,
        atom: &'a Atom,
    ) -> FullHierarchy<'a> {
        FullHierarchy {
            chain,
            residue,
            conformer,
            atom,
        }
    }
}

impl<'a> RTreeObject for FullHierarchy<'a> {
    type Envelope = AABB<[f64; 3]>;

    fn envelope(&self) -> Self::Envelope {
        self.atom.envelope()
    }
}

impl<'a> PointDistance for FullHierarchy<'a> {
    fn distance_2(&self, other: &[f64; 3]) -> f64 {
        self.atom.distance_2(other)
    }
}
