#![allow(dead_code)]
use crate::structs::*;
use crate::transformation::*;
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone)]
/// A Residue containing multiple Residues
pub struct Residue {
    /// The serial number of this Residue, can be negative as that is used sometimes. See https://proteopedia.org/wiki/index.php/Unusual_sequence_numbering.
    serial_number: isize,
    /// The insertion code of this Residue, used in conjunction with the serial number to uniquely identify Residues.
    insertion_code: Option<String>,
    /// The list of conformers making up this Residue
    conformers: Vec<Conformer>,
}

impl Residue {
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

    /// The amount of Conformers making up this Residue
    pub fn conformer_count(&self) -> usize {
        self.conformers.len()
    }

    /// Get the amount of Atoms making up this Residue
    pub fn atom_count(&self) -> usize {
        self.conformers().fold(0, |sum, res| res.atom_count() + sum)
    }

    /// Get a specific conformer from list of conformers making up this Residue.
    ///
    /// ## Arguments
    /// * `index` - the index of the conformer
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn conformer(&self, index: usize) -> Option<&Conformer> {
        self.conformers.get(index)
    }

    /// Get a specific conformer as a mutable reference from list of conformers making up this Residue.
    ///
    /// ## Arguments
    /// * `index` - the index of the conformer
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn conformer_mut(&mut self, index: usize) -> Option<&mut Conformer> {
        self.conformers.get_mut(index)
    }

    /// Get a specific Atom from list of Atoms making up this Residue.
    ///
    /// ## Arguments
    /// * `index` - the index of the Atom
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn atom(&self, index: usize) -> Option<&Atom> {
        self.atoms().nth(index)
    }

    /// Get a specific Atom as a mutable reference from list of Atoms making up this Residue.
    ///
    /// ## Arguments
    /// * `index` - the index of the Atom
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn atom_mut(&mut self, index: usize) -> Option<&mut Atom> {
        self.atoms_mut().nth(index)
    }

    /// Get the list of conformers making up this Residue.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn conformers(&self) -> impl DoubleEndedIterator<Item = &Conformer> + '_ {
        self.conformers.iter()
    }

    /// Get the list of conformers as mutable references making up this Residue.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn conformers_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Conformer> + '_ {
        self.conformers.iter_mut()
    }

    /// Get the list of Atoms making up this Residue.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.conformers.iter().flat_map(|a| a.atoms())
    }

    /// Get the list of Atoms as mutable references making up this Residue.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn atoms_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.conformers.iter_mut().flat_map(|a| a.atoms_mut())
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
        for conformer in self.conformers_mut() {
            conformer.remove_atoms_by(&predicate);
        }
    }

    /// Remove the conformer specified.
    ///
    /// ## Arguments
    /// * `index` - the index of the conformer to remove
    ///
    /// ## Panics
    /// It panics when the index is outside bounds.
    pub fn remove_conformer(&mut self, index: usize) {
        self.conformers.remove(index);
    }

    /// Remove the conformer specified. It returns `true` if it found a matching conformer and removed it.
    /// It removes the first matching conformer from the list.
    ///
    /// ## Arguments
    /// * `id` - the identifying construct of the Conformer to remove
    ///
    /// ## Panics
    /// It panics when the index is outside bounds.
    pub fn remove_conformer_by_id(&mut self, id: (&str, Option<&str>)) -> bool {
        let index = self.conformers.iter().position(|a| a.id() == id);

        if let Some(i) = index {
            self.remove_conformer(i);
            true
        } else {
            false
        }
    }

    /// Apply a transformation to the position of all conformers making up this Residue, the new position is immediately set.
    pub fn apply_transformation(&mut self, transformation: &TransformationMatrix) {
        for conformer in self.conformers_mut() {
            conformer.apply_transformation(transformation);
        }
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

impl PartialEq for Residue {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id() && self.conformers == other.conformers
    }
}

impl PartialOrd for Residue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.id().cmp(&other.id()))
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

    #[test]
    fn check_display() {
        let a = Residue::new(1, None, None).unwrap();
        format!("{:?}", a);
        format!("{}", a);
    }
}
