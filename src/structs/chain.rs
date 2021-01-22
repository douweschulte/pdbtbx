#![allow(dead_code)]
use crate::structs::*;
use crate::transformation::*;

#[derive(Debug)]
/// A Chain containing multiple Residues
pub struct Chain {
    /// The identifier of this Chain
    id: char,
    /// The Residues making up this Chain
    residues: Vec<Residue>,
    /// A possible reference to a database for this chain
    database_reference: Option<DatabaseReference>,
}

impl Chain {
    /// Create a new Chain
    ///
    /// ## Arguments
    /// * `id` - the identifier
    ///
    /// ## Fails
    /// It fails if the identifier is an invalid character.
    pub fn new(id: char) -> Option<Chain> {
        if !check_char(id) {
            return None;
        }
        Some(Chain {
            id,
            residues: Vec::new(),
            database_reference: None,
        })
    }

    /// The ID of the Chain
    pub fn id(&self) -> char {
        self.id
    }

    /// Set the ID of the Chain, returns `false` if the new id is an invalid character.
    pub fn set_id(&mut self, new_id: char) -> bool {
        if check_char(new_id) {
            self.id = new_id;
            true
        } else {
            false
        }
    }

    /// Get the database reference, if any, for this chain.
    pub fn database_reference(&self) -> Option<&DatabaseReference> {
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

    /// Get the amount of Residues making up this Chain
    pub fn residue_count(&self) -> usize {
        self.residues.len()
    }

    /// Get the amount of Atoms making up this Chain
    pub fn atom_count(&self) -> usize {
        self.residues().fold(0, |sum, res| res.atom_count() + sum)
    }

    /// Get a specific Residue from list of Residues making up this Chain.
    ///
    /// ## Arguments
    /// * `index` - the index of the Residue
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn residue(&self, index: usize) -> Option<&Residue> {
        self.residues.get(index)
    }

    /// Get a specific Residue as a mutable reference from list of Residues making up this Chain.
    ///
    /// ## Arguments
    /// * `index` - the index of the Residue
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn residue_mut(&mut self, index: usize) -> Option<&mut Residue> {
        self.residues.get_mut(index)
    }

    /// Get a specific Atom from the Atoms making up this Chain.
    ///
    /// ## Arguments
    /// * `index` - the index of the Atom
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn atom(&self, index: usize) -> Option<&Atom> {
        self.atoms().nth(index)
    }

    /// Get a specific Atom as a mutable reference from the Atoms making up this Chain.
    ///
    /// ## Arguments
    /// * `index` - the index of the Atom
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn atom_mut(&mut self, index: usize) -> Option<&mut Atom> {
        self.atoms_mut().nth(index)
    }

    /// Get the list of Residues making up this Chain.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn residues(&self) -> impl DoubleEndedIterator<Item = &Residue> + '_ {
        self.residues.iter()
    }

    /// Get the list of Residues as mutable references making up this Chain.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn residues_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Residue> + '_ {
        self.residues.iter_mut()
    }

    /// Get the list of Atoms making up this Chain.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.residues.iter().flat_map(|a| a.atoms())
    }

    /// Get the list of Atoms as mutable references making up this Chain.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn atoms_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.residues.iter_mut().flat_map(|a| a.atoms_mut())
    }

    /// Add a new Atom to this Chain. It finds if there already is a Residue with the given serial number if there is it will add this atom to that Residue, otherwise it will create a new Residue and add that to the list of Residues making up this Chain.
    ///
    /// ## Arguments
    /// * `new_atom` - the new Atom to add
    /// * `residue_serial_number` - the serial number of the Residue to add the Atom to
    /// * `residue_name` - the name of the Residue to add the Atom to, only used to create a new Residue if needed
    ///
    /// ## Panics
    /// It panics if the Residue name contains any invalid characters.
    pub fn add_atom(
        &mut self,
        new_atom: Atom,
        residue_serial_number: usize,
        residue_name: [char; 3],
    ) {
        let mut found = false;
        let mut new_residue = Residue::new(residue_serial_number, residue_name, None)
            .expect("Invalid chars in Residue creation");
        let mut current_residue = &mut new_residue;
        for residue in &mut self.residues {
            if residue.serial_number() == residue_serial_number {
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

        current_residue.add_atom(new_atom);
    }

    /// Add a Residue end of to the list of Residues making up this Chain. This does not detect any duplicates of names or serial numbers in the list of Residues.
    pub fn add_residue(&mut self, residue: Residue) {
        self.residues.push(residue);
    }

    /// Inserts a Residue to the index in the list of Residues making up this Chain. This does not detect any duplicates of names or serial numbers in the list of Residues.
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

    /// Remove all residues matching the given predicate. As this is done in place this is the fastest way to remove Residues from this Chain.
    pub fn remove_residues_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Residue) -> bool,
    {
        self.residues.retain(|residue| !predicate(residue));
    }

    /// Remove the Residue specified.
    ///
    /// ## Arguments
    /// * `index` - the index of the Residue to remove
    ///
    /// ## Panics
    /// It panics when the index is outside bounds.
    pub fn remove_residue_by_id(&mut self, index: usize) {
        self.residues.remove(index);
    }

    /// Remove the Residue specified. It returns `true` if it found a matching Residue and removed it.
    /// It removes the first matching Residue from the list.
    ///
    /// ## Arguments
    /// * `serial_number` - the serial number of the Residue to remove
    pub fn remove_residue_serial_number(&mut self, serial_number: usize) -> bool {
        let index = self
            .residues
            .iter()
            .position(|a| a.serial_number() == serial_number);

        if let Some(i) = index {
            self.remove_residue_by_id(i);
            true
        } else {
            false
        }
    }

    /// Remove the Residue specified. It returns `true` if it found a matching Residue and removed it.
    /// It removes the first matching Residue from the list.
    ///
    /// ## Arguments
    /// * `id` - the id of the Residue to remove  
    pub fn remove_residue_id(&mut self, id: String) -> bool {
        let index = self.residues.iter().position(|a| a.id() == id);

        if let Some(i) = index {
            self.remove_residue_by_id(i);
            true
        } else {
            false
        }
    }

    /// Apply a transformation to the position of all atoms making up this Chain, the new position is immediately set.
    pub fn apply_transformation(&mut self, transformation: &TransformationMatrix) {
        for atom in self.atoms_mut() {
            atom.apply_transformation(transformation);
        }
    }

    /// Join this Chain with another Chain, this moves all atoms from the other Chain
    /// to this Chain. All other (meta) data of this Chain will stay the same.
    pub fn join(&mut self, other: Chain) {
        self.residues.extend(other.residues);
    }
}

use std::fmt;
impl fmt::Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CHAIN ID:{}, Residues: {}",
            self.id(),
            self.residues.len()
        )
    }
}

impl Clone for Chain {
    fn clone(&self) -> Self {
        let mut chain = Chain::new(self.id).expect("Invalid Chain id while cloning a Chain");

        chain.residues = self.residues.clone();
        chain
    }
}

impl PartialEq for Chain {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id() && self.residues == other.residues
    }
}
