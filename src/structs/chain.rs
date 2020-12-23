#![allow(dead_code)]
use crate::structs::*;

#[derive(Debug)]
pub struct Chain {
    pub id: char,
    residues: Vec<Residue>,
}

impl Chain {
    pub fn new(id: Option<char>) -> Chain {
        let mut c = 'a';
        if let Some(ch) = id {
            c = ch;
        }
        Chain {
            id: c,
            residues: Vec::new(),
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
        let ptr: *mut Chain = self;
        let mut found = false;
        let mut new_residue = Residue::new(residue_serial_number, Some(residue_name), None, None);
        let mut current_residue = &mut new_residue;
        for residue in &mut self.residues {
            if residue.serial_number() == residue_serial_number {
                current_residue = residue;
                found = true;
                break;
            }
        }

        current_residue.add_atom(new_atom);
        current_residue.set_chain_pointer(ptr);

        if !found {
            self.residues.push(new_residue)
        }
    }
}
