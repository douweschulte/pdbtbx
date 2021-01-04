#![allow(dead_code)]
use crate::reference_tables;
use crate::structs::*;
use crate::transformation::*;
use std::fmt;

#[derive(Debug)]
/// A Residue containing multiple atoms
pub struct Residue {
    /// The identifier or name of Residue
    id: [u8; 3],
    /// The serial number of this Residue
    serial_number: usize,
    /// The list of atoms making up this Residue
    atoms: Vec<Atom>,
    /// The parent chain of this Residue if available
    chain: Option<*mut Chain>,
}

impl Residue {
    /// Create a new Residue
    ///
    /// ## Arguments
    /// * `number` - the serial number
    /// * `name` - the name or id
    /// * `atom` - if available it can already add an atom
    /// * `chain` - if available the parent of the Residue
    ///
    /// ## Fails
    /// It fails if any of the characters making up the name are invalid.
    pub fn new(
        number: usize,
        name: [u8; 3],
        atom: Option<Atom>,
        chain: Option<*mut Chain>,
    ) -> Option<Residue> {
        if !check_chars(&name) {
            return None;
        }

        let mut res = Residue {
            id: name,
            serial_number: number,
            atoms: Vec::new(),
            chain: None,
        };

        if let Some(mut a) = atom {
            a.set_residue(&mut res);
            res.atoms.push(a);
        }

        if let Some(c) = chain {
            res.chain = Some(c);
        }
        Some(res)
    }

    /// The ID or name of the Residue
    pub fn id(&self) -> String {
        if &self.id != b"   " {
            std::str::from_utf8(&self.id).unwrap().to_owned()
        } else {
            " ".to_string()
        }
    }

    /// The ID or name of the Residue, as a char array
    pub fn id_array(&self) -> [u8; 3] {
        self.id
    }

    /// Set the ID or name of the Residue
    ///
    /// ## Fails
    /// It fails if any of the characters of the new name are invalid. It also fails if the new name is longer than allowed,
    /// the max length is 3 characters.
    pub fn set_id(&mut self, new_id: &str) -> Result<(), String> {
        let bytes = new_id.as_bytes();
        if bytes.len() <= 3 {
            if !check_chars(&bytes) {
                self.id = [bytes[0], bytes[1], bytes[2]];
                Ok(())
            } else {
                Err(format!(
                    "New id has invalid characters for residue {} name {}",
                    self.serial_number, new_id
                ))
            }
        } else {
            Err(format!(
                "New id is too long (max 3 chars) for residue {} name {}",
                self.serial_number, new_id
            ))
        }
    }

    /// The serial number of the Residue
    pub fn serial_number(&self) -> usize {
        self.serial_number
    }

    /// Set the serial number of the Residue
    pub fn set_serial_number(&mut self, new_number: usize) {
        self.serial_number = new_number;
    }

    /// The amount of atoms making up this Residue
    pub fn atom_count(&self) -> usize {
        self.atoms.len()
    }

    /// Get a specific atom from list of atoms making up this Residue.
    ///
    /// ## Arguments
    /// * `index` - the index of the atom
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn atom(&self, index: usize) -> Option<&Atom> {
        self.atoms.get(index)
    }

    /// Get a specific atom as a mutable reference from list of atoms making up this Residue.
    ///
    /// ## Arguments
    /// * `index` - the index of the atom
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn atom_mut(&mut self, index: usize) -> Option<&mut Atom> {
        self.atoms.get_mut(index)
    }

    /// Get the list of atoms making up this Residue.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.atoms.iter()
    }

    /// Get the list of atoms as mutable references making up this Residue.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn atoms_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.atoms.iter_mut()
    }

    /// Add a new atom to the list of atoms making up this Residue.
    /// ## Arguments
    /// * `new_atom` - the new Atom to add
    pub fn add_atom(&mut self, mut new_atom: Atom) {
        new_atom.set_residue(self);
        self.atoms.push(new_atom);
    }

    /// Returns if this Residue is an amino acid
    pub fn amino_acid(&self) -> bool {
        reference_tables::get_amino_acid_number(self.id().as_str()).is_some()
    }

    /// Set the parent Chain. This is used to link back to the parent to read its properties.
    /// This function should only be used when you are sure what you do, in normal cases it is not needed.
    pub fn set_chain(&mut self, new_chain: &mut Chain) {
        self.chain = Some(new_chain);
    }

    /// Set the parent Chain. This is used to link back to the parent to read its properties.
    /// This function should only be used when you are sure what you do, in normal cases it is not needed.
    pub fn set_chain_pointer(&mut self, new_chain: *mut Chain) {
        self.chain = Some(new_chain);
    }

    /// Get the parent Chain.
    /// ## Panics
    /// It panics if there is no parent Chain set.
    pub fn chain(&self) -> &Chain {
        if let Some(reference) = self.chain {
            unsafe { &*reference }
        } else {
            panic!(format!(
                "No value for chain parent for the current residue {}",
                self.serial_number
            ))
        }
    }

    /// Get the parent Chain.
    /// ## Fails
    /// It fails if there is no parent Chain set.
    pub fn chain_safe(&self) -> Option<&Chain> {
        if let Some(reference) = self.chain {
            Some(unsafe { &*reference })
        } else {
            None
        }
    }

    /// Get the parent Chain mutably, pretty unsafe so you need to make sure yourself the use case is correct.
    /// ## Panics
    /// It panics if there is no parent Chain set.
    fn chain_mut(&mut self) -> &mut Chain {
        if let Some(reference) = self.chain {
            unsafe { &mut *reference }
        } else {
            panic!(format!(
                "No value for chain parent for the current residue {}",
                self.serial_number
            ))
        }
    }

    /// Get the parent Chain mutably, pretty unsafe so you need to make sure yourself the use case is correct.
    /// ## Fails
    /// It fails if there is no parent Chain set.
    fn chain_mut_safe(&self) -> Option<&mut Chain> {
        if let Some(reference) = self.chain {
            Some(unsafe { &mut *reference })
        } else {
            None
        }
    }

    /// This sets the parent Residue of the atoms making up this Residue to this Residue.
    /// This should not be needed to run as a user of the library.
    pub fn fix_pointers_of_children(&mut self) {
        let reference: *mut Residue = self;
        for atom in &mut self.atoms {
            atom.set_residue_pointer(reference);
        }
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
    pub fn remove_atom_serial_number(&mut self, serial_number: usize) -> bool {
        let index = self
            .atoms
            .iter()
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
    pub fn remove_atom_name(&mut self, name: String) -> bool {
        let index = self.atoms.iter().position(|a| a.name() == name);

        if let Some(i) = index {
            self.remove_atom(i);
            true
        } else {
            false
        }
    }

    /// Remove this Residue from its parent Chain
    pub fn remove(&mut self) {
        let i = self.serial_number();
        self.chain_mut().remove_residue_serial_number(i);
    }

    /// Apply a transformation to the position of all atoms making up this Residue, the new position is immediately set.
    pub fn apply_transformation(&mut self, transformation: &TransformationMatrix) {
        for atom in self.atoms_mut() {
            atom.apply_transformation(transformation);
        }
    }

    /// Join this Residue with another Residue, this moves all atoms from the other Residue
    /// to this Residue. All other (meta) data of this Residue will stay the same.
    pub fn join(&mut self, other: Residue) {
        for atom in other.atoms {
            self.add_atom(atom)
        }
        self.fix_pointers_of_children();
    }
}

impl fmt::Display for Residue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RESIDUE ID:{}, Number:{}, Atoms:{}",
            self.id(),
            self.serial_number(),
            self.atoms.len(),
        )
    }
}

impl Clone for Residue {
    fn clone(&self) -> Self {
        let mut res = Residue::new(self.serial_number, self.id, None, None).unwrap();

        for atom in self.atoms() {
            res.add_atom(atom.clone());
        }
        res.fix_pointers_of_children();
        res
    }
}

impl PartialEq for Residue {
    fn eq(&self, other: &Self) -> bool {
        self.serial_number == other.serial_number
            && self.id() == other.id()
            && self.atoms == other.atoms
    }
}
