#![allow(dead_code)]
use crate::structs::*;
use crate::transformation::TransformationMatrix;
#[cfg(feature = "rayon")]
use rayon::prelude::*;
use std::cmp::Ordering;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
/// A Chain containing multiple Residues
pub struct Chain {
    /// The identifier of this Chain
    id: String,
    /// The Residues making up this Chain
    residues: Vec<Residue>,
    /// A possible reference to a database for this chain
    database_reference: Option<DatabaseReference>,
}

impl<'a> Chain {
    /// Create a new Chain
    ///
    /// ## Arguments
    /// * `id` - the identifier
    ///
    /// ## Fails
    /// It returns `None` if the identifier is an invalid character.
    #[must_use]
    pub fn new(id: impl AsRef<str>) -> Option<Self> {
        prepare_identifier(id).map(|id| Self {
            id,
            residues: Vec::new(),
            database_reference: None,
        })
    }

    /// Create a new Chain filled with the Residues provided.
    ///
    /// ## Fails
    /// It returns `None` if the identifier is an invalid character.
    pub fn from_iter(id: impl AsRef<str>, residues: impl Iterator<Item = Residue>) -> Option<Self> {
        prepare_identifier(id).map(|id| Self {
            id,
            residues: residues.collect(),
            database_reference: None,
        })
    }

    /// The ID of the Chain
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Set the ID of the Chain, returns `false` if the new id is an invalid character.
    /// The ID will be changed to uppercase as requested by PDB/PDBx standard.
    pub fn set_id(&mut self, new_id: impl AsRef<str>) -> bool {
        prepare_identifier(new_id).map(|id| self.id = id).is_some()
    }

    /// Get the database reference, if any, for this chain.
    pub const fn database_reference(&self) -> Option<&DatabaseReference> {
        self.database_reference.as_ref()
    }

    /// Get the database reference mutably, if any, for this chain.
    pub fn database_reference_mut(&mut self) -> Option<&mut DatabaseReference> {
        self.database_reference.as_mut()
    }

    /// Set the database reference for this chain.
    pub fn set_database_reference(&mut self, reference: DatabaseReference) {
        self.database_reference = Some(reference);
    }

    /// Get the number of Residues making up this Chain
    pub fn residue_count(&self) -> usize {
        self.residues.len()
    }

    /// Get the number of Conformers making up this Chain
    pub fn conformer_count(&self) -> usize {
        self.residues().map(Residue::conformer_count).sum()
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Get the number of Conformers making up this Chain in parallel
    #[cfg(feature = "rayon")]
    pub fn par_conformer_count(&self) -> usize {
        self.par_residues().map(Residue::conformer_count).sum()
    }

    /// Get the number of Atoms making up this Chain
    pub fn atom_count(&self) -> usize {
        self.residues().map(Residue::atom_count).sum()
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Get the number of Atoms making up this Chain in parallel
    #[cfg(feature = "rayon")]
    pub fn par_atom_count(&self) -> usize {
        self.par_residues().map(Residue::par_atom_count).sum()
    }

    /// Get a reference to a specific Residue from list of Residues making up this Chain.
    ///
    /// ## Arguments
    /// * `index` - the index of the Residue
    ///
    /// ## Fails
    /// It returns `None` if the index is out of bounds.
    pub fn residue(&self, index: usize) -> Option<&Residue> {
        self.residues.get(index)
    }

    /// Get a mutable reference to a specific Residue from the list of Residues making up this Chain.
    ///
    /// ## Arguments
    /// * `index` - the index of the Residue
    ///
    /// ## Fails
    /// It returns `None` if the index is out of bounds.
    pub fn residue_mut(&mut self, index: usize) -> Option<&mut Residue> {
        self.residues.get_mut(index)
    }

    /// Get a reference to a specific Conformer from list of Conformers making up this Chain.
    ///
    /// ## Arguments
    /// * `index` - the index of the Conformer
    ///
    /// ## Fails
    /// It returns `None` if the index is out of bounds.
    pub fn conformer(&self, index: usize) -> Option<&Conformer> {
        self.conformers().nth(index)
    }

    /// Get a mutable reference to a specific Conformer from list of Conformers making up this Chain.
    ///
    /// ## Arguments
    /// * `index` - the index of the Conformer
    ///
    /// ## Fails
    /// It returns `None` if the index is out of bounds.
    pub fn conformer_mut(&mut self, index: usize) -> Option<&mut Conformer> {
        self.conformers_mut().nth(index)
    }

    /// Get a reference to a specific Atom from the Atoms making up this Chain.
    ///
    /// ## Arguments
    /// * `index` - the index of the Atom
    ///
    /// ## Fails
    /// It returns `None` if the index is out of bounds.
    pub fn atom(&self, index: usize) -> Option<&Atom> {
        self.atoms().nth(index)
    }

    /// Get a mutable reference to a specific Atom from the Atoms making up this Chain.
    ///
    /// ## Arguments
    /// * `index` - the index of the Atom
    ///
    /// ## Fails
    /// It returns `None` if the index is out of bounds.
    pub fn atom_mut(&mut self, index: usize) -> Option<&mut Atom> {
        self.atoms_mut().nth(index)
    }

    /// Get a reference to the specified atom. Its uniqueness is guaranteed by including the
    /// `alternative_location`, with its full hierarchy. The algorithm is based
    /// on binary search so it is faster than an exhaustive search, but the
    /// full structure is assumed to be sorted. This assumption can be enforced
    /// by using `pdb.full_sort()`.
    #[allow(clippy::unwrap_used)]
    pub fn binary_find_atom(
        &'a self,
        serial_number: usize,
        alternative_location: Option<&str>,
    ) -> Option<AtomConformerResidue<'a>> {
        if self.residue_count() == 0 {
            None
        } else {
            self.residues
                .binary_search_by(|residue| {
                    let low = residue.atoms().next().expect(
                        "All residues should have at least a single atom for binary_find_atom",
                    );
                    let high = residue.atoms().next_back().expect(
                        "All residues should have at least a single atom for binary_find_atom",
                    );

                    if low.serial_number() <= serial_number && serial_number <= high.serial_number()
                    {
                        Ordering::Equal
                    } else if serial_number < low.serial_number() {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                })
                .ok()
                .and_then(|index| {
                    self.residue(index)
                        .unwrap()
                        .binary_find_atom(serial_number, alternative_location)
                        .map(|h| h.extend(self.residue(index).unwrap()))
                })
        }
    }

    /// Get a mutable reference to the specified atom. Its uniqueness is guaranteed by including the
    /// `alternative_location`, with its full hierarchy. The algorithm is based
    /// on binary search so it is faster than an exhaustive search, but the
    /// full structure is assumed to be sorted. This assumption can be enforced
    /// by using `pdb.full_sort()`.
    #[allow(clippy::unwrap_used)]
    pub fn binary_find_atom_mut(
        &'a mut self,
        serial_number: usize,
        alternative_location: Option<&str>,
    ) -> Option<AtomConformerResidueMut<'a>> {
        if self.residue_count() == 0 {
            None
        } else {
            self.residues
                .binary_search_by(|residue| {
                    let low = residue.atoms().next().expect(
                        "All residues should have at least a single atom for binary_find_atom",
                    );
                    let high = residue.atoms().next_back().expect(
                        "All residues should have at least a single atom for binary_find_atom",
                    );

                    if low.serial_number() <= serial_number && serial_number <= high.serial_number()
                    {
                        Ordering::Equal
                    } else if serial_number < low.serial_number() {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                })
                .ok()
                .and_then(move |index| {
                    let residue: *mut Residue = self.residue_mut(index).unwrap();
                    self.residue_mut(index)
                        .unwrap()
                        .binary_find_atom_mut(serial_number, alternative_location)
                        .map(|h| h.extend(residue))
                })
        }
    }

    /// Find all hierarchies matching the given information
    pub fn find(
        &'a self,
        search: Search,
    ) -> impl DoubleEndedIterator<Item = AtomConformerResidue<'a>> + 'a {
        self.residues()
            .map(move |r| (r, search.clone().add_residue_info(r)))
            .filter(|(_r, search)| !matches!(search, Search::Known(false)))
            .flat_map(move |(r, search)| r.find(search).map(move |h| h.extend(r)))
    }

    /// Find all hierarchies matching the given information
    pub fn find_mut(
        &'a mut self,
        search: Search,
    ) -> impl DoubleEndedIterator<Item = AtomConformerResidueMut<'a>> + 'a {
        self.residues_mut()
            .map(move |r| {
                let search = search.clone().add_residue_info(r);
                (r, search)
            })
            .filter(|(_r, search)| !matches!(search, Search::Known(false)))
            .flat_map(move |(r, search)| {
                let r_ptr: *mut Residue = r;
                r.find_mut(search).map(move |h| h.extend(r_ptr))
            })
    }

    /// Get an iterator of references to Residues making up this Chain.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn residues(&self) -> impl DoubleEndedIterator<Item = &Residue> + '_ {
        self.residues.iter()
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Get a parallel iterator of references to Residues making up this Chain.
    #[cfg(feature = "rayon")]
    pub fn par_residues(&self) -> impl ParallelIterator<Item = &Residue> + '_ {
        self.residues.par_iter()
    }

    /// Get an iterator of mutable references to Residues making up this Chain.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn residues_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Residue> + '_ {
        self.residues.iter_mut()
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Get a parallel iterator of mutable references to Residues making up this Chain.
    #[cfg(feature = "rayon")]
    pub fn par_residues_mut(&mut self) -> impl ParallelIterator<Item = &mut Residue> + '_ {
        self.residues.par_iter_mut()
    }

    /// Get an iterator of references to Conformers making up this Chain.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn conformers(&self) -> impl DoubleEndedIterator<Item = &Conformer> + '_ {
        self.residues().flat_map(Residue::conformers)
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Get a parallel iterator of references to Conformers making up this Chain.
    #[cfg(feature = "rayon")]
    pub fn par_conformers(&self) -> impl ParallelIterator<Item = &Conformer> + '_ {
        self.par_residues().flat_map(Residue::par_conformers)
    }

    /// Get an iterator of mutable references to Conformers making up this Chain.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn conformers_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Conformer> + '_ {
        self.residues_mut().flat_map(Residue::conformers_mut)
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Get a parallel iterator of mutable references to Conformers making up this Chain.
    #[cfg(feature = "rayon")]
    pub fn par_conformers_mut(&mut self) -> impl ParallelIterator<Item = &mut Conformer> + '_ {
        self.par_residues_mut()
            .flat_map(Residue::par_conformers_mut)
    }

    /// Get an iterator of references to Atoms making up this Chain.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.residues().flat_map(Residue::atoms)
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Get a parallel iterator of references to Atoms making up this Chain.
    #[cfg(feature = "rayon")]
    pub fn par_atoms(&self) -> impl ParallelIterator<Item = &Atom> + '_ {
        self.par_residues().flat_map(Residue::par_atoms)
    }

    /// Get an iterator of mutable references to Atoms making up this Chain.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn atoms_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.residues_mut().flat_map(Residue::atoms_mut)
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Get a parallel iterator of mutablereferences to Atoms making up this Chain.
    #[cfg(feature = "rayon")]
    pub fn par_atoms_mut(&mut self) -> impl ParallelIterator<Item = &mut Atom> + '_ {
        self.par_residues_mut().flat_map(Residue::par_atoms_mut)
    }

    /// Get an iterator of references to a struct containing all atoms with their hierarchy making up this Chain.
    pub fn atoms_with_hierarchy(
        &'a self,
    ) -> impl DoubleEndedIterator<Item = AtomConformerResidue<'a>> + 'a {
        self.residues()
            .flat_map(|r| r.atoms_with_hierarchy().map(move |h| h.extend(r)))
    }

    /// Get an iterator of mutable references to a struct containing all atoms with their hierarchy making up this Chain.
    pub fn atoms_with_hierarchy_mut(
        &'a mut self,
    ) -> impl DoubleEndedIterator<Item = AtomConformerResidueMut<'a>> + 'a {
        self.residues_mut().flat_map(|r| {
            let residue: *mut Residue = r;
            r.atoms_with_hierarchy_mut().map(move |h| h.extend(residue))
        })
    }

    /// Add a new Atom to this Chain. If a Residue with the given serial number already exists, the
    /// Atom will be added to it, otherwise a new Residue is created to hold the created atom
    /// and added to the list of Residues in this chain.
    ///
    /// ## Arguments
    /// * `new_atom` - the new Atom to add
    /// * `residue_id` - the id construct of the Residue to add the Atom to
    /// * `conformer_id` - the id construct of the Conformer to add the Atom to
    ///
    /// ## Panics
    /// It panics if the given Residue ID contains any invalid characters.
    pub fn add_atom(
        &mut self,
        new_atom: Atom,
        residue_id: (isize, Option<&str>),
        conformer_id: (impl AsRef<str>, Option<&str>),
    ) {
        let mut found = false;
        let mut new_residue = Residue::new(residue_id.0, residue_id.1, None)
            .expect("Invalid chars in Residue creation");
        let mut current_residue = &mut new_residue;
        for residue in &mut self.residues.iter_mut().rev() {
            if residue.id() == residue_id {
                current_residue = residue;
                found = true;
                break;
            }
        }
        #[allow(clippy::unwrap_used)]
        if !found {
            self.residues.push(new_residue);
            current_residue = self.residues.last_mut().unwrap();
        }

        current_residue.add_atom(new_atom, conformer_id);
    }

    /// Add a Residue to the end of to the list of Residues making up this Chain. This does not detect any duplicates of names or serial numbers in the list of Residues.
    pub fn add_residue(&mut self, residue: Residue) {
        self.residues.push(residue);
    }

    /// Inserts a Residue at the given index into the list of Residues making up this Chain. This does not detect any duplicates of names or serial numbers in the list of Residues.
    /// This panics if `index > len`.
    pub fn insert_residue(&mut self, index: usize, residue: Residue) {
        self.residues.insert(index, residue);
    }

    /// Remove all Atoms matching the given predicate. As this is done in place this is the fastest way to remove Atoms from this Chain.
    pub fn remove_atoms_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Atom) -> bool,
    {
        for residue in self.residues_mut() {
            residue.remove_atoms_by(&predicate);
        }
    }

    /// Remove all Conformers matching the given predicate. As this is done in place this is the fastest way to remove Conformers from this Chain.
    pub fn remove_conformers_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Conformer) -> bool,
    {
        for residue in self.residues_mut() {
            residue.remove_conformers_by(&predicate);
        }
    }

    /// Remove all residues matching the given predicate. As this is done in place this is the fastest way to remove Residues from this Chain.
    pub fn remove_residues_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Residue) -> bool,
    {
        self.residues.retain(|residue| !predicate(residue));
    }

    /// Remove the specified Residue.
    ///
    /// ## Arguments
    /// * `index` - the index of the Residue to remove
    ///
    /// ## Panics
    /// It panics if the index is out of bounds.
    pub fn remove_residue(&mut self, index: usize) {
        self.residues.remove(index);
    }

    /// Remove the specified Residue. Returns `true` if a matching Residue was found and removed.
    /// Removes the first matching Residue from the list.
    ///
    /// ## Arguments
    /// * `id` - the id construct of the Residue to remove (see [`Residue::id`])
    pub fn remove_residue_by_id(&mut self, id: (isize, Option<&str>)) -> bool {
        let index = self.residues.iter().position(|a| a.id() == id);

        index.map_or(false, |i| {
            self.remove_residue(i);
            true
        })
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Remove the specified Residue. Returns `true` if a matching Residue was found and removed.
    /// Removes the first matching Residue from the list.
    ///
    /// ## Arguments
    /// * `id` - the id construct of the Residue to remove (see [`Residue::id`])
    #[cfg(feature = "rayon")]
    pub fn par_remove_residue_by_id(&mut self, id: (isize, Option<&str>)) -> bool {
        let index = self.residues.par_iter().position_first(|a| a.id() == id);

        index.map_or(false, |i| {
            self.remove_residue(i);
            true
        })
    }

    /// Remove all empty Residues from this Chain, and all empty Conformers from the Residues.
    pub fn remove_empty(&mut self) {
        self.residues_mut().for_each(Residue::remove_empty);
        self.residues.retain(|r| r.conformer_count() > 0);
    }

    /// Apply a transformation to the position of all atoms making up this Chain, the new position is immediately set.
    pub fn apply_transformation(&mut self, transformation: &TransformationMatrix) {
        for atom in self.atoms_mut() {
            atom.apply_transformation(transformation);
        }
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Apply a transformation to the position of all atoms making up this Chain, the new position is immediately set.
    /// Done in parallel.
    #[cfg(feature = "rayon")]
    pub fn par_apply_transformation(&mut self, transformation: &TransformationMatrix) {
        self.par_atoms_mut()
            .for_each(|atom| atom.apply_transformation(transformation));
    }

    /// Join this Chain with another Chain, this moves all atoms from the other Chain
    /// to this Chain. All other (meta) data of this Chain will stay the same.
    pub fn join(&mut self, other: Self) {
        self.residues.extend(other.residues);
    }

    /// Sort the residues of this chain
    pub fn sort(&mut self) {
        self.residues.sort();
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Sort the residues of this chain in parallel
    #[cfg(feature = "rayon")]
    pub fn par_sort(&mut self) {
        self.residues.par_sort();
    }
}

use std::fmt;
impl fmt::Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CHAIN ID:{}, Residues: {}",
            self.id(),
            self.residues.len()
        )
    }
}

impl PartialOrd for Chain {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.id().cmp(other.id()))
    }
}

impl Ord for Chain {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id().cmp(other.id())
    }
}

impl Extend<Residue> for Chain {
    /// Extend the Residues on this Chain by the given iterator of Residues.
    fn extend<T: IntoIterator<Item = Residue>>(&mut self, iter: T) {
        self.residues.extend(iter);
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_id_validation() {
        let mut a = Chain::new("A").unwrap();
        assert_eq!(Chain::new("R̊"), None);
        assert!(!a.set_id("Oͦ"));
        assert_eq!(a.id(), "A");
        a.set_id("atom");
        assert_eq!(a.id(), "atom");
    }

    #[test]
    fn ordering_and_equality() {
        let a = Chain::new("A").unwrap();
        let b = Chain::new("A").unwrap();
        let c = Chain::new("B").unwrap();
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert!(a < c);
        assert!(b < c);
    }

    #[test]
    fn test_empty_chain() {
        let mut a = Chain::new("A").unwrap();
        assert_eq!(a.database_reference(), None);
        assert_eq!(a.database_reference_mut(), None);
        assert_eq!(a.residue_count(), 0);
        assert_eq!(a.conformer_count(), 0);
        assert_eq!(a.atom_count(), 0);
    }

    #[test]
    fn test_residue() {
        let mut a = Chain::new("A").unwrap();
        let mut r = Residue::new(1, None, None).unwrap();
        a.add_residue(r.clone());
        a.add_residue(Residue::new(13, None, None).unwrap());
        assert_eq!(a.residue(0), Some(&r));
        assert_eq!(a.residue_mut(0), Some(&mut r));
        a.remove_residue(0);
        assert!(a.remove_residue_by_id((13, None)));
        assert_eq!(a.residue_count(), 0);
        assert!(!a.remove_residue_by_id((13, None)));
    }
}
