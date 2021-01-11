#![allow(dead_code)]
use crate::reference_tables;
use crate::structs::*;
use crate::transformation::*;
use std::fmt;

#[derive(Debug)]
/// A Residue containing multiple atoms
pub struct Residue {
    /// The identifier or name of Residue
    id: [char; 3],
    /// The serial number of this Residue
    serial_number: usize,
    /// The list of atoms making up this Residue
    atoms: Vec<Atom>,
}

impl Residue {
    /// Create a new Residue
    ///
    /// ## Arguments
    /// * `number` - the serial number
    /// * `name` - the name or id
    /// * `atom` - if available it can already add an atom
    ///
    /// ## Fails
    /// It fails if any of the characters making up the name are invalid.
    pub fn new(number: usize, name: [char; 3], atom: Option<Atom>) -> Option<Residue> {
        if !check_char3(name) {
            return None;
        }

        let mut res = Residue {
            id: name,
            serial_number: number,
            atoms: Vec::new(),
        };

        if let Some(a) = atom {
            res.atoms.push(a);
        }

        Some(res)
    }

    /// The ID or name of the Residue
    pub fn id(&self) -> String {
        let str_id = self.id.iter().collect::<String>();
        if str_id != "   " {
            str_id.split_whitespace().collect::<String>()
        } else {
            " ".to_string()
        }
    }

    /// The ID or name of the Residue, as a char array
    pub fn id_array(&self) -> [char; 3] {
        self.id
    }

    /// Set the ID or name of the Residue
    ///
    /// ## Fails
    /// It fails if any of the characters of the new name are invalid. It also fails if the new name is longer than allowed,
    /// the max length is 3 characters.
    pub fn set_id(&mut self, new_id: &str) -> Result<(), String> {
        let chars = new_id.to_ascii_uppercase().chars().collect::<Vec<char>>();
        if chars.len() <= 3 {
            if check_chars(new_id.to_string()) {
                self.id = [chars[0], chars[1], chars[2]];
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
    pub fn add_atom(&mut self, new_atom: Atom) {
        self.atoms.push(new_atom);
    }

    /// Returns if this Residue is an amino acid
    pub fn amino_acid(&self) -> bool {
        reference_tables::get_amino_acid_number(self.id().as_str()).is_some()
    }

    /// Remove all Atoms matching the given predicate. As this is done in place this is the fastest way to remove Atoms from this Residue.
    pub fn remove_atoms_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Atom) -> bool,
    {
        let atoms = std::mem::take(&mut self.atoms);
        self.atoms
            .extend(atoms.into_iter().filter(|atom| !predicate(atom)));
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

    /// Apply a transformation to the position of all atoms making up this Residue, the new position is immediately set.
    pub fn apply_transformation(&mut self, transformation: &TransformationMatrix) {
        for atom in self.atoms_mut() {
            atom.apply_transformation(transformation);
        }
    }

    /// Join this Residue with another Residue, this moves all atoms from the other Residue
    /// to this Residue. All other (meta) data of this Residue will stay the same.
    pub fn join(&mut self, other: Residue) {
        self.atoms.extend(other.atoms);
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
        let mut res = Residue::new(self.serial_number, self.id, None).unwrap();
        res.atoms = self.atoms.clone();
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
