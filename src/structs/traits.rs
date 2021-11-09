use crate::structs::*;
use crate::transformation::*;
use doc_cfg::doc_cfg;
#[cfg(feature = "rayon")]
use rayon::prelude::*;
use std::fmt::Display;

/// All functionality expected for a struct that contains multiple Atoms.
pub trait ContainsAtoms<'a>: PartialOrd + Ord + Display {
    type ParallelAtom: ParallelIterator<Item = &'a Atom>;
    type ParallelAtomMut: ParallelIterator<Item = &'a mut Atom>;
    /// The amount of atoms making up this struct
    fn atom_count(&self) -> usize;
    /// Get a specific atom from list of atoms making up this struct.
    ///
    /// ## Arguments
    /// * `index` - the index of the atom
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    fn atom(&self, index: usize) -> Option<&Atom>;
    /// Get a specific atom as a mutable reference from list of atoms making up this struct.
    ///
    /// ## Arguments
    /// * `index` - the index of the atom
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    fn atom_mut(&mut self, index: usize) -> Option<&mut Atom>;
    /// Get the list of atoms making up this struct.
    /// Double ended so iterating from the end is just as fast as from the start.
    fn atoms(&self) -> Box<dyn DoubleEndedIterator<Item = &Atom> + '_>;
    /// Get the list of atoms as mutable references making up this struct.
    /// Double ended so iterating from the end is just as fast as from the start.
    fn atoms_mut(&mut self) -> Box<dyn DoubleEndedIterator<Item = &mut Atom> + '_>;
    /// Get the list of atoms making up this struct in parallel.
    #[doc_cfg(feature = "rayon")]
    fn par_atoms(&'a self) -> Self::ParallelAtom;
    /// Get the list of atoms as mutable references making up this struct in parallel.
    #[doc_cfg(feature = "rayon")]
    fn par_atoms_mut(&'a mut self) -> Self::ParallelAtomMut;
    /// Remove all Atoms matching the given predicate. As this is done in place this is the fastest way to remove Atoms from this struct.
    fn remove_atoms_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Atom) -> bool;
    /// Remove the Atom specified.
    /// It removes all matching Atoms from the list.
    ///
    /// ## Arguments
    /// * `serial_number` - the serial number of the Atom to remove
    fn remove_atom_by_serial_number(&mut self, serial_number: usize) {
        self.remove_atoms_by(|a| a.serial_number() != serial_number)
    }
    /// Remove the Atom specified.
    /// It removes all matching Atoms from the list.
    ///
    /// ## Arguments
    /// * `name` - the name of the Atom to remove
    fn remove_atom_by_name(&mut self, name: String) {
        self.remove_atoms_by(|a| a.name() != name)
    }
    /// Apply a transformation to the position of all atoms making up this struct, the new position is immediately set.
    fn apply_transformation(&mut self, transformation: &TransformationMatrix) {
        for atom in self.atoms_mut() {
            atom.apply_transformation(transformation);
        }
    }
    /// Apply a transformation to the position of all atoms making up this struct, the new position is immediately set.
    /// This is done in parallel.
    #[doc_cfg(feature = "rayon")]
    fn par_apply_transformation(&'a mut self, transformation: &TransformationMatrix) {
        self.par_atoms_mut()
            .for_each(|a| a.apply_transformation(transformation))
    }
    /// Sort the Atoms of this struct
    fn sort(&mut self);
    /// Sort the Atoms of this struct in parallel
    #[doc_cfg(feature = "rayon")]
    fn par_sort(&mut self);
}

pub trait ContainsConformers<'a> {
    type ParallelConformer: ParallelIterator<Item = &'a Conformer>;
    type ParallelConformerMut: ParallelIterator<Item = &'a mut Conformer>;

    /// The amount of Conformers making up this Residue
    fn conformer_count(&self) -> usize;

    /// Get a specific conformer from list of conformers making up this Residue.
    ///
    /// ## Arguments
    /// * `index` - the index of the conformer
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    fn conformer(&self, index: usize) -> Option<&Conformer>;

    /// Get a specific conformer as a mutable reference from list of conformers making up this Residue.
    ///
    /// ## Arguments
    /// * `index` - the index of the conformer
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    fn conformer_mut(&mut self, index: usize) -> Option<&mut Conformer>;
    /// Get the list of conformers making up this Residue.
    /// Double ended so iterating from the end is just as fast as from the start.
    fn conformers(&self) -> Box<dyn DoubleEndedIterator<Item = &Conformer> + '_>;

    /// Get the list of conformers making up this Residue in parallel.
    #[doc_cfg(feature = "rayon")]
    fn par_conformers(&'a self) -> Self::ParallelConformer;

    /// Get the list of conformers as mutable references making up this Residue.
    /// Double ended so iterating from the end is just as fast as from the start.
    fn conformers_mut(&mut self) -> Box<dyn DoubleEndedIterator<Item = &mut Conformer> + '_>;

    /// Get the list of conformers as mutable references making up this Residue in parallel.
    #[doc_cfg(feature = "rayon")]
    fn par_conformers_mut(&'a mut self) -> Self::ParallelConformerMut;

    /// Remove all conformers matching the given predicate. As this is done in place this is the fastest way to remove conformers from this Residue.
    fn remove_conformers_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Conformer) -> bool;

    /// Remove the conformer specified. It returns `true` if it found a matching conformer and removed it.
    /// It removes the first matching conformer from the list.
    ///
    /// ## Arguments
    /// * `id` - the identifying construct of the Conformer to remove
    fn remove_conformer_by_id(&mut self, id: (&str, Option<&str>)) {
        self.remove_conformers_by(|c| c.id() == id)
    }
}
