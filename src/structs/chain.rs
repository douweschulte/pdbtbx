#![allow(dead_code)]
use crate::structs::*;

#[derive(Debug)]
pub struct Chain {
    id: char,
    residues: Vec<Residue>,
    model: Option<*mut Model>,
}

impl Chain {
    pub fn new(id: Option<char>, model: Option<&mut Model>) -> Option<Chain> {
        let mut c = 'a';
        if let Some(ch) = id {
            if !check_char(ch) {
                return None;
            }
            c = ch;
        }
        let mut c = Chain {
            id: c,
            residues: Vec::new(),
            model: None,
        };

        if let Some(m) = model {
            c.model = Some(m);
        }

        Some(c)
    }

    pub fn id(&self) -> char {
        self.id
    }

    pub fn set_id(&mut self, new_id: char) -> bool {
        if check_char(new_id) {
            self.id = new_id;
            true
        } else {
            false
        }
    }

    pub fn residues(&self) -> Vec<&Residue> {
        let mut output = Vec::new();

        for residue in &self.residues {
            output.push(residue);
        }

        output
    }

    pub fn residues_mut(&mut self) -> Vec<&mut Residue> {
        let mut output = Vec::new();

        for residue in &mut self.residues {
            output.push(residue);
        }

        output
    }

    pub fn atoms(&self) -> Vec<&Atom> {
        let mut output = Vec::new();

        for residue in &self.residues {
            output.append(&mut residue.atoms());
        }

        output
    }

    pub fn atoms_mut(&mut self) -> Vec<&mut Atom> {
        let mut output = Vec::new();

        for residue in &mut self.residues {
            output.append(&mut residue.atoms_mut());
        }

        output
    }

    pub fn add_atom(
        &mut self,
        new_atom: Atom,
        residue_serial_number: usize,
        residue_name: [char; 3],
    ) {
        let mut found = false;
        let mut new_residue =
            Residue::new(residue_serial_number, Some(residue_name), None, Some(self))
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
            // As this moves the residue the atom should be added later to keep the reference intact
            self.residues.push(new_residue);
            current_residue = self.residues.last_mut().unwrap();
        }

        current_residue.add_atom(new_atom);
    }

    pub fn set_model(&mut self, new_model: &mut Model) {
        self.model = Some(new_model);
    }

    pub fn set_model_pointer(&mut self, new_model: *mut Model) {
        self.model = Some(new_model);
    }

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

    pub fn model_safe(&self) -> Option<&Model> {
        if let Some(reference) = self.model {
            Some(unsafe { &*reference })
        } else {
            None
        }
    }

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

    fn model_mut_safe(&self) -> Option<&mut Model> {
        if let Some(reference) = self.model {
            Some(unsafe { &mut *reference })
        } else {
            None
        }
    }

    pub fn fix_pointers_of_children(&mut self) {
        let reference: *mut Chain = self;
        for res in &mut self.residues {
            res.set_chain_pointer(reference);
            res.fix_pointers_of_children();
        }
    }

    pub fn remove_residue(&mut self, index: usize) {
        self.residues.remove(index);
    }

    pub fn remove_residue_serial_number(&mut self, serial_number: usize) -> bool {
        let index = self
            .residues
            .iter()
            .position(|a| a.serial_number() == serial_number);

        if let Some(i) = index {
            self.remove_residue(i);
            true
        } else {
            false
        }
    }

    pub fn remove_residue_id(&mut self, id: String) -> bool {
        let index = self.residues.iter().position(|a| a.id() == id);

        if let Some(i) = index {
            self.remove_residue(i);
            true
        } else {
            false
        }
    }

    pub fn remove(&mut self) {
        self.model_mut().remove_chain_id(self.id());
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
