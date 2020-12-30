#![allow(dead_code)]
use crate::reference_tables;
use crate::structs::*;
use crate::transformation::*;
use std::fmt;

#[derive(Debug)]
pub struct Residue {
    id: [char; 3],
    serial_number: usize,
    atoms: Vec<Atom>,
    chain: Option<*mut Chain>,
}

impl Residue {
    pub fn new(
        number: usize,
        name: Option<[char; 3]>,
        atom: Option<Atom>,
        chain: Option<&mut Chain>,
    ) -> Option<Residue> {
        let mut res = Residue {
            id: [' ', ' ', ' '],
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

        if let Some(n) = name {
            if !check_char3(n) {
                return None;
            }
            res.id = n;
        }

        Some(res)
    }

    pub fn id(&self) -> String {
        let str_id = self.id.iter().collect::<String>();
        if str_id != "   " {
            str_id.split_whitespace().collect::<String>()
        } else {
            " ".to_string()
        }
    }

    pub fn id_array(&self) -> [char; 3] {
        self.id
    }

    pub fn set_id(&mut self, new_id: &str) -> Result<(), String> {
        let chars = new_id.to_ascii_uppercase().chars().collect::<Vec<char>>();
        if chars.len() <= 3 {
            if !check_chars(new_id.to_string()) {
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

    pub fn serial_number(&self) -> usize {
        self.serial_number
    }

    pub fn set_serial_number(&mut self, new_number: usize) {
        self.serial_number = new_number;
    }

    pub fn amount_atoms(&self) -> usize {
        self.atoms.len()
    }

    pub fn atom(&self, index: usize) -> Option<&Atom> {
        self.atoms.get(index)
    }

    pub fn atom_mut(&mut self, index: usize) -> Option<&mut Atom> {
        self.atoms.get_mut(index)
    }

    pub fn atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.atoms.iter()
    }

    pub fn atoms_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.atoms.iter_mut()
    }

    pub fn add_atom(&mut self, mut new_atom: Atom) {
        new_atom.set_residue(self);
        self.atoms.push(new_atom);
    }

    pub fn amino_acid(&self) -> bool {
        if reference_tables::get_amino_acid_number(self.id().as_str()).is_some() {
            true
        } else {
            false
        }
    }

    pub fn set_chain(&mut self, new_chain: &mut Chain) {
        self.chain = Some(new_chain);
    }

    pub fn set_chain_pointer(&mut self, new_chain: *mut Chain) {
        self.chain = Some(new_chain);
    }

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

    pub fn chain_safe(&self) -> Option<&Chain> {
        if let Some(reference) = self.chain {
            Some(unsafe { &*reference })
        } else {
            None
        }
    }

    fn chain_mut(&self) -> &mut Chain {
        if let Some(reference) = self.chain {
            unsafe { &mut *reference }
        } else {
            panic!(format!(
                "No value for chain parent for the current residue {}",
                self.serial_number
            ))
        }
    }

    fn chain_mut_safe(&self) -> Option<&mut Chain> {
        if let Some(reference) = self.chain {
            Some(unsafe { &mut *reference })
        } else {
            None
        }
    }

    pub fn fix_pointers_of_children(&mut self) {
        let reference: *mut Residue = self;
        for atom in &mut self.atoms {
            atom.set_residue_pointer(reference);
        }
    }

    pub fn remove_atom(&mut self, index: usize) {
        self.atoms.remove(index);
    }

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

    pub fn remove_atom_name(&mut self, name: String) -> bool {
        let index = self.atoms.iter().position(|a| a.name() == name);

        if let Some(i) = index {
            self.remove_atom(i);
            true
        } else {
            false
        }
    }

    pub fn remove(&mut self) {
        self.chain_mut()
            .remove_residue_serial_number(self.serial_number());
    }

    pub fn apply_transformation(&mut self, transformation: &TransformationMatrix) {
        for atom in self.atoms_mut() {
            atom.apply_transformation(transformation);
        }
    }

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
        let mut res = Residue::new(self.serial_number, Some(self.id), None, None).unwrap();

        for atom in self.atoms() {
            res.add_atom(atom.clone());
        }
        res.fix_pointers_of_children();
        res
    }
}
