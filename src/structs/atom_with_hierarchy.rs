use super::*;
use std::marker::PhantomData;

/// A structure containing references to the full hierarchy for a single Atom
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
    pub fn is_backbone(&self) -> bool {
        self.conformer.is_amino_acid() && self.atom.is_backbone()
    }

    /// Tests if this atom is part of a side chain of an amino acid
    pub fn is_side_chain(&self) -> bool {
        self.conformer.is_amino_acid() && !self.atom.hetero()
    }
}

impl<'a> Eq for AtomWithHierarchy<'a> {}

impl<'a> PartialEq for AtomWithHierarchy<'a> {
    fn eq(&self, other: &Self) -> bool {
        // By definition the combination of serial number and alternative location should be
        // unique across the whole PDB, this does not account for the fact that there could
        // be multiple models, but that is impossible to check anyway without Model information.
        self.atom.serial_number() == other.atom.serial_number()
            && self.conformer.alternative_location() == other.conformer.alternative_location()
    }
}

#[derive(Debug)]
/// A version of the AtomWithHierarchy that allows mutable access to its members in a safe way
pub struct AtomWithHierarchyMut<'a> {
    /// The Chain containing this Atom
    chain: *mut Chain,
    /// The Residue containing this Atom
    residue: *mut Residue,
    /// The Conformer containing this Atom
    conformer: *mut Conformer,
    /// This Atom
    atom: *mut Atom,
    phantom: PhantomData<&'a usize>,
}

impl<'a, 'b> AtomWithHierarchyMut<'a> {
    pub(crate) fn new(
        chain: *mut Chain,
        residue: *mut Residue,
        conformer: *mut Conformer,
        atom: *mut Atom,
    ) -> Self {
        AtomWithHierarchyMut {
            chain,
            residue,
            conformer,
            atom,
            phantom: PhantomData,
        }
    }

    /// Create an AtomWithHierarchy from a Tuple containing all needed references
    pub fn from_tuple(
        hierarchy: (*mut Chain, *mut Residue, *mut Conformer, *mut Atom),
    ) -> AtomWithHierarchyMut<'a> {
        AtomWithHierarchyMut {
            chain: hierarchy.0,
            residue: hierarchy.1,
            conformer: hierarchy.2,
            atom: hierarchy.3,
            phantom: PhantomData,
        }
    }

    #[allow(clippy::unwrap_used)]
    /// Get a reference to the chain
    pub fn chain(&'b self) -> &'b Chain {
        unsafe { self.chain.as_ref().unwrap() }
    }

    #[allow(clippy::unwrap_used)]
    /// Get the chain with a mutable reference
    pub fn chain_mut(&'b mut self) -> &'b mut Chain {
        unsafe { self.chain.as_mut().unwrap() }
    }

    #[allow(clippy::unwrap_used)]
    /// Get a reference to the residue
    pub fn residue(&'b self) -> &'b Residue {
        unsafe { self.residue.as_ref().unwrap() }
    }

    #[allow(clippy::unwrap_used)]
    /// Get the residue with a mutable reference
    pub fn residue_mut(&'b mut self) -> &'b mut Residue {
        unsafe { self.residue.as_mut().unwrap() }
    }
}

#[cfg(feature = "rstar")]
use rstar::{PointDistance, RTreeObject, AABB};

#[cfg(feature = "rstar")]
impl<'a> RTreeObject for AtomWithHierarchy<'a> {
    type Envelope = AABB<[f64; 3]>;

    fn envelope(&self) -> Self::Envelope {
        self.atom.envelope()
    }
}

#[cfg(feature = "rstar")]
impl<'a> PointDistance for AtomWithHierarchy<'a> {
    fn distance_2(&self, other: &[f64; 3]) -> f64 {
        self.atom.distance_2(other)
    }
}
