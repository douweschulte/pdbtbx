#![allow(dead_code)]
use crate::structs::hierarchy::*;
use crate::structs::*;
use crate::transformation::TransformationMatrix;
#[cfg(feature = "rayon")]
use rayon::prelude::*;
use std::cmp::Ordering;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
/// A Model containing multiple Chains.
pub struct Model {
    /// The serial number of this Model
    serial_number: usize,
    /// The Chains making up this model
    chains: Vec<Chain>,
}

impl<'a> Model {
    /// Create a new Model.
    ///
    /// ## Arguments
    /// * `serial_number` - the serial number
    #[must_use]
    pub const fn new(serial_number: usize) -> Self {
        Self {
            serial_number,
            chains: Vec::new(),
        }
    }

    /// Create a new Model.
    ///
    /// ## Arguments
    /// * `serial_number` - the serial number
    pub fn from_iter(serial_number: usize, chains: impl Iterator<Item = Chain>) -> Self {
        Self {
            serial_number,
            chains: chains.collect(),
        }
    }

    /// Get the serial number of this Model.
    pub const fn serial_number(&self) -> usize {
        self.serial_number
    }

    /// Set the serial number of this Model.
    pub fn set_serial_number(&mut self, new_number: usize) {
        self.serial_number = new_number;
    }

    /// Get the number of Chains making up this Model.
    pub fn chain_count(&self) -> usize {
        self.chains.len()
    }

    /// Get the number of Residues making up this Model.
    pub fn residue_count(&self) -> usize {
        self.chains().map(Chain::residue_count).sum()
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Get the number of Residues making up this Model in parallel.
    #[cfg(feature = "rayon")]
    pub fn par_residue_count(&self) -> usize {
        self.par_chains().map(Chain::residue_count).sum()
    }

    /// Get the number of Conformers making up this Model.
    pub fn conformer_count(&self) -> usize {
        self.chains().map(Chain::conformer_count).sum()
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Get the number of Conformers making up this Model in parallel.
    #[cfg(feature = "rayon")]
    pub fn par_conformer_count(&self) -> usize {
        self.par_chains().map(Chain::par_conformer_count).sum()
    }

    /// Get the number of Atoms making up this Model.
    pub fn atom_count(&self) -> usize {
        self.chains().map(Chain::atom_count).sum()
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Get the number of Atoms making up this Model in parallel.
    #[cfg(feature = "rayon")]
    pub fn par_atom_count(&self) -> usize {
        self.par_chains().map(Chain::par_atom_count).sum()
    }

    /// Get a reference to a specific Chain from list of Chains making up this Model.
    ///
    /// ## Arguments
    /// * `index` - the index of the Chain
    ///
    /// ## Fails
    /// Returns `None` if the index is out of bounds.
    pub fn chain(&self, index: usize) -> Option<&Chain> {
        self.chains().nth(index)
    }

    /// Get a mutable reference to a specific Chain reference from list of Chains making up this Model.
    ///
    /// ## Arguments
    /// * `index` - the index of the Chain
    ///
    /// ## Fails
    /// Returns `None` if the index is out of bounds.
    pub fn chain_mut(&mut self, index: usize) -> Option<&mut Chain> {
        self.chains_mut().nth(index)
    }

    /// Get a reference to a specific Residue from the Residues making up this Model.
    ///
    /// ## Arguments
    /// * `index` - the index of the Residue
    ///
    /// ## Fails
    /// Returns `None` if the index is out of bounds.
    pub fn residue(&self, index: usize) -> Option<&Residue> {
        self.residues().nth(index)
    }

    /// Get a mutable reference to a specific Residue from the Residues making up this Model.
    ///
    /// ## Arguments
    /// * `index` - the index of the Residue
    ///
    /// ## Fails
    /// Returns `None` if the index is out of bounds.
    pub fn residue_mut(&mut self, index: usize) -> Option<&mut Residue> {
        self.residues_mut().nth(index)
    }

    /// Get a reference to a specific Conformer from the Conformers making up this Model.
    ///
    /// ## Arguments
    /// * `index` - the index of the Conformer
    ///
    /// ## Fails
    /// Returns `None` if the index is out of bounds.
    pub fn conformer(&self, index: usize) -> Option<&Conformer> {
        self.conformers().nth(index)
    }

    /// Get a mutable reference to a specific Conformer from the Conformers making up this Model.
    ///
    /// ## Arguments
    /// * `index` - the index of the Conformer
    ///
    /// ## Fails
    /// Returns `None` if the index is out of bounds.
    pub fn conformer_mut(&mut self, index: usize) -> Option<&mut Conformer> {
        self.conformers_mut().nth(index)
    }

    /// Get a reference to a specific Atom from the Atoms making up this Model.
    ///
    /// ## Arguments
    /// * `index` - the index of the Atom
    ///
    /// ## Fails
    /// Returns `None` if the index is out of bounds.
    pub fn atom(&self, index: usize) -> Option<&Atom> {
        self.atoms().nth(index)
    }

    /// Get a mutable reference to a specific Atom from the Atoms making up this Model.
    ///
    /// ## Arguments
    /// * `index` - the index of the Atom
    ///
    /// ## Fails
    /// Returns `None` if the index is out of bounds.
    pub fn atom_mut(&mut self, index: usize) -> Option<&mut Atom> {
        self.atoms_mut().nth(index)
    }

    /// Get a reference to the specified atom. Its uniqueness is guaranteed by including the
    /// `insertion_code`, with its full hierarchy. The algorithm is based
    /// on binary search so it is faster than an exhaustive search, but the
    /// full structure is assumed to be sorted. This assumption can be enforced
    /// by using `pdb.full_sort()`.
    #[allow(clippy::unwrap_used)]
    pub fn binary_find_atom(
        &'a self,
        serial_number: usize,
        insertion_code: Option<&str>,
    ) -> Option<AtomConformerResidueChain<'a>> {
        if self.chain_count() == 0 {
            None
        } else {
            self.chains
                .binary_search_by(|chain| {
                    if let (Some(low), Some(high)) =
                        (chain.atoms().next(), chain.atoms().next_back())
                    {
                        if low.serial_number() <= serial_number
                            && serial_number <= high.serial_number()
                        {
                            Ordering::Equal
                        } else if serial_number < low.serial_number() {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    } else {
                        panic!(
                            "All chains should have at least a single atom for binary_find_atom"
                        );
                    }
                })
                .ok()
                .and_then(|index| {
                    self.chain(index)
                        .unwrap()
                        .binary_find_atom(serial_number, insertion_code)
                        .map(|h| h.extend(self.chain(index).unwrap()))
                })
        }
    }

    /// Get a mutable reference to the specified atom. Its uniqueness is guaranteed by
    /// including the `insertion_code`, with its full hierarchy. The algorithm is based
    /// on binary search so it is faster than an exhaustive search, but the
    /// full structure is assumed to be sorted. This assumption can be enforced
    /// by using `pdb.full_sort()`.
    #[allow(clippy::unwrap_used)]
    pub fn binary_find_atom_mut(
        &'a mut self,
        serial_number: usize,
        insertion_code: Option<&str>,
    ) -> Option<AtomConformerResidueChainMut<'a>> {
        if self.chain_count() == 0 {
            None
        } else {
            self.chains
                .binary_search_by(|chain| {
                    if let (Some(low), Some(high)) =
                        (chain.atoms().next(), chain.atoms().next_back())
                    {
                        if low.serial_number() <= serial_number
                            && serial_number <= high.serial_number()
                        {
                            Ordering::Equal
                        } else if serial_number < low.serial_number() {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    } else {
                        panic!(
                            "All chains should have at least a single atom for binary_find_atom"
                        );
                    }
                })
                .ok()
                .and_then(move |index| {
                    let chain: *mut Chain = self.chain_mut(index).unwrap();
                    self.chain_mut(index)
                        .unwrap()
                        .binary_find_atom_mut(serial_number, insertion_code)
                        .map(|h| h.extend(chain))
                })
        }
    }

    /// Find all hierarchies matching the given search. For more details see [Search].
    pub fn find(
        &'a self,
        search: Search,
    ) -> impl DoubleEndedIterator<Item = AtomConformerResidueChain<'a>> + 'a {
        self.chains()
            .map(move |c| (c, search.clone().add_chain_info(c)))
            .filter(|(_c, search)| !matches!(search, Search::Known(false)))
            .flat_map(move |(c, search)| c.find(search).map(move |h| h.extend(c)))
    }

    /// Find all hierarchies matching the given search. For more details see [Search].
    pub fn find_mut(
        &'a mut self,
        search: Search,
    ) -> impl DoubleEndedIterator<Item = AtomConformerResidueChainMut<'a>> + 'a {
        self.chains_mut()
            .map(move |c| {
                let search = search.clone().add_chain_info(c);
                (c, search)
            })
            .filter(|(_c, search)| !matches!(search, Search::Known(false)))
            .flat_map(move |(c, search)| {
                let c_ptr: *mut Chain = c;
                c.find_mut(search).map(move |h| h.extend(c_ptr))
            })
    }

    /// Get an iterator of references to Chains making up this Model.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn chains(&self) -> impl DoubleEndedIterator<Item = &Chain> + '_ {
        self.chains.iter()
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Get a parallel iterator of references to Chains making up this Model.
    #[cfg(feature = "rayon")]
    pub fn par_chains(&self) -> impl ParallelIterator<Item = &Chain> + '_ {
        self.chains.par_iter()
    }

    /// Get an iterator of mutable references to Chains making up this Model.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn chains_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Chain> + '_ {
        self.chains.iter_mut()
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Get a parallel iterator of mutable references to Chains making up this Model.
    #[cfg(feature = "rayon")]
    pub fn par_chains_mut(&mut self) -> impl ParallelIterator<Item = &mut Chain> + '_ {
        self.chains.par_iter_mut()
    }

    /// Get an iterator of references to Residues making up this Model.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn residues(&self) -> impl DoubleEndedIterator<Item = &Residue> + '_ {
        self.chains().flat_map(Chain::residues)
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Get a parallel iterator of references to Residues making up this Model.
    #[cfg(feature = "rayon")]
    pub fn par_residues(&self) -> impl ParallelIterator<Item = &Residue> + '_ {
        self.par_chains().flat_map(Chain::par_residues)
    }

    /// Get an iterator of mutable references to Residues making up this Model.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn residues_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Residue> + '_ {
        self.chains_mut().flat_map(Chain::residues_mut)
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Get a parallel iterator of mutable references to Residues making up this Model.
    #[cfg(feature = "rayon")]
    pub fn par_residues_mut(&mut self) -> impl ParallelIterator<Item = &mut Residue> + '_ {
        self.par_chains_mut().flat_map(Chain::par_residues_mut)
    }

    /// Get an iterator of references to Conformers making up this Model.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn conformers(&self) -> impl DoubleEndedIterator<Item = &Conformer> + '_ {
        self.chains().flat_map(Chain::conformers)
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Get a parallel iterator of references to Conformers making up this Model.
    #[cfg(feature = "rayon")]
    pub fn par_conformers(&self) -> impl ParallelIterator<Item = &Conformer> + '_ {
        self.par_chains().flat_map(Chain::par_conformers)
    }

    /// Get an iterator of mutable references to Conformers making up this Model.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn conformers_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Conformer> + '_ {
        self.chains_mut().flat_map(Chain::conformers_mut)
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Get a parallel iterator of mutable references to Conformers making up this Model.
    #[cfg(feature = "rayon")]
    pub fn par_conformers_mut(&mut self) -> impl ParallelIterator<Item = &mut Conformer> + '_ {
        self.par_chains_mut().flat_map(Chain::par_conformers_mut)
    }

    /// Get an iterator of references to Atoms making up this Model.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.chains().flat_map(Chain::atoms)
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Get a parallel iterator of references to Atoms making up this Model.
    #[cfg(feature = "rayon")]
    pub fn par_atoms(&self) -> impl ParallelIterator<Item = &Atom> + '_ {
        self.par_chains().flat_map(Chain::par_atoms)
    }

    /// Get an iterator of mutable references to Atoms making up this Model.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn atoms_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.chains_mut().flat_map(Chain::atoms_mut)
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Get a parallel iterator of mutable references to Atoms making up this Model.
    #[cfg(feature = "rayon")]
    pub fn par_atoms_mut(&mut self) -> impl ParallelIterator<Item = &mut Atom> + '_ {
        self.par_chains_mut().flat_map(Chain::par_atoms_mut)
    }

    /// Get an iterator of references to a struct containing all atoms with their hierarchy making up this Model.
    pub fn atoms_with_hierarchy(
        &'a self,
    ) -> impl DoubleEndedIterator<Item = AtomConformerResidueChain<'a>> + 'a {
        self.chains()
            .flat_map(|c| c.atoms_with_hierarchy().map(move |h| h.extend(c)))
    }

    /// Get an iterator of mutable references to a struct containing all atoms with their hierarchy making up this Model.
    pub fn atoms_with_hierarchy_mut(
        &'a mut self,
    ) -> impl DoubleEndedIterator<Item = AtomConformerResidueChainMut<'a>> + 'a {
        self.chains_mut().flat_map(|c| {
            let chain: *mut Chain = c;
            c.atoms_with_hierarchy_mut().map(move |h| h.extend(chain))
        })
    }

    /// Add a new Atom to this Model. It finds if there already is a Chain with the given `chain_id` if there is it will add this atom to that Chain, otherwise it will create a new Chain and add that to the list of Chains making up this Model. It does the same for the Residue, so it will create a new one if there does not yet exist a Residue with the given serial number.
    ///
    /// ## Arguments
    /// * `new_atom` - the new Atom to add
    /// * `chain_id` - the id of the Chain to add the Atom to
    /// * `residue_id` - the id construct of the Residue to add the Atom to
    /// * `conformer_id` - the id construct of the Conformer to add the Atom to
    ///
    /// ## Panics
    /// It panics if the Chain ID or Residue ID contain any invalid characters.
    pub fn add_atom(
        &mut self,
        new_atom: Atom,
        chain_id: impl AsRef<str>,
        residue_id: (isize, Option<&str>),
        conformer_id: (impl AsRef<str>, Option<&str>),
    ) {
        let chain_id = chain_id.as_ref().trim();
        let mut found = false;
        let mut new_chain = Chain::new(chain_id).expect("Invalid characters in chain creation");
        let mut current_chain = &mut new_chain;
        for chain in &mut self.chains {
            if chain.id() == chain_id {
                current_chain = chain;
                found = true;
                break;
            }
        }
        #[allow(clippy::unwrap_used)]
        if !found {
            // As this moves the chain the atom should be added later to keep the reference intact
            self.chains.push(new_chain);
            current_chain = self.chains.last_mut().unwrap();
        }

        current_chain.add_atom(new_atom, residue_id, conformer_id);
    }

    /// Add a Chain to the list of Chains making up this Model. This does not detect any duplicates of names or serial numbers in the list of Chains.
    pub fn add_chain(&mut self, chain: Chain) {
        self.chains.push(chain);
    }

    /// Remove all Atoms matching the given predicate.
    /// As this is done in place this is the fastest way to remove Atoms from this Model.
    pub fn remove_atoms_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Atom) -> bool,
    {
        for residue in self.residues_mut() {
            residue.remove_atoms_by(&predicate);
        }
    }

    /// Remove all Conformers matching the given predicate.
    /// As this is done in place this is the fastest way to remove Conformers from this Model.
    pub fn remove_conformers_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Conformer) -> bool,
    {
        for chain in self.chains_mut() {
            chain.remove_conformers_by(&predicate);
        }
    }

    /// Remove all Residues matching the given predicate.
    /// As this is done in place this is the fastest way to remove Residues from this Model.
    pub fn remove_residues_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Residue) -> bool,
    {
        for chain in self.chains_mut() {
            chain.remove_residues_by(&predicate);
        }
    }

    /// Remove all Chains matching the given predicate.
    /// As this is done in place this is the fastest way to remove Chains from this Model.
    pub fn remove_chains_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Chain) -> bool,
    {
        self.chains.retain(|chain| !predicate(chain));
    }

    /// Remove the Chain specified.
    ///
    /// ## Arguments
    /// * `index` - the index of the Chain to remove
    ///
    /// ## Panics
    /// Panics if the index is out of bounds.
    pub fn remove_chain(&mut self, index: usize) -> Chain {
        self.chains.remove(index)
    }

    /// Remove the Chain specified. It returns `true` if it found a matching Chain and removed it.
    /// It removes the first matching Chain from the list.
    ///
    /// ## Arguments
    /// * `id` - the id of the Chain to remove
    pub fn remove_chain_by_id(&mut self, id: impl AsRef<str>) -> bool {
        let id = id.as_ref();
        let index = self.chains().position(|a| a.id() == id);

        index.map_or(false, |i| {
            self.remove_chain(i);
            true
        })
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Remove the Chain specified. It returns `true` if it found a matching Chain and removed it.
    /// It removes the first matching Chain from the list.
    /// Done in parallel.
    ///
    /// ## Arguments
    /// * `id` - the id of the Chain to remove
    #[cfg(feature = "rayon")]
    pub fn par_remove_chain_by_id(&mut self, id: impl AsRef<str>) -> bool {
        let id = id.as_ref();
        let index = self.chains.par_iter().position_first(|a| a.id() == id);

        index.map_or(false, |i| {
            self.remove_chain(i);
            true
        })
    }

    /// Remove all empty Chain from this Model, and all empty Residues from the Chains.
    pub fn remove_empty(&mut self) {
        self.chains_mut().for_each(Chain::remove_empty);
        self.chains.retain(|c| c.residue_count() > 0);
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Remove all empty Chain from this Model, and all empty Residues from the Chains in parallel.
    #[cfg(feature = "rayon")]
    pub fn par_remove_empty(&mut self) {
        self.par_chains_mut().for_each(Chain::remove_empty);
        self.chains.retain(|c| c.residue_count() > 0);
    }

    /// Apply a transformation to the position of all atoms making up this Model, the new position is immediately set.
    pub fn apply_transformation(&mut self, transformation: &TransformationMatrix) {
        for atom in self.atoms_mut() {
            atom.apply_transformation(transformation);
        }
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Apply a transformation to the position of all atoms making up this Model, the new position is immediately set.
    /// Done in parallel.
    #[cfg(feature = "rayon")]
    pub fn par_apply_transformation(&mut self, transformation: &TransformationMatrix) {
        self.par_atoms_mut()
            .for_each(|atom| atom.apply_transformation(transformation));
    }

    /// Join this Model with another Model, this moves all atoms from the other Model
    /// to this Model. All other (meta) data of this Model will stay the same. It will add
    /// new Chains and Residues as defined in the other model.
    pub fn join(&mut self, other: Self) {
        self.chains.extend(other.chains);
    }

    /// Sort the Chains of this Model.
    pub fn sort(&mut self) {
        self.chains.sort();
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Sort the Chains of this Model in parallel.
    #[cfg(feature = "rayon")]
    pub fn par_sort(&mut self) {
        self.chains.par_sort();
    }
}

use std::fmt;
impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MODEL SerialNumber:{}, Chains: {}",
            self.serial_number,
            self.chains.len()
        )
    }
}

impl PartialOrd for Model {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.serial_number.cmp(&other.serial_number))
    }
}

impl Ord for Model {
    fn cmp(&self, other: &Self) -> Ordering {
        self.serial_number.cmp(&other.serial_number)
    }
}

impl Extend<Chain> for Model {
    /// Extend the Chains on this Model by the given iterator of Chains.
    fn extend<T: IntoIterator<Item = Chain>>(&mut self, iter: T) {
        self.chains.extend(iter);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equality() {
        let a = Model::new(0);
        let b = a.clone();
        let c = Model::new(1);
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(b, c);
        assert!(a < c);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_children() {
        let mut a = Model::new(0);
        a.add_chain(Chain::new("A").unwrap());
        let mut iter = a.chains();
        assert_eq!(iter.next(), Some(&Chain::new("A").unwrap()));
        assert_eq!(iter.next(), None);
        assert_eq!(a.chain_count(), 1);
        assert_eq!(a.conformer_count(), 0);
        assert_eq!(a.residue_count(), 0);
        assert_eq!(a.atom_count(), 0);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_remove() {
        let mut a = Model::new(0);
        a.add_chain(Chain::new("A").unwrap());
        a.add_chain(Chain::new("C").unwrap());
        assert_eq!(a.remove_chain(0), Chain::new("A").unwrap());
        a.remove_chain_by_id("C");
        assert_eq!(a.chain_count(), 0);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_hierarchy_mut() {
        // Create the struct to use
        let mut a = Model::new(0);
        a.add_chain(Chain::new("A").unwrap());
        a.add_atom(
            Atom::new(false, 0, "0", "ATOM", 0.0, 0.0, 0.0, 0.0, 0.0, "C", 0).unwrap(),
            "A",
            (0, None),
            ("ALA", None),
        );
        // Test if changing properties for each element of the hierarchy is possible
        for mut hierarchy in a.atoms_with_hierarchy_mut() {
            let _ = hierarchy.residue().serial_number();
            hierarchy.chain_mut().set_id("B");
            let _ = hierarchy.residue().serial_number(); // Show that multiple borrows is valid, as long as they do not overlap
            hierarchy.residue_mut().set_serial_number(1);
            hierarchy.chain_mut().set_id("C");
            hierarchy.conformer_mut().set_name("D");
            hierarchy.atom_mut().set_serial_number(123);
        }
        // Test that casting it to a 'normal' hierarchy works (needs some 'magic' to get an owned variant)
        let hierarchy = a.atoms_with_hierarchy_mut().next().unwrap();
        assert_eq!(hierarchy.without_mut().chain().id(), "C");

        // Test that all changes were properly executed
        assert_eq!(a.chain(0).unwrap().id(), "C");
        assert_eq!(a.residue(0).unwrap().serial_number(), 1);
        assert_eq!(a.conformer(0).unwrap().name(), "D");
        assert_eq!(a.atom(0).unwrap().serial_number(), 123);
    }
}
