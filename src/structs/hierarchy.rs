//! # Atoms with containing Hierarchy
//!
//! Defines structs to contain a piece of Hierarchy within the PDB structure. It also defines mutable
//! counterparts to these to allow for mutable access to one level in the hierarchy at the same time.
//!
//! Using the traits you can write more generic functions.
//! ```rust
//! use pdbtbx::*;
//! let (mut pdb, _errors) = pdbtbx::open("example-pdbs/1ubq.pdb").unwrap();
//!
//! // Return the X Y coordinates if the conformer name is "HOH"
//! fn find_position(hierarchy: impl ContainsAtomConformer) -> Option<(f64, f64)> {
//!     if hierarchy.conformer().name() == "HOH" {
//!         Some((hierarchy.atom().x(), hierarchy.atom().y()))
//!     } else {
//!         None
//!     }
//! }
//!
//! // Translate the Y position of all atoms and return all HOH X Y coordinates
//! pdb.atoms_with_hierarchy_mut().filter_map(|mut hierarchy| {
//!     let new_y = hierarchy.atom().y() - 150.0;
//!     hierarchy.atom_mut().set_y(new_y);
//!     find_position(hierarchy)
//! });
//!
//! ```
//!
//! ## Crate supporters
//!
//! Each struct implements three important methods: `new`, `from_tuple`, and `extend`. These methods
//! allow for the construction of the structs and are designated `pub(crate)` to ensure that users
//! cannot create structs that do not follow the rule:
//!
//! > All elements of the struct should be a child of all previous levels in the hierarchy.
#![allow(clippy::unwrap_used, dead_code)]
use super::*;
use std::marker::PhantomData;

/// A struct to hold references to an Atom and its containing Conformer.
#[derive(Debug, Clone)]
pub struct AtomConformer<'a> {
    /// This Atom
    atom: &'a Atom,
    /// The Conformer containing this Atom
    conformer: &'a Conformer,
}

/// A struct to hold references to an Atom and its containing Conformer and Residue.
#[derive(Debug, Clone)]
pub struct AtomConformerResidue<'a> {
    /// This Atom
    atom: &'a Atom,
    /// The Conformer containing this Atom
    conformer: &'a Conformer,
    /// The Residue containing this Atom
    residue: &'a Residue,
}

/// A struct to hold references to an Atom and its containing Conformer, Residue, and Chain.
#[derive(Debug, Clone)]
pub struct AtomConformerResidueChain<'a> {
    /// This Atom
    atom: &'a Atom,
    /// The Conformer containing this Atom
    conformer: &'a Conformer,
    /// The Residue containing this Atom
    residue: &'a Residue,
    /// The Chain containing this Atom
    chain: &'a Chain,
}

/// A struct to hold references to an Atom and its containing Conformer, Residue, Chain, and Model.
#[derive(Debug, Clone)]
pub struct AtomConformerResidueChainModel<'a> {
    /// This Atom
    atom: &'a Atom,
    /// The Conformer containing this Atom
    conformer: &'a Conformer,
    /// The Residue containing this Atom
    residue: &'a Residue,
    /// The Chain containing this Atom
    chain: &'a Chain,
    /// The Model containing this Atom
    model: &'a Model,
}

/// A struct to hold mutable references to an Atom and its containing Conformer.
#[derive(Debug, Clone)]
pub struct AtomConformerMut<'a> {
    /// This Atom
    atom: *mut Atom,
    /// The Conformer containing this Atom
    conformer: *mut Conformer,
    phantom: PhantomData<&'a usize>,
}

/// A struct to hold mutable references to an Atom and its containing Conformer and Residue.
#[derive(Debug, Clone)]
pub struct AtomConformerResidueMut<'a> {
    /// This Atom
    atom: *mut Atom,
    /// The Conformer containing this Atom
    conformer: *mut Conformer,
    /// The Residue containing this Atom
    residue: *mut Residue,
    phantom: PhantomData<&'a usize>,
}

/// A struct to hold mutable references to an Atom and its containing Conformer, Residue, and
/// Chain.
#[derive(Debug, Clone)]
pub struct AtomConformerResidueChainMut<'a> {
    /// This Atom
    atom: *mut Atom,
    /// The Conformer containing this Atom
    conformer: *mut Conformer,
    /// The Residue containing this Atom
    residue: *mut Residue,
    /// The Chain containing this Atom
    chain: *mut Chain,
    phantom: PhantomData<&'a usize>,
}

/// A struct to hold mutable references to an Atom and its containing Conformer, Residue, Chain, and Model.
#[derive(Debug, Clone)]
pub struct AtomConformerResidueChainModelMut<'a> {
    /// This Atom
    atom: *mut Atom,
    /// The Conformer containing this Atom
    conformer: *mut Conformer,
    /// The Residue containing this Atom
    residue: *mut Residue,
    /// The Chain containing this Atom
    chain: *mut Chain,
    /// The Model containing this Atom
    model: *mut Model,
    phantom: PhantomData<&'a usize>,
}

/// A trait which defines all functions on a hierarchy which contains Atoms and Conformers.
pub trait ContainsAtomConformer {
    /// Get a reference to the atom
    fn atom(&self) -> &Atom;
    /// Get a reference to the conformer
    fn conformer(&self) -> &Conformer;
    /// Tests if this atom is part of the protein backbone
    fn is_backbone(&self) -> bool {
        self.conformer().is_amino_acid() && self.atom().is_backbone()
    }
    /// Tests if this atom is part of a side chain of an amino acid
    fn is_sidechain(&self) -> bool {
        self.conformer().is_amino_acid() && !self.atom().is_backbone()
    }
}

/// A trait which defines all functions on a hierarchy which contains Atoms, Conformers, and Residues.
pub trait ContainsAtomConformerResidue: ContainsAtomConformer {
    /// Get a reference to the residue
    fn residue(&self) -> &Residue;
}

/// A trait which defines all functions on a hierarchy which contains Atoms, Conformers, Residues, and Chains.
pub trait ContainsAtomConformerResidueChain: ContainsAtomConformerResidue {
    /// Get a reference to the chain
    fn chain(&self) -> &Chain;
}

/// A trait which defines all functions on a hierarchy which contains Atoms, Conformers, Residues, Chains, and Models.
pub trait ContainsAtomConformerResidueChainModel: ContainsAtomConformerResidueChain {
    /// Get a reference to the model
    fn model(&self) -> &Model;
}

/// A trait which defines all functions on a mutable hierarchy which contains Atoms and Conformers.
pub trait ContainsAtomConformerMut: ContainsAtomConformer {
    /// Get a mutable reference to the atom
    fn atom_mut(&mut self) -> &mut Atom;
    /// Get a mutable reference to the conformer
    fn conformer_mut(&mut self) -> &mut Conformer;
}

/// A trait which defines all functions on a mutable hierarchy which contains Atoms, Conformers, and Residues.
pub trait ContainsAtomConformerResidueMut:
    ContainsAtomConformerResidue + ContainsAtomConformerMut
{
    /// Get a mutable reference to the residue
    fn residue_mut(&mut self) -> &mut Residue;
}

/// A trait which defines all functions on a mutable hierarchy which contains Atoms, Conformers, Residues, and Chains.
pub trait ContainsAtomConformerResidueChainMut:
    ContainsAtomConformerResidueChain + ContainsAtomConformerResidueMut
{
    /// Get a mutable reference to the chain
    fn chain_mut(&mut self) -> &mut Chain;
}

/// A trait which defines all functions on a mutable hierarchy which contains Atoms, Conformers, Residues, Chains, and Models.
pub trait ContainsAtomConformerResidueChainModelMut:
    ContainsAtomConformerResidueChainModel + ContainsAtomConformerResidueChainMut
{
    /// Get a mutable reference to the model
    fn model_mut(&mut self) -> &mut Model;
}

#[cfg(feature = "rstar")]
use rstar::{PointDistance, RTreeObject, AABB};

macro_rules! impl_hierarchy {
    ($($type:ty,)*) => {
        $(#[cfg(feature = "rstar")]
        impl RTreeObject for $type {
            type Envelope = AABB<(f64, f64, f64)>;

            fn envelope(&self) -> Self::Envelope {
                self.atom().envelope()
            }
        }

        #[cfg(feature = "rstar")]
        impl PointDistance for $type {
            fn distance_2(&self, other: &(f64, f64, f64)) -> f64 {
                self.atom().distance_2(other)
            }
        }

        impl Eq for $type {}
        impl PartialEq for $type {
            fn eq(&self, other: &Self) -> bool {
                // By definition the combination of serial number and alternative location should be
                // unique across the whole PDB, this does not account for the fact that there could
                // be multiple models, but that is impossible to check anyway without Model information.
                self.atom().serial_number() == other.atom().serial_number()
                    && self.conformer().alternative_location() == other.conformer().alternative_location()
            }
        })*
    };
}

impl_hierarchy!(
    AtomConformer<'_>,
    AtomConformerMut<'_>,
    AtomConformerResidue<'_>,
    AtomConformerResidueMut<'_>,
    AtomConformerResidueChain<'_>,
    AtomConformerResidueChainMut<'_>,
    AtomConformerResidueChainModel<'_>,
    AtomConformerResidueChainModelMut<'_>,
);

// ______ AtomConformer

impl<'a> AtomConformer<'a> {
    pub(crate) const fn new(atom: &'a Atom, conformer: &'a Conformer) -> Self {
        Self { atom, conformer }
    }
    pub(crate) const fn from_tuple(tuple: (&'a Atom, &'a Conformer)) -> Self {
        Self {
            atom: tuple.0,
            conformer: tuple.1,
        }
    }
    pub(crate) const fn extend(self, residue: &'a Residue) -> AtomConformerResidue<'a> {
        AtomConformerResidue {
            atom: self.atom,
            conformer: self.conformer,
            residue,
        }
    }
}

impl ContainsAtomConformer for AtomConformer<'_> {
    fn atom(&self) -> &Atom {
        self.atom
    }
    fn conformer(&self) -> &Conformer {
        self.conformer
    }
}

// ______ AtomConformerResidue

impl<'a> AtomConformerResidue<'a> {
    pub(crate) const fn new(
        atom: &'a Atom,
        conformer: &'a Conformer,
        residue: &'a Residue,
    ) -> Self {
        Self {
            atom,
            conformer,
            residue,
        }
    }
    pub(crate) const fn from_tuple(tuple: (&'a Atom, &'a Conformer, &'a Residue)) -> Self {
        Self {
            atom: tuple.0,
            conformer: tuple.1,
            residue: tuple.2,
        }
    }
    pub(crate) const fn extend(self, chain: &'a Chain) -> AtomConformerResidueChain<'a> {
        AtomConformerResidueChain {
            atom: self.atom,
            conformer: self.conformer,
            residue: self.residue,
            chain,
        }
    }
}

impl ContainsAtomConformer for AtomConformerResidue<'_> {
    fn atom(&self) -> &Atom {
        self.atom
    }
    fn conformer(&self) -> &Conformer {
        self.conformer
    }
}

impl ContainsAtomConformerResidue for AtomConformerResidue<'_> {
    fn residue(&self) -> &Residue {
        self.residue
    }
}

// ______ AtomConformerResidueChain

impl<'a> AtomConformerResidueChain<'a> {
    pub(crate) const fn new(
        atom: &'a Atom,
        conformer: &'a Conformer,
        residue: &'a Residue,
        chain: &'a Chain,
    ) -> Self {
        Self {
            atom,
            conformer,
            residue,
            chain,
        }
    }
    pub(crate) const fn from_tuple(
        tuple: (&'a Atom, &'a Conformer, &'a Residue, &'a Chain),
    ) -> Self {
        Self {
            atom: tuple.0,
            conformer: tuple.1,
            residue: tuple.2,
            chain: tuple.3,
        }
    }
    pub(crate) const fn extend(self, model: &'a Model) -> AtomConformerResidueChainModel<'a> {
        AtomConformerResidueChainModel {
            atom: self.atom,
            conformer: self.conformer,
            residue: self.residue,
            chain: self.chain,
            model,
        }
    }
}

impl ContainsAtomConformer for AtomConformerResidueChain<'_> {
    fn atom(&self) -> &Atom {
        self.atom
    }
    fn conformer(&self) -> &Conformer {
        self.conformer
    }
}

impl ContainsAtomConformerResidue for AtomConformerResidueChain<'_> {
    fn residue(&self) -> &Residue {
        self.residue
    }
}

impl ContainsAtomConformerResidueChain for AtomConformerResidueChain<'_> {
    fn chain(&self) -> &Chain {
        self.chain
    }
}

// ______ AtomConformerResidueChainModel

impl<'a> AtomConformerResidueChainModel<'a> {
    pub(crate) const fn new(
        atom: &'a Atom,
        conformer: &'a Conformer,
        residue: &'a Residue,
        chain: &'a Chain,
        model: &'a Model,
    ) -> Self {
        Self {
            atom,
            conformer,
            residue,
            chain,
            model,
        }
    }
    pub(crate) const fn from_tuple(
        tuple: (&'a Atom, &'a Conformer, &'a Residue, &'a Chain, &'a Model),
    ) -> Self {
        Self {
            atom: tuple.0,
            conformer: tuple.1,
            residue: tuple.2,
            chain: tuple.3,
            model: tuple.4,
        }
    }
}

impl ContainsAtomConformer for AtomConformerResidueChainModel<'_> {
    fn atom(&self) -> &Atom {
        self.atom
    }
    fn conformer(&self) -> &Conformer {
        self.conformer
    }
}

impl ContainsAtomConformerResidue for AtomConformerResidueChainModel<'_> {
    fn residue(&self) -> &Residue {
        self.residue
    }
}

impl ContainsAtomConformerResidueChain for AtomConformerResidueChainModel<'_> {
    fn chain(&self) -> &Chain {
        self.chain
    }
}

impl ContainsAtomConformerResidueChainModel for AtomConformerResidueChainModel<'_> {
    fn model(&self) -> &Model {
        self.model
    }
}

// ______ AtomConformerMut

impl<'a> AtomConformerMut<'a> {
    pub(crate) const fn new(atom: *mut Atom, conformer: *mut Conformer) -> Self {
        Self {
            atom,
            conformer,
            phantom: PhantomData,
        }
    }
    pub(crate) const fn from_tuple(tuple: (*mut Atom, *mut Conformer)) -> Self {
        Self {
            atom: tuple.0,
            conformer: tuple.1,
            phantom: PhantomData,
        }
    }
    pub(crate) const fn extend(self, residue: *mut Residue) -> AtomConformerResidueMut<'a> {
        AtomConformerResidueMut {
            atom: self.atom,
            conformer: self.conformer,
            residue,
            phantom: PhantomData,
        }
    }
    /// Change this mutable hierarchy into an immutable hierarchy
    pub fn without_mut(self) -> AtomConformer<'a> {
        unsafe {
            AtomConformer {
                atom: self.atom.as_ref().unwrap(),
                conformer: self.conformer.as_ref().unwrap(),
            }
        }
    }
}

impl ContainsAtomConformer for AtomConformerMut<'_> {
    fn atom(&self) -> &Atom {
        unsafe { self.atom.as_ref().unwrap() }
    }
    fn conformer(&self) -> &Conformer {
        unsafe { self.conformer.as_ref().unwrap() }
    }
}

impl ContainsAtomConformerMut for AtomConformerMut<'_> {
    fn atom_mut(&mut self) -> &mut Atom {
        unsafe { self.atom.as_mut().unwrap() }
    }
    fn conformer_mut(&mut self) -> &mut Conformer {
        unsafe { self.conformer.as_mut().unwrap() }
    }
}

// ______ AtomConformerResidueMut

impl<'a> AtomConformerResidueMut<'a> {
    pub(crate) const fn new(
        atom: *mut Atom,
        conformer: *mut Conformer,
        residue: *mut Residue,
    ) -> Self {
        Self {
            atom,
            conformer,
            residue,
            phantom: PhantomData,
        }
    }
    pub(crate) const fn from_tuple(tuple: (*mut Atom, *mut Conformer, *mut Residue)) -> Self {
        Self {
            atom: tuple.0,
            conformer: tuple.1,
            residue: tuple.2,
            phantom: PhantomData,
        }
    }
    pub(crate) const fn extend(self, chain: *mut Chain) -> AtomConformerResidueChainMut<'a> {
        AtomConformerResidueChainMut {
            atom: self.atom,
            conformer: self.conformer,
            residue: self.residue,
            chain,
            phantom: PhantomData,
        }
    }
    /// Change this mutable hierarchy into an immutable hierarchy
    pub fn without_mut(self) -> AtomConformerResidue<'a> {
        unsafe {
            AtomConformerResidue {
                atom: self.atom.as_ref().unwrap(),
                conformer: self.conformer.as_ref().unwrap(),
                residue: self.residue.as_ref().unwrap(),
            }
        }
    }
}

impl ContainsAtomConformer for AtomConformerResidueMut<'_> {
    fn atom(&self) -> &Atom {
        unsafe { self.atom.as_ref().unwrap() }
    }
    fn conformer(&self) -> &Conformer {
        unsafe { self.conformer.as_ref().unwrap() }
    }
}

impl ContainsAtomConformerMut for AtomConformerResidueMut<'_> {
    fn atom_mut(&mut self) -> &mut Atom {
        unsafe { self.atom.as_mut().unwrap() }
    }
    fn conformer_mut(&mut self) -> &mut Conformer {
        unsafe { self.conformer.as_mut().unwrap() }
    }
}

impl ContainsAtomConformerResidue for AtomConformerResidueMut<'_> {
    fn residue(&self) -> &Residue {
        unsafe { self.residue.as_ref().unwrap() }
    }
}

impl ContainsAtomConformerResidueMut for AtomConformerResidueMut<'_> {
    fn residue_mut(&mut self) -> &mut Residue {
        unsafe { self.residue.as_mut().unwrap() }
    }
}

// ______ AtomConformerResidueChainMut

impl<'a> AtomConformerResidueChainMut<'a> {
    pub(crate) const fn new(
        atom: *mut Atom,
        conformer: *mut Conformer,
        residue: *mut Residue,
        chain: *mut Chain,
    ) -> Self {
        Self {
            atom,
            conformer,
            residue,
            chain,
            phantom: PhantomData,
        }
    }
    pub(crate) const fn from_tuple(
        tuple: (*mut Atom, *mut Conformer, *mut Residue, *mut Chain),
    ) -> Self {
        Self {
            atom: tuple.0,
            conformer: tuple.1,
            residue: tuple.2,
            chain: tuple.3,
            phantom: PhantomData,
        }
    }
    pub(crate) const fn extend(self, model: *mut Model) -> AtomConformerResidueChainModelMut<'a> {
        AtomConformerResidueChainModelMut {
            atom: self.atom,
            conformer: self.conformer,
            residue: self.residue,
            chain: self.chain,
            model,
            phantom: PhantomData,
        }
    }
    /// Change this mutable hierarchy into an immutable hierarchy
    pub fn without_mut(self) -> AtomConformerResidueChain<'a> {
        unsafe {
            AtomConformerResidueChain {
                atom: self.atom.as_ref().unwrap(),
                conformer: self.conformer.as_ref().unwrap(),
                residue: self.residue.as_ref().unwrap(),
                chain: self.chain.as_ref().unwrap(),
            }
        }
    }
}

impl ContainsAtomConformer for AtomConformerResidueChainMut<'_> {
    fn atom(&self) -> &Atom {
        unsafe { self.atom.as_ref().unwrap() }
    }
    fn conformer(&self) -> &Conformer {
        unsafe { self.conformer.as_ref().unwrap() }
    }
}

impl ContainsAtomConformerMut for AtomConformerResidueChainMut<'_> {
    fn atom_mut(&mut self) -> &mut Atom {
        unsafe { self.atom.as_mut().unwrap() }
    }
    fn conformer_mut(&mut self) -> &mut Conformer {
        unsafe { self.conformer.as_mut().unwrap() }
    }
}

impl ContainsAtomConformerResidue for AtomConformerResidueChainMut<'_> {
    fn residue(&self) -> &Residue {
        unsafe { self.residue.as_ref().unwrap() }
    }
}

impl ContainsAtomConformerResidueMut for AtomConformerResidueChainMut<'_> {
    fn residue_mut(&mut self) -> &mut Residue {
        unsafe { self.residue.as_mut().unwrap() }
    }
}

impl ContainsAtomConformerResidueChain for AtomConformerResidueChainMut<'_> {
    fn chain(&self) -> &Chain {
        unsafe { self.chain.as_ref().unwrap() }
    }
}

impl ContainsAtomConformerResidueChainMut for AtomConformerResidueChainMut<'_> {
    fn chain_mut(&mut self) -> &mut Chain {
        unsafe { self.chain.as_mut().unwrap() }
    }
}

// ______ AtomConformerResidueChainModelMut

impl<'a> AtomConformerResidueChainModelMut<'a> {
    pub(crate) const fn new(
        atom: *mut Atom,
        conformer: *mut Conformer,
        residue: *mut Residue,
        chain: *mut Chain,
        model: *mut Model,
    ) -> Self {
        Self {
            atom,
            conformer,
            residue,
            chain,
            model,
            phantom: PhantomData,
        }
    }
    pub(crate) const fn from_tuple(
        tuple: (
            *mut Atom,
            *mut Conformer,
            *mut Residue,
            *mut Chain,
            *mut Model,
        ),
    ) -> Self {
        Self {
            atom: tuple.0,
            conformer: tuple.1,
            residue: tuple.2,
            chain: tuple.3,
            model: tuple.4,
            phantom: PhantomData,
        }
    }
    /// Change this mutable hierarchy into an immutable hierarchy
    pub fn without_mut(self) -> AtomConformerResidueChainModel<'a> {
        unsafe {
            AtomConformerResidueChainModel {
                atom: self.atom.as_ref().unwrap(),
                conformer: self.conformer.as_ref().unwrap(),
                residue: self.residue.as_ref().unwrap(),
                chain: self.chain.as_ref().unwrap(),
                model: self.model.as_ref().unwrap(),
            }
        }
    }
}

impl ContainsAtomConformer for AtomConformerResidueChainModelMut<'_> {
    fn atom(&self) -> &Atom {
        unsafe { self.atom.as_ref().unwrap() }
    }
    fn conformer(&self) -> &Conformer {
        unsafe { self.conformer.as_ref().unwrap() }
    }
}

impl ContainsAtomConformerMut for AtomConformerResidueChainModelMut<'_> {
    fn atom_mut(&mut self) -> &mut Atom {
        unsafe { self.atom.as_mut().unwrap() }
    }
    fn conformer_mut(&mut self) -> &mut Conformer {
        unsafe { self.conformer.as_mut().unwrap() }
    }
}

impl ContainsAtomConformerResidue for AtomConformerResidueChainModelMut<'_> {
    fn residue(&self) -> &Residue {
        unsafe { self.residue.as_ref().unwrap() }
    }
}

impl ContainsAtomConformerResidueMut for AtomConformerResidueChainModelMut<'_> {
    fn residue_mut(&mut self) -> &mut Residue {
        unsafe { self.residue.as_mut().unwrap() }
    }
}

impl ContainsAtomConformerResidueChain for AtomConformerResidueChainModelMut<'_> {
    fn chain(&self) -> &Chain {
        unsafe { self.chain.as_ref().unwrap() }
    }
}

impl ContainsAtomConformerResidueChainMut for AtomConformerResidueChainModelMut<'_> {
    fn chain_mut(&mut self) -> &mut Chain {
        unsafe { self.chain.as_mut().unwrap() }
    }
}

impl ContainsAtomConformerResidueChainModel for AtomConformerResidueChainModelMut<'_> {
    fn model(&self) -> &Model {
        unsafe { self.model.as_ref().unwrap() }
    }
}

impl ContainsAtomConformerResidueChainModelMut for AtomConformerResidueChainModelMut<'_> {
    fn model_mut(&mut self) -> &mut Model {
        unsafe { self.model.as_mut().unwrap() }
    }
}
