#![allow(dead_code)]
use crate::structs::*;

#[derive(Debug)]
pub struct PDB {
    pub remarks: Vec<(usize, String)>,
    pub scale: Option<Scale>,
    pub unit_cell: Option<UnitCell>,
    pub symmetry: Option<Symmetry>,
    pub models: Vec<Model>,
}

impl PDB {
    pub fn new() -> PDB {
        PDB {
            remarks: Vec::new(),
            scale: None,
            unit_cell: None,
            symmetry: None,
            models: Vec::new(),
        }
    }

    pub fn chains(&self) -> Vec<&Chain> {
        let mut output = Vec::new();

        for model in &self.models {
            for chain in model.chains() {
                output.push(chain)
            }
        }

        output
    }

    pub fn chains_mut(&mut self) -> Vec<&mut Chain> {
        let mut output = Vec::new();

        for model in &mut self.models {
            for chain in model.chains_mut() {
                output.push(chain)
            }
        }

        output
    }

    pub fn residues(&self) -> Vec<&Residue> {
        let mut output = Vec::new();

        for model in &self.models {
            output.append(&mut model.residues())
        }

        output
    }

    pub fn residues_mut(&mut self) -> Vec<&mut Residue> {
        let mut output = Vec::new();

        for model in &mut self.models {
            output.append(&mut model.residues_mut())
        }

        output
    }

    pub fn atoms(&self) -> Vec<&Atom> {
        let mut output = Vec::new();

        for model in &self.models {
            output.append(&mut model.atoms())
        }

        output
    }

    pub fn atoms_mut(&mut self) -> Vec<&mut Atom> {
        let mut output = Vec::new();

        for model in &mut self.models {
            output.append(&mut model.atoms_mut())
        }

        output
    }

    pub fn hetero_atoms(&self) -> Vec<&Atom> {
        let mut output = Vec::new();

        for model in &self.models {
            output.append(&mut model.hetero_atoms())
        }

        output
    }

    pub fn hetero_atoms_mut(&mut self) -> Vec<&mut Atom> {
        let mut output = Vec::new();

        for model in &mut self.models {
            output.append(&mut model.hetero_atoms_mut())
        }

        output
    }

    pub fn all_atoms(&self) -> Vec<&Atom> {
        let mut output = Vec::new();

        for model in &self.models {
            output.append(&mut model.all_atoms())
        }

        output
    }

    pub fn all_atoms_mut(&mut self) -> Vec<&mut Atom> {
        let mut output = Vec::new();

        for model in &mut self.models {
            output.append(&mut model.all_atoms_mut())
        }

        output
    }

    pub fn scale(&mut self) -> &mut Scale {
        match &mut self.scale {
            Some(s) => s,
            None => panic!("Expected a Scale but it was not in place (it was None)."),
        }
    }

    pub fn fix_pointers_of_children(&mut self) {
        let reference: *mut PDB = self;
        for model in &mut self.models {
            model.set_pdb_pointer(reference);
            model.fix_pointers_of_children();
        }
    }

    pub fn remove_model(&mut self, index: usize) {
        self.models.remove(index);
    }

    pub fn remove_model_serial_number(&mut self, serial_number: usize) -> bool {
        let index = self
            .models
            .iter()
            .position(|a| a.serial_number() == serial_number);

        if let Some(i) = index {
            self.remove_model(i);
            true
        } else {
            false
        }
    }
}

use std::fmt;
impl fmt::Display for PDB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PDB Models: {}", self.models.len())
    }
}
