#![allow(dead_code)]
use crate::structs::*;
use doc_cfg::doc_cfg;
#[cfg(feature = "rayon")]
use rayon::prelude::*;
use std::cmp::Ordering;
use std::fmt;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
/// A Residue containing multiple Residues
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
    /// It fails if any of the characters making up the insertion_code are invalid.
    pub fn new(
        number: isize,
        insertion_code: Option<&str>,
        conformer: Option<Conformer>,
    ) -> Option<Residue> {
        let mut res = Residue {
            serial_number: number,
            insertion_code: None,
            conformers: Vec::new(),
        };
        if let Some(ic) = insertion_code {
            if !valid_identifier(ic) {
                return None;
            }
            res.set_insertion_code(ic);
        }

        if let Some(c) = conformer {
            res.conformers.push(c);
        }

        Some(res)
    }

    /// The serial number of the Residue
    pub fn serial_number(&self) -> isize {
        self.serial_number
    }

    /// Set the serial number of the Residue
    pub fn set_serial_number(&mut self, new_number: isize) {
        self.serial_number = new_number;
    }

    /// The insertion code of the Residue
    pub fn insertion_code(&self) -> Option<&str> {
        self.insertion_code.as_deref()
    }

    /// Set the insertion code of the Residue
    /// It returns false if the `new_code` contains invalid characters
    pub fn set_insertion_code(&mut self, new_code: &str) -> bool {
        if let Some(c) = prepare_identifier(new_code) {
            self.insertion_code = Some(c);
            true
        } else {
            false
        }
    }

    /// Set the insertion code of the Residue to None
    pub fn remove_insertion_code(&mut self) {
        self.insertion_code = None;
    }

    /// Returns the uniquely identifying construct for this Residue.
    /// It consists of the serial number and the insertion code.
    pub fn id(&self) -> (isize, Option<&str>) {
        (self.serial_number, self.insertion_code())
    }

    /// The ID or name of the Residue, it will only give a value if there is only one conformer or if all conformers have the same name
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

    /// Get the specified atom, its uniqueness is guaranteed by including the
    /// alternative_location, with its full hierarchy. The algorithm is based
    /// on binary search so it is faster than an exhaustive search, but the
    /// full structure is assumed to be sorted. This assumption can be enforced
    /// by using `pdb.full_sort()`.
    pub fn binary_find_atom(
        &'a self,
        serial_number: usize,
        alternative_location: Option<&str>,
    ) -> Option<hierarchy::AtomConformer<'a>> {
        for conformer in self.conformers() {
            if conformer.alternative_location() == alternative_location {
                if let Some(f) = conformer.atoms().next() {
                    if let Some(b) = conformer.atoms().next_back() {
                        if f.serial_number() <= serial_number && serial_number <= b.serial_number()
                        {
                            if let Some(atom) = conformer.binary_find_atom(serial_number) {
                                return Some(hierarchy::AtomConformer::new(atom, conformer));
                            }
                        }
                    }
                }
            }
        }
        None
    }

    /// Get the specified atom, its uniqueness is guaranteed by including the
    /// alternative_location, with its full hierarchy. The algorithm is based
    /// on binary search so it is faster than an exhaustive search, but the
    /// full structure is assumed to be sorted. This assumption can be enforced
    /// by using `pdb.full_sort()`.
    #[allow(clippy::unwrap_used)]
    pub fn binary_find_atom_mut(
        &'a mut self,
        serial_number: usize,
        alternative_location: Option<&str>,
    ) -> Option<hierarchy::AtomConformerMut<'a>> {
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
                                    return Some(hierarchy::AtomConformerMut::new(atom, c_ptr));
                                }
                            }
                        }
                    }
                }
            }
            None
        }
    }

    /// Returns all atom with their hierarchy struct for each atom in this residue.
    pub fn atoms_with_hierarchy(
        &'a self,
    ) -> impl DoubleEndedIterator<Item = hierarchy::AtomConformer<'a>> + '_ {
        self.conformers()
            .flat_map(|c| c.atoms().map(move |a| (a, c)))
            .map(hierarchy::AtomConformer::form_tuple)
    }

    /// Returns all atom with their hierarchy struct for each atom in this residue.
    #[allow(trivial_casts)]
    pub fn atoms_with_hierarchy_mut(
        &'a mut self,
    ) -> impl DoubleEndedIterator<Item = hierarchy::AtomConformerMut<'a>> + '_ {
        self.conformers_mut()
            .flat_map(|c| {
                let conformer: *mut Conformer = c;
                c.atoms_mut().map(move |a| (a as *mut Atom, conformer))
            })
            .map(hierarchy::AtomConformerMut::form_tuple)
    }

    /// Add a new conformer to the list of conformers making up this Residue.
    /// ## Arguments
    /// * `new_conformer` - the new conformer to add
    pub fn add_conformer(&mut self, new_conformer: Conformer) {
        self.conformers.push(new_conformer);
    }

    /// Add a new Atom to this Residue. It finds if there already is a Residue with the given serial number if there is it will add this atom to that Residue, otherwise it will create a new Residue and add that to the list of Residues making up this Chain.
    ///
    /// ## Arguments
    /// * `new_atom` - the new Atom to add
    /// * `residue_serial_number` - the serial number of the Residue to add the Atom to
    /// * `residue_name` - the name of the Residue to add the Atom to, only used to create a new Residue if needed
    ///
    /// ## Panics
    /// It panics if the Residue name contains any invalid characters.
    pub fn add_atom(&mut self, new_atom: Atom, conformer_id: (&str, Option<&str>)) {
        let mut found = false;
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

    /// Join this Residue with another Residue, this moves all conformers from the other Residue
    /// to this Residue. All other (meta) data of this Residue will stay the same.
    pub fn join(&mut self, other: Residue) {
        self.conformers.extend(other.conformers);
    }

    /// Extend the conformers on this Residue by the given iterator.
    pub fn extend<T: IntoIterator<Item = Conformer>>(&mut self, iter: T) {
        self.conformers.extend(iter);
    }
}

impl<'a> ContainsAtoms<'a> for Residue {
    type ParallelAtom =
        rayon::iter::FlatMap<rayon::slice::Iter<'a, Conformer>, dyn Fn(&Atom) -> ()>;
    type ParallelAtomMut = usize;

    fn atom_count(&self) -> usize {
        self.conformers().fold(0, |sum, res| res.atom_count() + sum)
    }

    fn atom(&self, index: usize) -> Option<&Atom> {
        self.atoms().nth(index)
    }

    fn atom_mut(&mut self, index: usize) -> Option<&mut Atom> {
        self.atoms_mut().nth(index)
    }

    fn atoms(&self) -> Box<dyn DoubleEndedIterator<Item = &Atom> + '_> {
        Box::new(self.conformers().flat_map(|a| a.atoms()))
    }

    #[doc_cfg(feature = "rayon")]
    fn par_atoms(&'a self) -> Self::ParallelAtom {
        self.par_conformers().flat_map(|a| a.par_atoms())
    }

    fn atoms_mut(&mut self) -> Box<dyn DoubleEndedIterator<Item = &mut Atom> + '_> {
        Box::new(self.conformers_mut().flat_map(|a| a.atoms_mut()))
    }

    #[doc_cfg(feature = "rayon")]
    fn par_atoms_mut(&'a mut self) -> Self::ParallelAtomMut {
        self.par_conformers_mut().flat_map(|a| a.par_atoms_mut())
    }

    fn remove_atoms_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Atom) -> bool,
    {
        self.conformers_mut()
            .for_each(|conformer| conformer.remove_atoms_by(&predicate))
    }

    fn sort(&mut self) {
        self.conformers.sort();
    }

    #[doc_cfg(feature = "rayon")]
    fn par_sort(&mut self) {
        self.conformers.par_sort();
    }
}

impl<'a> ContainsConformers<'a> for Residue {
    type ParallelConformer = rayon::slice::Iter<'a, Conformer>;
    type ParallelConformerMut = rayon::slice::IterMut<'a, Conformer>;

    fn conformer_count(&self) -> usize {
        self.conformers.len()
    }
    fn conformer(&self, index: usize) -> Option<&Conformer> {
        self.conformers.get(index)
    }
    fn conformer_mut(&mut self, index: usize) -> Option<&mut Conformer> {
        self.conformers.get_mut(index)
    }
    fn conformers(&self) -> Box<dyn DoubleEndedIterator<Item = &Conformer> + '_> {
        Box::new(self.conformers.iter())
    }

    #[doc_cfg(feature = "rayon")]
    fn par_conformers(&'a self) -> Self::ParallelConformer {
        self.conformers.par_iter()
    }

    fn conformers_mut(&mut self) -> Box<dyn DoubleEndedIterator<Item = &mut Conformer> + '_> {
        Box::new(self.conformers.iter_mut())
    }

    #[doc_cfg(feature = "rayon")]
    fn par_conformers_mut(&'a mut self) -> Self::ParallelConformerMut {
        self.conformers.par_iter_mut()
    }

    fn remove_conformers_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Conformer) -> bool,
    {
        self.conformers.retain(|conformer| !predicate(conformer));
    }
}

impl fmt::Display for Residue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RESIDUE Number:{}, InsertionCode:{:?}, Conformers:{}",
            self.serial_number(),
            self.insertion_code(),
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
        a.remove_conformer_by_id(("B", None));
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

    #[test]
    fn check_display() {
        let a = Residue::new(1, None, None).unwrap();
        format!("{:?}", a);
        format!("{}", a);
    }
}
