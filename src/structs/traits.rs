use crate::structs::*;
use crate::transformation::*;
use doc_cfg::doc_cfg;
#[cfg(feature = "rayon")]
use rayon::prelude::*;

/// All functionality expected for a struct that contains multiple Atoms.
pub trait ContainsAtoms {
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
    fn par_atoms(&self) -> rayon::slice::Iter<'_, Atom>;
    /// Get the list of atoms as mutable references making up this struct in parallel.
    #[doc_cfg(feature = "rayon")]
    fn par_atoms_mut(&mut self) -> rayon::slice::IterMut<'_, Atom>;
    /// Remove all Atoms matching the given predicate. As this is done in place this is the fastest way to remove Atoms from this struct.
    fn remove_atoms_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Atom) -> bool;
    /// Remove the Atom specified.
    ///
    /// ## Arguments
    /// * `index` - the index of the atom to remove
    ///
    /// ## Panics
    /// It panics when the index is outside bounds.
    fn remove_atom(&mut self, index: usize);
    /// Remove the Atom specified. It returns `true` if it found a matching Atom and removed it.
    /// It removes the first matching Atom from the list.
    ///
    /// ## Arguments
    /// * `serial_number` - the serial number of the Atom to remove
    ///
    /// ## Panics
    /// It panics when the index is outside bounds.
    fn remove_atom_by_serial_number(&mut self, serial_number: usize) -> bool {
        let index = self
            .atoms()
            .position(|a| a.serial_number() == serial_number);

        if let Some(i) = index {
            self.remove_atom(i);
            true
        } else {
            false
        }
    }
    /// Remove the Atom specified. It returns `true` if it found a matching Atom and removed it.
    /// It removes the first matching Atom from the list.
    ///
    /// ## Arguments
    /// * `name` - the name of the Atom to remove
    ///
    /// ## Panics
    /// It panics when the index is outside bounds.
    fn remove_atom_by_name(&mut self, name: String) -> bool {
        let index = self.atoms().position(|a| a.name() == name);

        if let Some(i) = index {
            self.remove_atom(i);
            true
        } else {
            false
        }
    }
    /// Remove the Atom specified. It returns `true` if it found a matching Atom and removed it.
    /// It removes the first matching Atom from the list. Matching is done in parallel.
    ///
    /// ## Arguments
    /// * `serial_number` - the serial number of the Atom to remove
    ///
    /// ## Panics
    /// It panics when the index is outside bounds.
    #[doc_cfg(feature = "rayon")]
    fn par_remove_atom_by_serial_number(&mut self, serial_number: usize) -> bool {
        let index = self
            .par_atoms()
            .position_first(|a| a.serial_number() == serial_number);

        if let Some(i) = index {
            self.remove_atom(i);
            true
        } else {
            false
        }
    }
    /// Remove the Atom specified. It returns `true` if it found a matching Atom and removed it.
    /// It removes the first matching Atom from the list. Matching is done in parallel.
    ///
    /// ## Arguments
    /// * `name` - the name of the Atom to remove
    ///
    /// ## Panics
    /// It panics when the index is outside bounds.
    #[doc_cfg(feature = "rayon")]
    fn par_remove_atom_by_name(&mut self, name: String) -> bool {
        let index = self.par_atoms().position_first(|a| a.name() == name);

        if let Some(i) = index {
            self.remove_atom(i);
            true
        } else {
            false
        }
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
    fn par_apply_transformation(&mut self, transformation: &TransformationMatrix) {
        self.par_atoms_mut()
            .for_each(|a| a.apply_transformation(transformation))
    }
    /// Sort the Atoms of this struct
    fn sort(&mut self);
    /// Sort the Atoms of this struct in parallel
    #[doc_cfg(feature = "rayon")]
    fn par_sort(&mut self);
}
