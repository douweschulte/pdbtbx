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
    /// The parent Model of this Chain, if available
    model: Option<*mut Model>,
}

impl Chain {
    /// Create a new Chain
    ///
    /// ## Arguments
    /// * `id` - the identifier
    /// * `model` - if available the parent of the Chain
    ///
    /// ## Fails
    /// It fails if the identifier is an invalid character.
    pub fn new(id: char, model: Option<*mut Model>) -> Option<Chain> {
        if !check_char(id as u8) {
            return None;
        }
        Some(Chain {
            id,
            residues: Vec::new(),
            model,
        })
    }

    /// The ID of the Chain
    pub fn id(&self) -> char {
        self.id as char
    }

    /// Set the ID of the Chain, returns `false` if the new id is an invalid character.
    pub fn set_id(&mut self, new_id: char) -> bool {
        if check_char(new_id as u8) {
            self.id = new_id;
            true
        } else {
            false
        }
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
        residue_name: [u8; 3],
    ) {
        let mut found = false;
        let mut new_residue = Residue::new(residue_serial_number, residue_name, None, Some(self))
            .expect("Invalid chars in Residue creation");
        let mut current_residue = &mut new_residue;
        for residue in &mut self.residues {
            if residue.serial_number() == residue_serial_number {
                current_residue = residue;
                found = true;
                break;
            }
        }
        if !found {
            self.residues.push(new_residue);
            current_residue = self.residues.last_mut().unwrap();
        }

        current_residue.add_atom(new_atom);
    }

    /// Add a Residue to the list of Residues making up this Chain. This does not detect any duplicates of names or serial numbers in the list of Residues.
    fn add_residue(&mut self, mut residue: Residue) {
        residue.set_chain(self);
        self.residues.push(residue);
    }

    /// Set the parent Model. This is used to link back to the parent to read its properties.
    /// This function should only be used when you are sure what you do, in normal cases it is not needed.
    pub fn set_model(&mut self, new_model: &mut Model) {
        self.model = Some(new_model);
    }

    /// Set the parent Model. This is used to link back to the parent to read its properties.
    /// This function should only be used when you are sure what you do, in normal cases it is not needed.
    pub fn set_model_pointer(&mut self, new_model: *mut Model) {
        self.model = Some(new_model);
    }

    /// Get the parent Model.
    /// ## Panics
    /// It panics if there is no parent Model set.
    pub fn model(&self) -> &Model {
        if let Some(reference) = self.model {
            unsafe { &*reference }
        } else {
            panic!(format!(
                "No value for model parent for the current chain {}",
                self.id
            ))
        }
    }

    /// Get the parent Model.
    /// ## Fails
    /// It fails if there is no parent Model set.
    pub fn model_safe(&self) -> Option<&Model> {
        if let Some(reference) = self.model {
            Some(unsafe { &*reference })
        } else {
            None
        }
    }

    /// Get the parent Model mutably, pretty unsafe so you need to make sure yourself the use case is correct.
    /// ## Panics
    /// It panics if there is no parent Model set.
    #[allow(clippy::mut_from_ref)]
    fn model_mut(&self) -> &mut Model {
        if let Some(reference) = self.model {
            unsafe { &mut *reference }
        } else {
            panic!(format!(
                "No value for model parent for the current chain {}",
                self.id
            ))
        }
    }

    /// Get the parent Model mutably, pretty unsafe so you need to make sure yourself the use case is correct.
    /// ## Fails
    /// It fails if there is no parent Model set.
    fn model_mut_safe(&self) -> Option<&mut Model> {
        if let Some(reference) = self.model {
            Some(unsafe { &mut *reference })
        } else {
            None
        }
    }

    /// This sets the parent of all structs contained by this Chain.
    /// This should not be needed to run as a user of the library.
    pub fn fix_pointers_of_children(&mut self) {
        let reference: *mut Chain = self;
        for res in &mut self.residues {
            res.set_chain_pointer(reference);
            res.fix_pointers_of_children();
        }
    }

    pub fn remove_residues_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Residue) -> bool,
    {
        let residues = std::mem::take(&mut self.residues);
        self.residues
            .extend(residues.into_iter().filter(|residue| !predicate(residue)));
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

    /// Remove this Chain from its parent Model
    pub fn remove(&mut self) {
        self.model_mut().remove_chain_id(self.id());
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
        for atom in other.atoms() {
            self.add_atom(
                atom.clone(),
                atom.residue().serial_number(),
                atom.residue().id_array(),
            )
        }
        self.fix_pointers_of_children();
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
        let mut chain = Chain::new(self.id, None).unwrap();

        for residue in self.residues() {
            chain.add_residue(residue.clone());
        }
        chain.fix_pointers_of_children();
        chain
    }
}

impl PartialEq for Chain {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id() && self.residues == other.residues
    }
}
