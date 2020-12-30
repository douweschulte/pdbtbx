#![allow(dead_code)]
use crate::structs::*;
use crate::transformation::*;

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

    pub fn amount_chains(&self) -> usize {
        self.chains.len()
    }

    pub fn amount_residues(&self) -> usize {
        self.chains()
            .fold(0, |sum, chain| chain.amount_residues() + sum)
    }

    pub fn amount_atoms(&self) -> usize {
        self.chains()
            .fold(0, |sum, chain| chain.amount_atoms() + sum)
    }

    pub fn total_amount_chains(&self) -> usize {
        self.chains.len() + self.hetero_atoms.len()
    }

    pub fn total_amount_residues(&self) -> usize {
        self.all_chains()
            .fold(0, |sum, chain| chain.amount_residues() + sum)
    }

    pub fn total_amount_atoms(&self) -> usize {
        self.all_chains()
            .fold(0, |sum, chain| chain.amount_atoms() + sum)
    }

    pub fn chain(&self, index: usize) -> Option<&Chain> {
        self.all_chains().nth(index)
    }

    pub fn chain_mut(&mut self, index: usize) -> Option<&mut Chain> {
        self.all_chains_mut().nth(index)
    }

    pub fn residue(&self, index: usize) -> Option<&Residue> {
        self.all_residues().nth(index)
    }

    pub fn residue_mut(&mut self, index: usize) -> Option<&mut Residue> {
        self.all_residues_mut().nth(index)
    }

    pub fn atom(&self, index: usize) -> Option<&Atom> {
        self.all_atoms().nth(index)
    }

    pub fn atom_mut(&mut self, index: usize) -> Option<&mut Atom> {
        self.all_atoms_mut().nth(index)
    }

    pub fn chains(&self) -> impl DoubleEndedIterator<Item = &Chain> + '_ {
        self.chains.iter()
    }

    pub fn chains_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Chain> + '_ {
        self.chains.iter_mut()
    }

    pub fn residues(&self) -> impl DoubleEndedIterator<Item = &Residue> + '_ {
        self.chains.iter().map(|a| a.residues()).flatten()
    }

    pub fn residues_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Residue> + '_ {
        self.chains.iter_mut().map(|a| a.residues_mut()).flatten()
    }

    pub fn atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.chains.iter().map(|a| a.atoms()).flatten()
    }

    pub fn atoms_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.chains.iter_mut().map(|a| a.atoms_mut()).flatten()
    }

    pub fn hetero_chains(&self) -> impl DoubleEndedIterator<Item = &Chain> + '_ {
        self.hetero_atoms.iter()
    }

    pub fn hetero_chains_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Chain> + '_ {
        self.hetero_atoms.iter_mut()
    }

    pub fn hetero_residues(&self) -> impl DoubleEndedIterator<Item = &Residue> + '_ {
        self.hetero_atoms.iter().map(|a| a.residues()).flatten()
    }

    pub fn hetero_residues_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Residue> + '_ {
        self.hetero_atoms
            .iter_mut()
            .map(|a| a.residues_mut())
            .flatten()
    }

    pub fn hetero_atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.hetero_atoms.iter().map(|a| a.atoms()).flatten()
    }

    pub fn hetero_atoms_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.hetero_atoms
            .iter_mut()
            .map(|a| a.atoms_mut())
            .flatten()
    }

    pub fn all_chains(&self) -> impl DoubleEndedIterator<Item = &Chain> + '_ {
        self.chains.iter().chain(self.hetero_atoms.iter())
    }

    pub fn all_chains_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Chain> + '_ {
        self.chains.iter_mut().chain(self.hetero_atoms.iter_mut())
    }

    pub fn all_residues(&self) -> impl DoubleEndedIterator<Item = &Residue> + '_ {
        self.chains
            .iter()
            .map(|a| a.residues())
            .flatten()
            .chain(self.hetero_atoms.iter().map(|a| a.residues()).flatten())
    }

    pub fn all_residues_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Residue> + '_ {
        self.chains
            .iter_mut()
            .map(|a| a.residues_mut())
            .flatten()
            .chain(
                self.hetero_atoms
                    .iter_mut()
                    .map(|a| a.residues_mut())
                    .flatten(),
            )
    }

    pub fn all_atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.chains
            .iter()
            .map(|a| a.atoms())
            .flatten()
            .chain(self.hetero_atoms.iter().map(|a| a.atoms()).flatten())
    }

    pub fn all_atoms_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.chains
            .iter_mut()
            .map(|a| a.atoms_mut())
            .flatten()
            .chain(
                self.hetero_atoms
                    .iter_mut()
                    .map(|a| a.atoms_mut())
                    .flatten(),
            )
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

    pub fn add_hetero_atom(
        &mut self,
        new_atom: Atom,
        chain_id: char,
        residue_serial_number: usize,
        residue_name: [char; 3],
    ) {
        let mut found = false;
        let mut new_chain = Chain::new(Some(chain_id), Some(self)).expect(&format!(
            "Invalid characters in chain creation ({})",
            chain_id
        ));
        let mut current_chain = &mut new_chain;
        for chain in &mut self.hetero_atoms {
            if chain.id() == chain_id {
                current_chain = chain;
                found = true;
                break;
            }
        }
        if !found {
            self.hetero_atoms.push(new_chain);
            current_chain = self.hetero_atoms.last_mut().unwrap();
        }

        current_chain.add_atom(new_atom, residue_serial_number, residue_name);
    }

    fn add_chain(&mut self, mut chain: Chain) {
        chain.set_model(self);
        self.chains.push(chain);
    }

    fn add_hetero_chain(&mut self, mut chain: Chain) {
        chain.set_model(self);
        self.hetero_atoms.push(chain);
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

    pub fn apply_transformation(&mut self, transformation: &TransformationMatrix) {
        for atom in self.all_atoms_mut() {
            atom.apply_transformation(transformation);
        }
    }

    pub fn join(&mut self, other: Model) {
        for atom in other.atoms() {
            self.add_atom(
                atom.clone(),
                atom.residue().chain().id(),
                atom.residue().serial_number(),
                atom.residue().id_array(),
            )
        }
        for atom in other.hetero_atoms() {
            self.add_hetero_atom(
                atom.clone(),
                atom.residue().chain().id(),
                atom.residue().serial_number(),
                atom.residue().id_array(),
            )
        }
        self.fix_pointers_of_children();
    }
}

use std::fmt;
impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MODEL SerialNumber:{}, Chains: {}",
            self.serial_number,
            self.chains.len() + self.hetero_atoms.len()
        )
    }
}

impl Clone for Model {
    fn clone(&self) -> Self {
        let mut model = Model::new(Some(self.serial_number), None);

        for chain in self.chains() {
            model.add_chain(chain.clone());
        }

        for chain in self.hetero_chains() {
            model.add_hetero_chain(chain.clone());
        }
        model.fix_pointers_of_children();
        model
    }
}
