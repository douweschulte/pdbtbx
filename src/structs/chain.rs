#![allow(dead_code)]
use crate::structs::*;

#[derive(Debug)]
pub struct Chain {
    id: char,
    residues: Vec<Residue>,
}

impl Chain {
    pub fn new(id: Option<char>) -> Option<Chain> {
        let mut c = 'a';
        if let Some(ch) = id {
            if !check_char(ch) {
                return None;
            }
            c = ch;
        }
        Some(Chain {
            id: c,
            residues: Vec::new(),
        })
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

        current_residue.add_atom(new_atom);

        if !found {
            self.residues.push(new_residue);
            // Fix the pointer of the atom
            let n = self.residues.len();
            self.residues[n - 1].fix_pointers_of_children();
        }
    }

    pub fn fix_pointers_of_children(&mut self) {
        let reference: *mut Chain = self;
        for res in &mut self.residues {
            res.set_chain_pointer(reference);
            res.fix_pointers_of_children();
        }
    }
}
