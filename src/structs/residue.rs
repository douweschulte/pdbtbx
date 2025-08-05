#![allow(dead_code)]
use crate::structs::*;
use crate::transformation::TransformationMatrix;
#[cfg(feature = "rayon")]
use rayon::prelude::*;
use std::cmp::Ordering;
use std::fmt;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
/// A Residue containing multiple Conformers
pub struct Residue {
    /// The serial number of this Residue, can be negative as that is used sometimes. See <https://proteopedia.org/wiki/index.php/Unusual_sequence_numbering>.
    serial_number: isize,
    /// The insertion code of this Residue, used in conjunction with the serial number to uniquely identify Residues.
    insertion_code: Option<String>,
    /// The list of conformers making up this Residue
    conformers: Vec<Conformer>,
}

impl<'a> Residue {
    /// Create a new Residue
    ///
    /// ## Arguments
    /// * `number` - the serial number
    /// * `insertion_code` - the insertion code
    /// * `conformer` - if available it can already add an conformer
    ///
    /// ## Fails
    /// It fails and returns `None` if any of the characters making up the `insertion_code` are invalid.
    #[must_use]
    pub fn new(
        number: isize,
        insertion_code: Option<&str>,
        conformer: Option<Conformer>,
    ) -> Option<Self> {
        let mut res = Self {
            serial_number: number,
            insertion_code: None,
            conformers: Vec::new(),
        };
        if let Some(ic) = insertion_code {
            if !res.set_insertion_code(ic) {
                return None;
            }
        }

        if let Some(c) = conformer {
            res.conformers.push(c);
        }

        Some(res)
    }

    /// Get the serial number of the Residue.
    #[must_use]
    pub const fn serial_number(&self) -> isize {
        self.serial_number
    }

    /// Set the serial number of the Residue.
    pub fn set_serial_number(&mut self, new_number: isize) {
        self.serial_number = new_number;
    }

    /// Get the insertion code of the Residue.
    #[must_use]
    pub fn insertion_code(&self) -> Option<&str> {
        self.insertion_code.as_deref()
    }

    /// Set the insertion code of the Residue.
    /// Fails and returns false if the `new_code` contains invalid characters
    pub fn set_insertion_code(&mut self, new_code: impl AsRef<str>) -> bool {
        prepare_identifier_uppercase(new_code)
            .map(|c| self.insertion_code = Some(c))
            .is_some()
    }

    /// Set the insertion code of the Residue to None.
    pub fn remove_insertion_code(&mut self) {
        self.insertion_code = None;
    }

    /// Returns the uniquely identifying construct for this Residue,
    /// consisting of the serial number and the insertion code.
    #[must_use]
    pub fn id(&self) -> (isize, Option<&str>) {
        (self.serial_number, self.insertion_code())
    }

    /// The ID or name of the Residue, it will only give a value if there is only one conformer or if all conformers have the same name
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        match self.conformers.len() {
            0 => None,
            1 => Some(self.conformers[0].name()),
            _ => {
                let res = self.conformers[0].name();
                for conf in self.conformers().skip(1) {
                    if res != conf.name() {
                        return None;
                    }
                }
                Some(res)
            }
        }
    }

    /// The number of Conformers making up this Residue.
    #[must_use]
    pub fn conformer_count(&self) -> usize {
        self.conformers.len()
    }

    /// Get the number of Atoms making up this Residue.
    #[must_use]
    pub fn atom_count(&self) -> usize {
        self.conformers().fold(0, |sum, res| res.atom_count() + sum)
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Get the number of Atoms making up this Residue in parallel.
    #[cfg(feature = "rayon")]
    #[must_use]
    pub fn par_atom_count(&self) -> usize {
        self.par_conformers().map(Conformer::atom_count).sum()
    }

    /// Get a reference to a specific Conformer from the list of Conformers making up this Residue.
    ///
    /// ## Arguments
    /// * `index` - the index of the conformer
    ///
    /// ## Fails
    /// Returns `None` if the index is out of bounds.
    #[must_use]
    pub fn conformer(&self, index: usize) -> Option<&Conformer> {
        self.conformers.get(index)
    }

    /// Get a mutable reference to a specific Conformer from the list of Conformers making up this Residue.
    ///
    /// ## Arguments
    /// * `index` - the index of the conformer
    ///
    /// ## Fails
    /// Returns `None` if the index is out of bounds.
    #[must_use]
    pub fn conformer_mut(&mut self, index: usize) -> Option<&mut Conformer> {
        self.conformers.get_mut(index)
    }

    /// Get a reference to a specific Atom from the list of Conformers making up this Residue.
    ///
    /// ## Arguments
    /// * `index` - the index of the Atom
    ///
    /// ## Fails
    /// Returns `None` if the index is out of bounds.
    #[must_use]
    pub fn atom(&self, index: usize) -> Option<&Atom> {
        self.atoms().nth(index)
    }

    /// Get a mutable reference to a specific Atom from the list of Conformers making up this Residue.
    ///
    /// ## Arguments
    /// * `index` - the index of the Atom
    ///
    /// ## Fails
    /// Returns `None` if the index is out of bounds.
    #[must_use]
    pub fn atom_mut(&mut self, index: usize) -> Option<&mut Atom> {
        self.atoms_mut().nth(index)
    }

    /// Get A reference to the specified Atom. Its uniqueness is guaranteed by including the
    /// `insertion_code`, with its full hierarchy. The algorithm is based
    /// on binary search so it is faster than an exhaustive search, but the
    /// full structure is assumed to be sorted. This assumption can be enforced
    /// by using `pdb.full_sort()`.
    #[must_use]
    pub fn binary_find_atom(
        &'a self,
        serial_number: usize,
        alternative_location: Option<&str>,
    ) -> Option<AtomConformer<'a>> {
        for conformer in self.conformers() {
            if conformer.alternative_location() == alternative_location {
                if let Some(f) = conformer.atoms().next() {
                    if let Some(b) = conformer.atoms().next_back() {
                        if f.serial_number() <= serial_number && serial_number <= b.serial_number()
                        {
                            if let Some(atom) = conformer.binary_find_atom(serial_number) {
                                return Some(AtomConformer::new(atom, conformer));
                            }
                        }
                    }
                }
            }
        }
        None
    }

    /// Get a mutable reference to the specified Atom. Its uniqueness is guaranteed by
    /// including the `insertion_code`, with its full hierarchy. The algorithm is based
    /// on binary search so it is faster than an exhaustive search, but the
    /// full structure is assumed to be sorted. This assumption can be enforced
    /// by using `pdb.full_sort()`.
    #[allow(clippy::unwrap_used)]
    #[must_use]
    pub fn binary_find_atom_mut(
        &'a mut self,
        serial_number: usize,
        alternative_location: Option<&str>,
    ) -> Option<AtomConformerMut<'a>> {
        unsafe {
            for c in self.conformers_mut() {
                let c_ptr: *mut Conformer = c;
                let conformer = c_ptr.as_mut().unwrap();
                if conformer.alternative_location() == alternative_location {
                    if let Some(f) = conformer.atoms().next() {
                        if let Some(b) = conformer.atoms().next_back() {
                            if f.serial_number() <= serial_number
                                && serial_number <= b.serial_number()
                            {
                                if let Some(atom) =
                                    c_ptr.as_mut().unwrap().binary_find_atom_mut(serial_number)
                                {
                                    return Some(AtomConformerMut::new(atom, c_ptr));
                                }
                            }
                        }
                    }
                }
            }
            None
        }
    }

    /// Find all hierarchies matching the given search. For more details see [Search].
    #[must_use]
    pub fn find(
        &'a self,
        search: Search,
    ) -> impl DoubleEndedIterator<Item = AtomConformer<'a>> + 'a {
        self.conformers()
            .map(move |c| (c, search.clone().add_conformer_info(c)))
            .filter(|(_c, search)| !matches!(search, Search::Known(false)))
            .flat_map(move |(c, search)| c.find(search).map(move |a| AtomConformer::new(a, c)))
    }

    /// Find all hierarchies matching the given search. For more details see [Search].
    #[must_use]
    pub fn find_mut(
        &'a mut self,
        search: Search,
    ) -> impl DoubleEndedIterator<Item = AtomConformerMut<'a>> + 'a {
        self.conformers_mut()
            .map(move |c| {
                let search = search.clone().add_conformer_info(c);
                (c, search)
            })
            .filter(|(_c, search)| !matches!(search, Search::Known(false)))
            .flat_map(move |(c, search)| {
                let c_ptr: *mut Conformer = c;
                c.find_mut(search)
                    .map(move |a| AtomConformerMut::new(a, c_ptr))
            })
    }

    /// Get an iterator of references to Conformers making up this Model.
    /// Double ended so iterating from the end is just as fast as from the start.
    #[must_use]
    pub fn conformers(&self) -> impl DoubleEndedIterator<Item = &Conformer> + '_ {
        self.conformers.iter()
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Get a parallel iterator of references to Conformers making up this Model.
    #[cfg(feature = "rayon")]
    #[must_use]
    pub fn par_conformers(&self) -> impl ParallelIterator<Item = &Conformer> + '_ {
        self.conformers.par_iter()
    }

    /// Get an iterator of mutable references to Conformers making up this Model.
    /// Double ended so iterating from the end is just as fast as from the start.
    #[must_use]
    pub fn conformers_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Conformer> + '_ {
        self.conformers.iter_mut()
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Get a parallel iterator of mutable references to Conformers making up this Model.
    #[cfg(feature = "rayon")]
    #[must_use]
    pub fn par_conformers_mut(&mut self) -> impl ParallelIterator<Item = &mut Conformer> + '_ {
        self.conformers.par_iter_mut()
    }

    /// Get an iterator of references to Atoms making up this Model.
    /// Double ended so iterating from the end is just as fast as from the start.
    #[must_use]
    pub fn atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.conformers().flat_map(Conformer::atoms)
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Get a parallel iterator of references to Atoms making up this Model.
    #[cfg(feature = "rayon")]
    #[must_use]
    pub fn par_atoms(&self) -> impl ParallelIterator<Item = &Atom> + '_ {
        self.par_conformers().flat_map(Conformer::par_atoms)
    }

    /// Get an iterator of mutable references to Atoms making up this Model.
    /// Double ended so iterating from the end is just as fast as from the start.
    #[must_use]
    pub fn atoms_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.conformers_mut().flat_map(Conformer::atoms_mut)
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Get a parallel iterator of mutable references to Atoms making up this Model.
    #[cfg(feature = "rayon")]
    #[must_use]
    pub fn par_atoms_mut(&mut self) -> impl ParallelIterator<Item = &mut Atom> + '_ {
        self.par_conformers_mut().flat_map(Conformer::par_atoms_mut)
    }

    /// Get an iterator of references to a struct containing all atoms with their hierarchy making up this Model.
    #[must_use]
    pub fn atoms_with_hierarchy(
        &'a self,
    ) -> impl DoubleEndedIterator<Item = AtomConformer<'a>> + 'a {
        self.conformers()
            .flat_map(|c| c.atoms().map(move |a| (a, c)))
            .map(AtomConformer::from_tuple)
    }

    /// Get an iterator of mutable references to a struct containing all atoms with their hierarchy making up this Model.
    #[allow(trivial_casts)]
    #[must_use]
    pub fn atoms_with_hierarchy_mut(
        &'a mut self,
    ) -> impl DoubleEndedIterator<Item = AtomConformerMut<'a>> + 'a {
        self.conformers_mut()
            .flat_map(|c| {
                let conformer: *mut Conformer = c;
                c.atoms_mut().map(move |a| (a as *mut Atom, conformer))
            })
            .map(AtomConformerMut::from_tuple)
    }

    /// Add a new conformer to the list of conformers making up this Residue.
    /// ## Arguments
    /// * `new_conformer` - the new conformer to add
    pub fn add_conformer(&mut self, new_conformer: Conformer) {
        self.conformers.push(new_conformer);
    }

    /// Add a new Atom to this Residue. If a Residue with the given serial number already exists, the
    /// Atom will be added to it, otherwise a new Residue is created to hold the created atom
    /// and added to the list of Residues in its chain.
    ///
    /// ## Arguments
    /// * `new_atom` - the new Atom to add
    /// * `residue_serial_number` - the serial number of the Residue to add the Atom to
    /// * `residue_name` - the name of the Residue to add the Atom to, only used to create a new Residue if needed
    ///
    /// ## Panics
    /// It panics if the Residue name contains any invalid characters.
    pub fn add_atom(&mut self, new_atom: Atom, conformer_id: (impl AsRef<str>, Option<&str>)) {
        let mut found = false;
        let name = prepare_identifier_uppercase(conformer_id.0).expect("Invalid Conformer ID");
        let conformer_id = (name.as_str(), conformer_id.1);
        let mut new_conformer = Conformer::new(conformer_id.0, conformer_id.1, None)
            .expect("Invalid chars in Residue creation");
        let mut current_conformer = &mut new_conformer;
        for conformer in &mut self.conformers {
            if conformer.id() == conformer_id {
                current_conformer = conformer;
                found = true;
                break;
            }
        }
        #[allow(clippy::unwrap_used)]
        if !found {
            self.conformers.push(new_conformer);
            current_conformer = self.conformers.last_mut().unwrap();
        }

        current_conformer.add_atom(new_atom);
    }

    /// Remove all empty Conformers from this Residue.
    pub fn remove_empty(&mut self) {
        self.conformers.retain(|c| c.atom_count() > 0);
    }

    /// Remove all conformers matching the given predicate. As this is done in place this is the fastest way to remove conformers from this Residue.
    pub fn remove_conformers_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Conformer) -> bool,
    {
        self.conformers.retain(|conformer| !predicate(conformer));
    }

    /// Remove all atoms matching the given predicate. As this is done in place this is the fastest way to remove atoms from this Residue.
    pub fn remove_atoms_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Atom) -> bool,
    {
        // for conformer in self.conformers_mut() {
        //     conformer.remove_atoms_by(&predicate);
        // }
        self.conformers_mut()
            .for_each(|conformer| conformer.remove_atoms_by(&predicate));
    }

    /// Remove the specified conformer.
    ///
    /// ## Arguments
    /// * `index` - the index of the Conformer to remove
    ///
    /// ## Panics
    /// Panics when the index is outside bounds.
    pub fn remove_conformer(&mut self, index: usize) {
        self.conformers.remove(index);
    }

    /// Remove the specified conformer. Returns `true` if a matching Conformer was found and
    /// removed.
    /// Removes the first matching Conformer from the list.
    ///
    /// ## Arguments
    /// * `id` - the identifying construct of the Conformer to remove
    ///
    /// ## Panics
    /// Panics when the index is outside bounds.
    pub fn remove_conformer_by_id(&mut self, id: (&str, Option<&str>)) -> bool {
        let index = self.conformers().position(|a| a.id() == id);

        index.map_or(false, |i| {
            self.remove_conformer(i);
            true
        })
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Remove the specified Conformer. Returns `true` if a matching Conformer was found and
    /// removed.
    /// It removes the first matching Conformer from the list. Searching is done in parallel.
    ///
    /// ## Arguments
    /// * `id` - the identifying construct of the Conformer to remove
    ///
    /// ## Panics
    /// Panics when the index is outside bounds.
    #[cfg(feature = "rayon")]
    pub fn par_remove_conformer_by_id(&mut self, id: (&str, Option<&str>)) -> bool {
        let index = self.conformers.par_iter().position_first(|a| a.id() == id);

        index.map_or(false, |i| {
            self.remove_conformer(i);
            true
        })
    }

    /// Apply a transformation to the position of all Conformers making up this Residue, the new position is immediately set.
    pub fn apply_transformation(&mut self, transformation: &TransformationMatrix) {
        for conformer in self.conformers_mut() {
            conformer.apply_transformation(transformation);
        }
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Apply a transformation to the position of all Conformers making up this Residue, the new position is immediately set.
    /// Done in parallel
    #[cfg(feature = "rayon")]
    pub fn par_apply_transformation(&mut self, transformation: &TransformationMatrix) {
        self.par_conformers_mut()
            .for_each(|conformer| conformer.apply_transformation(transformation));
    }

    /// Join this Residue with another Residue, this moves all Conformers from the other Residue
    /// to this Residue. All other (meta) data of this Residue will stay the same.
    pub fn join(&mut self, other: Self) {
        self.conformers.extend(other.conformers);
    }

    /// Sort the Conformers of this Residue
    pub fn sort(&mut self) {
        self.conformers.sort();
    }

    /// <div class="warning">Available on crate feature `rayon` only</div>
    /// Sort the Conformers of this Residue in parallel
    #[cfg(feature = "rayon")]
    pub fn par_sort(&mut self) {
        self.conformers.par_sort();
    }
}

impl fmt::Display for Residue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RESIDUE Number:{}, InsertionCode:{}, Conformers:{}",
            self.serial_number(),
            self.insertion_code().unwrap_or(""),
            self.conformers.len(),
        )
    }
}

impl PartialOrd for Residue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.id().cmp(&other.id()))
    }
}

impl Ord for Residue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id().cmp(&other.id())
    }
}

impl Extend<Conformer> for Residue {
    /// Extend the Conformers on this Residue by the given iterator.
    fn extend<T: IntoIterator<Item = Conformer>>(&mut self, iter: T) {
        self.conformers.extend(iter);
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_text_validation() {
        let mut a = Residue::new(1, Some("A"), None).unwrap();
        assert_eq!(Residue::new(2, Some("Rͦ"), None), None);
        assert!(!a.set_insertion_code("Oͦ"));
        assert_eq!(a.insertion_code(), Some("A"));
        a.set_insertion_code("Conformer");
        assert_eq!(a.insertion_code(), Some("CONFORMER"));
    }

    #[test]
    fn ordering_and_equality() {
        let a = Residue::new(1, None, None).unwrap();
        let b = Residue::new(1, None, None).unwrap();
        let c = Residue::new(2, None, None).unwrap();
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert!(a < c);
        assert!(b < c);
    }

    #[test]
    fn test_empty() {
        let a = Residue::new(1, None, None).unwrap();
        assert_eq!(a.conformer_count(), 0);
    }

    #[test]
    fn test_conformer() {
        let mut a = Residue::new(1, None, None).unwrap();
        let mut conformer1 = Conformer::new("A", None, None).unwrap();
        a.add_conformer(conformer1.clone());
        a.add_conformer(Conformer::new("B", None, None).unwrap());
        assert_eq!(a.conformer(0), Some(&conformer1));
        assert_eq!(a.conformer_mut(0), Some(&mut conformer1));
        a.remove_conformer(0);
        assert!(a.remove_conformer_by_id(("B", None)));
        assert_eq!(a.conformer_count(), 0);
    }

    #[test]
    fn test_join() {
        let mut a = Residue::new(1, None, None).unwrap();
        let mut b = Residue::new(1, None, None).unwrap();
        let conformer1 = Conformer::new("A", None, None).unwrap();
        b.add_conformer(conformer1.clone());

        a.join(b);
        a.extend(vec![conformer1]);

        assert_eq!(a.conformer_count(), 2);
    }
}
