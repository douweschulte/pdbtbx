#![allow(dead_code)]
use crate::reference_tables;
use crate::structs::*;
use crate::transformation::*;
use doc_cfg::doc_cfg;
#[cfg(feature = "rayon")]
use rayon::prelude::*;
use std::cmp::Ordering;
use std::fmt;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
/// A Conformer of a Conformer containing multiple atoms, analogous to 'atom_group' in cctbx
pub struct Conformer {
    /// The name of this Conformer
    name: String,
    /// The alternative location of this Conformer, None is blank
    alternative_location: Option<String>,
    /// The list of atoms making up this Conformer
    atoms: Vec<Atom>,
    /// The modification, if present
    modification: Option<(String, String)>,
}

impl Conformer {
    /// Create a new Conformer
    ///
    /// ## Arguments
    /// * `name` - the name
    /// * `alt_loc` - the alternative location identifier, if not blank
    /// * `atom` - if available it can already add an atom
    ///
    /// ## Fails
    /// It fails if any of the characters making up the name are invalid.
    pub fn new(name: &str, alt_loc: Option<&str>, atom: Option<Atom>) -> Option<Conformer> {
        if let Some(n) = prepare_identifier(name) {
            let mut res = Conformer {
                name: n,
                alternative_location: None,
                atoms: Vec::new(),
                modification: None,
            };
            if let Some(al) = alt_loc {
                res.alternative_location = prepare_identifier(al);
            }
            if let Some(a) = atom {
                res.atoms.push(a);
            }
            Some(res)
        } else {
            None
        }
    }

    /// The name of the Conformer
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the name of the Conformer
    ///
    /// ## Fails
    /// It fails if any of the characters of the new name are invalid.
    pub fn set_name(&mut self, new_name: &str) -> bool {
        if let Some(n) = prepare_identifier(new_name) {
            self.name = n;
            true
        } else {
            false
        }
    }

    /// The alternative location of the Conformer
    pub fn alternative_location(&self) -> Option<&str> {
        self.alternative_location.as_deref()
    }

    /// Set the alternative location of the Conformer
    ///
    /// ## Fails
    /// It fails if any of the characters of the new alternative location are invalid.
    pub fn set_alternative_location(&mut self, new_loc: &str) -> bool {
        if let Some(l) = prepare_identifier(new_loc) {
            self.alternative_location = Some(l);
            true
        } else {
            false
        }
    }

    /// Set the alternative location of the Conformer to None
    pub fn remove_alternative_location(&mut self) {
        self.alternative_location = None;
    }

    /// Returns the uniquely identifying construct for this Conformer.
    /// It consists of the name and the alternative location.
    pub fn id(&self) -> (&str, Option<&str>) {
        (&self.name, self.alternative_location())
    }

    /// Get the modification of this Conformer e.g., chemical or post-translational. These will be saved in the MODRES records in the PDB file
    pub fn modification(&self) -> Option<&(String, String)> {
        self.modification.as_ref()
    }

    /// Set the modification of this Conformer e.g., chemical or post-translational. These will be saved in the MODRES records in the PDB file
    pub fn set_modification(&mut self, new_modification: (String, String)) -> Result<(), String> {
        if !valid_identifier(&new_modification.0) {
            Err(format!(
                "New modification has invalid characters for standard conformer name, conformer: {:?}, standard name \"{}\"",
                self.id(), new_modification.0
            ))
        } else if !valid_text(&new_modification.1) {
            Err(format!(
                "New modification has invalid characters the comment, conformer: {:?}, comment \"{}\"",
                self.id(), new_modification.1
            ))
        } else {
            self.modification = Some(new_modification);
            Ok(())
        }
    }

    /// The amount of atoms making up this Conformer
    pub fn atom_count(&self) -> usize {
        self.atoms.len()
    }

    /// Get a specific atom from list of atoms making up this Conformer.
    ///
    /// ## Arguments
    /// * `index` - the index of the atom
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn atom(&self, index: usize) -> Option<&Atom> {
        self.atoms.get(index)
    }

    /// Get a specific atom as a mutable reference from list of atoms making up this Conformer.
    ///
    /// ## Arguments
    /// * `index` - the index of the atom
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn atom_mut(&mut self, index: usize) -> Option<&mut Atom> {
        self.atoms.get_mut(index)
    }

    /// Get a specific Atom specified by its serial number, which is defined to be unique
    /// within a single conformer. It does this using binary search so the underlying vector
    /// is assumed to be sorted, this can be enforced by using `conformer.sort()` beforehand.
    pub fn binary_find_atom(&self, serial_number: usize) -> Option<&Atom> {
        if let Ok(i) = self
            .atoms
            .binary_search_by(|a| a.serial_number().cmp(&serial_number))
        {
            unsafe { Some(self.atoms.get_unchecked(i)) }
        } else {
            None
        }
    }

    /// Get a specific Atom specified by its serial number, which is defined to be unique
    /// within a single conformer. It does this using binary search so the underlying vector
    /// is assumed to be sorted, this can be enforced by using `conformer.sort()` beforehand.
    pub fn binary_find_atom_mut(&mut self, serial_number: usize) -> Option<&mut Atom> {
        if let Ok(i) = self
            .atoms
            .binary_search_by(|a| a.serial_number().cmp(&serial_number))
        {
            unsafe { Some(self.atoms.get_unchecked_mut(i)) }
        } else {
            None
        }
    }

    /// Find all atoms matching the given information
    pub fn find(&self, atom: FindAtom) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.atoms().filter(move |a| atom.matches(a))
    }

    /// Find all atoms matching the given information
    pub fn find_mut(&mut self, atom: FindAtom) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.atoms_mut().filter(move |a| atom.matches(a))
    }

    /// Get the list of atoms making up this Conformer.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.atoms.iter()
    }

    /// Get the list of atoms making up this Conformer in parallel.
    #[doc_cfg(feature = "rayon")]
    pub fn par_atoms(&self) -> impl ParallelIterator<Item = &Atom> + '_ {
        self.atoms.par_iter()
    }

    /// Get the list of atoms as mutable references making up this Conformer.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn atoms_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.atoms.iter_mut()
    }

    /// Get the list of atoms as mutable references making up this Conformer in parallel.
    #[doc_cfg(feature = "rayon")]
    pub fn par_atoms_mut(&mut self) -> impl ParallelIterator<Item = &mut Atom> + '_ {
        self.atoms.par_iter_mut()
    }

    /// Add a new atom to the list of atoms making up this Conformer.
    /// ## Arguments
    /// * `new_atom` - the new Atom to add
    pub fn add_atom(&mut self, new_atom: Atom) {
        self.atoms.push(new_atom);
    }

    /// Returns if this Conformer is an amino acid
    pub fn is_amino_acid(&self) -> bool {
        reference_tables::get_amino_acid_number(self.name()).is_some()
    }

    /// Remove all Atoms matching the given predicate. As this is done in place this is the fastest way to remove Atoms from this Conformer.
    pub fn remove_atoms_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Atom) -> bool,
    {
        self.atoms.retain(|atom| !predicate(atom));
    }

    /// Remove the Atom specified.
    ///
    /// ## Arguments
    /// * `index` - the index of the atom to remove
    ///
    /// ## Panics
    /// It panics when the index is outside bounds.
    pub fn remove_atom(&mut self, index: usize) {
        self.atoms.remove(index);
    }

    /// Remove the Atom specified. It returns `true` if it found a matching Atom and removed it.
    /// It removes the first matching Atom from the list.
    ///
    /// ## Arguments
    /// * `serial_number` - the serial number of the Atom to remove
    ///
    /// ## Panics
    /// It panics when the index is outside bounds.
    pub fn remove_atom_by_serial_number(&mut self, serial_number: usize) -> bool {
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
    /// It removes the first matching Atom from the list. Matching is done in parallel.
    ///
    /// ## Arguments
    /// * `serial_number` - the serial number of the Atom to remove
    ///
    /// ## Panics
    /// It panics when the index is outside bounds.
    #[doc_cfg(feature = "rayon")]
    pub fn par_remove_atom_by_serial_number(&mut self, serial_number: usize) -> bool {
        let index = self
            .atoms
            .par_iter()
            .position_first(|a| a.serial_number() == serial_number);

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
    pub fn remove_atom_by_name(&mut self, name: String) -> bool {
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
    /// * `name` - the name of the Atom to remove
    ///
    /// ## Panics
    /// It panics when the index is outside bounds.
    #[doc_cfg(feature = "rayon")]
    pub fn par_remove_atom_by_name(&mut self, name: String) -> bool {
        let index = self.atoms.par_iter().position_first(|a| a.name() == name);

        if let Some(i) = index {
            self.remove_atom(i);
            true
        } else {
            false
        }
    }

    /// Apply a transformation to the position of all atoms making up this Conformer, the new position is immediately set.
    pub fn apply_transformation(&mut self, transformation: &TransformationMatrix) {
        for atom in self.atoms_mut() {
            atom.apply_transformation(transformation);
        }
    }

    /// Apply a transformation to the position of all atoms making up this Conformer, the new position is immediately set.
    /// This is done in parallel.
    #[doc_cfg(feature = "rayon")]
    pub fn par_apply_transformation(&mut self, transformation: &TransformationMatrix) {
        self.par_atoms_mut()
            .for_each(|a| a.apply_transformation(transformation))
    }

    /// Join this Conformer with another Conformer, this moves all atoms from the other Conformer
    /// to this Conformer. All other (meta) data of this Conformer will stay the same.
    pub fn join(&mut self, other: Conformer) {
        self.atoms.extend(other.atoms);
    }

    /// Extend the Atoms on this Conformer by the given iterator.
    pub fn extend<T: IntoIterator<Item = Atom>>(&mut self, iter: T) {
        self.atoms.extend(iter);
    }

    /// Sort the Atoms of this Conformer
    pub fn sort(&mut self) {
        self.atoms.sort();
    }

    /// Sort the Atoms of this Conformer in parallel
    #[doc_cfg(feature = "rayon")]
    pub fn par_sort(&mut self) {
        self.atoms.par_sort();
    }
}

impl fmt::Display for Conformer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CONFORMER ID:{:?}, Atoms:{}",
            self.id(),
            self.atoms.len(),
        )
    }
}

impl PartialOrd for Conformer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.id().cmp(&other.id()))
    }
}

impl Ord for Conformer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id().cmp(&other.id())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_text_validation() {
        let mut a = Conformer::new("A", None, None).unwrap();
        assert_eq!(Conformer::new("R̊", None, None), None);
        assert!(!a.set_name("Oͦ"));
        assert_eq!(a.name(), "A");
        a.set_name("atom");
        assert_eq!(a.name(), "ATOM");

        assert!(a.set_alternative_location("A"));
        assert!(!a.set_alternative_location("Aͦ"));
        assert_eq!(a.alternative_location(), Some("A"));

        assert!(a
            .set_modification(("ALA".to_string(), "Alanine".to_string()))
            .is_ok());
        assert!(a
            .set_modification(("ALAͦ".to_string(), "Alanine".to_string()))
            .is_err());
        assert!(a
            .set_modification(("ALA".to_string(), "Aͦlanine".to_string()))
            .is_err());
    }

    #[test]
    fn ordering_and_equality() {
        let a = Conformer::new("A", None, None).unwrap();
        let b = Conformer::new("A", None, None).unwrap();
        let c = Conformer::new("B", None, None).unwrap();
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert!(a < c);
        assert!(b < c);
    }

    #[test]
    fn test_empty() {
        let a = Conformer::new("A", None, None).unwrap();
        assert_eq!(a.modification(), None);
        assert_eq!(a.atom_count(), 0);
    }

    #[test]
    fn test_atom() {
        let mut a = Conformer::new("A", None, None).unwrap();
        let mut atom1 = Atom::new(false, 12, "CB", 1.0, 1.0, 1.0, 1.0, 1.0, "C", 0).unwrap();
        let atom2 = Atom::new(false, 13, "CB", 1.0, 1.0, 1.0, 1.0, 1.0, "C", 0).unwrap();
        a.add_atom(atom1.clone());
        a.add_atom(atom2.clone());
        a.add_atom(atom2);
        assert_eq!(a.atom(0), Some(&atom1));
        assert_eq!(a.atom_mut(0), Some(&mut atom1));
        a.remove_atom(0);
        assert!(a.remove_atom_by_name("CB".to_string()));
        assert!(a.remove_atom_by_serial_number(13));
        assert_eq!(a.atom_count(), 0);
    }

    #[test]
    fn check_display() {
        let a = Conformer::new("A", None, None).unwrap();
        format!("{:?}", a);
        format!("{}", a);
    }
}
