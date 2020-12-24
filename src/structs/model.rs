#![allow(dead_code)]
use crate::structs::*;

#[derive(Debug)]
pub struct Model {
    serial_number: usize,
    chains: Vec<Chain>,
    hetero_atoms: Vec<Chain>,
    pdb: Option<*mut PDB>,
}

impl Model {
    pub fn new(serial_number: Option<usize>, pdb: Option<&mut PDB>) -> Model {
        let mut model = Model {
            serial_number: 0,
            chains: Vec::new(),
            hetero_atoms: Vec::new(),
            pdb: None,
        };

        if let Some(n) = serial_number {
            model.serial_number = n;
        }

        if let Some(p) = pdb {
            model.pdb = Some(p);
        }

        model
    }

    pub fn serial_number(&self) -> usize {
        self.serial_number
    }

    pub fn set_serial_number(&mut self, new_number: usize) {
        self.serial_number = new_number;
    }

    pub fn chains(&self) -> Vec<&Chain> {
        let mut output = Vec::new();

        for chain in &self.chains {
            output.push(chain);
        }

        output
    }

    pub fn chains_mut(&mut self) -> Vec<&mut Chain> {
        let mut output = Vec::new();

        for chain in &mut self.chains {
            output.push(chain);
        }

        output
    }

    pub fn residues(&self) -> Vec<&Residue> {
        let mut output = Vec::new();

        for chain in &self.chains {
            output.append(&mut chain.residues());
        }

        output
    }

    pub fn residues_mut(&mut self) -> Vec<&mut Residue> {
        let mut output = Vec::new();

        for chain in &mut self.chains {
            output.append(&mut chain.residues_mut());
        }

        output
    }

    pub fn atoms(&self) -> Vec<&Atom> {
        let mut output = Vec::new();

        for chain in &self.chains {
            output.append(&mut chain.atoms())
        }

        output
    }

    pub fn atoms_mut(&mut self) -> Vec<&mut Atom> {
        let mut output = Vec::new();

        for chain in &mut self.chains {
            output.append(&mut chain.atoms_mut())
        }

        output
    }

    pub fn add_atom(
        &mut self,
        new_atom: Atom,
        chain_id: char,
        residue_serial_number: usize,
        residue_name: [char; 3],
    ) {
        let mut found = false;
        let mut new_chain =
            Chain::new(Some(chain_id), Some(self)).expect("Invalid characters in chain creation");
        let mut current_chain = &mut new_chain;
        for chain in &mut self.chains {
            if chain.id() == chain_id {
                current_chain = chain;
                found = true;
                break;
            }
        }
        if !found {
            // As this moves the chain the atom should be added later to keep the reference intact
            self.chains.push(new_chain);
            current_chain = (&mut self.chains).last_mut().unwrap();
        }

        current_chain.add_atom(new_atom, residue_serial_number, residue_name);
    }

    pub fn hetero_chains(&self) -> Vec<&Chain> {
        let mut output = Vec::new();

        for chain in &self.hetero_atoms {
            output.push(chain);
        }

        output
    }

    pub fn hetero_chains_mut(&mut self) -> Vec<&mut Chain> {
        let mut output = Vec::new();

        for chain in &mut self.hetero_atoms {
            output.push(chain);
        }

        output
    }

    pub fn hetero_residues(&self) -> Vec<&Residue> {
        let mut output = Vec::new();

        for chain in &self.hetero_atoms {
            output.append(&mut chain.residues());
        }

        output
    }

    pub fn hetero_residues_mut(&mut self) -> Vec<&mut Residue> {
        let mut output = Vec::new();

        for chain in &mut self.hetero_atoms {
            output.append(&mut chain.residues_mut());
        }

        output
    }

    pub fn hetero_atoms(&self) -> Vec<&Atom> {
        let mut output = Vec::new();

        for chain in &self.hetero_atoms {
            output.append(&mut chain.atoms())
        }

        output
    }

    pub fn hetero_atoms_mut(&mut self) -> Vec<&mut Atom> {
        let mut output = Vec::new();

        for chain in &mut self.hetero_atoms {
            output.append(&mut chain.atoms_mut())
        }

        output
    }

    pub fn add_hetero_atom(
        &mut self,
        new_atom: Atom,
        chain_id: char,
        residue_serial_number: usize,
        residue_name: [char; 3],
    ) {
        let mut found = false;
        let mut new_chain =
            Chain::new(Some(chain_id), Some(self)).expect("Invalid characters in chain creation");
        let mut current_chain = &mut new_chain;
        for chain in &mut self.hetero_atoms {
            if chain.id() == chain_id {
                current_chain = chain;
                found = true;
                break;
            }
        }

        current_chain.add_atom(new_atom, residue_serial_number, residue_name);

        if !found {
            self.chains.push(new_chain)
        }
    }

    pub fn all_chains(&self) -> Vec<&Chain> {
        let mut output = Vec::new();

        for chain in &self.chains {
            output.push(chain);
        }
        for chain in &self.hetero_atoms {
            output.push(chain);
        }

        output
    }

    pub fn all_chains_mut(&mut self) -> Vec<&mut Chain> {
        let mut output = Vec::new();

        for chain in &mut self.chains {
            output.push(chain);
        }
        for chain in &mut self.hetero_atoms {
            output.push(chain);
        }

        output
    }

    pub fn all_residues(&self) -> Vec<&Residue> {
        let mut output = Vec::new();

        for chain in &self.chains {
            output.append(&mut chain.residues());
        }
        for chain in &self.hetero_atoms {
            output.append(&mut chain.residues());
        }

        output
    }

    pub fn all_residues_mut(&mut self) -> Vec<&mut Residue> {
        let mut output = Vec::new();

        for chain in &mut self.chains {
            output.append(&mut chain.residues_mut());
        }
        for chain in &mut self.hetero_atoms {
            output.append(&mut chain.residues_mut());
        }

        output
    }

    pub fn all_atoms(&self) -> Vec<&Atom> {
        let mut output = Vec::new();

        for chain in &self.chains {
            output.append(&mut chain.atoms())
        }

        for chain in &self.hetero_atoms {
            output.append(&mut chain.atoms())
        }

        output
    }

    pub fn all_atoms_mut(&mut self) -> Vec<&mut Atom> {
        let mut output = Vec::new();

        for chain in &mut self.chains {
            output.append(&mut chain.atoms_mut())
        }

        for chain in &mut self.hetero_atoms {
            output.append(&mut chain.atoms_mut())
        }

        output
    }

    pub fn set_pdb(&mut self, new_pdb: &mut PDB) {
        self.pdb = Some(new_pdb);
    }

    pub fn set_pdb_pointer(&mut self, new_pdb: *mut PDB) {
        self.pdb = Some(new_pdb);
    }

    pub fn pdb(&self) -> &PDB {
        if let Some(reference) = self.pdb {
            unsafe { &*reference }
        } else {
            panic!(format!(
                "No value for PDB parent for the current model {}",
                self.serial_number
            ))
        }
    }

    pub fn pdb_safe(&self) -> Option<&PDB> {
        if let Some(reference) = self.pdb {
            Some(unsafe { &*reference })
        } else {
            None
        }
    }

    fn pdb_mut(&self) -> &mut PDB {
        if let Some(reference) = self.pdb {
            unsafe { &mut *reference }
        } else {
            panic!(format!(
                "No value for PDB parent for the current model {}",
                self.serial_number
            ))
        }
    }

    fn pdb_mut_safe(&self) -> Option<&mut PDB> {
        if let Some(reference) = self.pdb {
            Some(unsafe { &mut *reference })
        } else {
            None
        }
    }

    pub fn fix_pointers_of_children(&mut self) {
        let reference: *mut Model = self;
        for chain in &mut self.chains {
            chain.set_model_pointer(reference);
            chain.fix_pointers_of_children();
        }
        for chain in &mut self.hetero_atoms {
            chain.set_model_pointer(reference);
            chain.fix_pointers_of_children();
        }
    }

    pub fn remove_chain(&mut self, index: usize) {
        self.chains.remove(index);
    }

    pub fn remove_chain_id(&mut self, id: char) -> bool {
        let index = self.chains.iter().position(|a| a.id() == id);

        if let Some(i) = index {
            self.remove_chain(i);
            true
        } else {
            false
        }
    }

    pub fn remove(&mut self) {
        self.pdb_mut()
            .remove_model_serial_number(self.serial_number());
    }
}

use std::fmt;
impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MODEL SerialNumber:{}, Chains: {}",
            self.serial_number,
            self.chains.len()
        )
    }
}
