#![allow(dead_code)]
use crate::structs::*;
use crate::transformation::*;

#[derive(Debug)]
pub struct PDB {
    pub remarks: Vec<(usize, String)>,
    pub scale: Option<Scale>,
    pub origx: Option<OrigX>,
    pub mtrix: Vec<MtriX>,
    pub unit_cell: Option<UnitCell>,
    pub symmetry: Option<Symmetry>,
    models: Vec<Model>,
}

impl PDB {
    pub fn new() -> PDB {
        PDB {
            remarks: Vec::new(),
            scale: None,
            origx: None,
            mtrix: Vec::new(),
            unit_cell: None,
            symmetry: None,
            models: Vec::new(),
        }
    }

    pub fn add_model(&mut self, new_model: Model) {
        self.models.push(new_model);
        self.models.last_mut().unwrap().fix_pointers_of_children();
    }

    pub fn amount_models(&self) -> usize {
        self.models.len()
    }

    pub fn amount_chains(&self) -> usize {
        if self.models.len() > 0 {
            self.models[0].amount_chains()
        } else {
            0
        }
    }

    pub fn amount_residues(&self) -> usize {
        if self.models.len() > 0 {
            self.models[0].amount_residues()
        } else {
            0
        }
    }

    pub fn amount_atoms(&self) -> usize {
        if self.models.len() > 0 {
            self.models[0].amount_atoms()
        } else {
            0
        }
    }

    pub fn total_amount_chains(&self) -> usize {
        self.models.len() * self.amount_chains()
    }

    pub fn total_amount_residues(&self) -> usize {
        self.models.len() * self.amount_residues()
    }

    pub fn total_amount_atoms(&self) -> usize {
        self.models.len() * self.amount_atoms()
    }

    pub fn model(&self, index: usize) -> Option<&Model> {
        self.models.get(index)
    }

    pub fn model_mut(&mut self, index: usize) -> Option<&mut Model> {
        self.models.get_mut(index)
    }

    pub fn chain(&self, index: usize) -> Option<&Chain> {
        self.chains().nth(index)
    }

    pub fn chain_mut(&mut self, index: usize) -> Option<&mut Chain> {
        self.chains_mut().nth(index)
    }

    pub fn residue(&self, index: usize) -> Option<&Residue> {
        self.residues().nth(index)
    }

    pub fn residue_mut(&mut self, index: usize) -> Option<&mut Residue> {
        self.residues_mut().nth(index)
    }

    pub fn atom(&self, index: usize) -> Option<&Atom> {
        self.atoms().nth(index)
    }

    pub fn atom_mut(&mut self, index: usize) -> Option<&mut Atom> {
        self.atoms_mut().nth(index)
    }

    pub fn models(&self) -> impl DoubleEndedIterator<Item = &Model> + '_ {
        self.models.iter()
    }

    pub fn models_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Model> + '_ {
        self.models.iter_mut()
    }

    pub fn chains(&self) -> impl DoubleEndedIterator<Item = &Chain> + '_ {
        self.models.iter().map(|a| a.chains()).flatten()
    }

    pub fn chains_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Chain> + '_ {
        self.models.iter_mut().map(|a| a.chains_mut()).flatten()
    }

    pub fn residues(&self) -> impl DoubleEndedIterator<Item = &Residue> + '_ {
        self.models.iter().map(|a| a.residues()).flatten()
    }

    pub fn residues_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Residue> + '_ {
        self.models.iter_mut().map(|a| a.residues_mut()).flatten()
    }

    pub fn atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.models.iter().map(|a| a.atoms()).flatten()
    }

    pub fn atoms_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.models.iter_mut().map(|a| a.atoms_mut()).flatten()
    }

    pub fn hetero_chains(&self) -> impl DoubleEndedIterator<Item = &Chain> + '_ {
        self.models.iter().map(|a| a.hetero_chains()).flatten()
    }

    pub fn hetero_chains_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Chain> + '_ {
        self.models
            .iter_mut()
            .map(|a| a.hetero_chains_mut())
            .flatten()
    }

    pub fn hetero_residues(&self) -> impl DoubleEndedIterator<Item = &Residue> + '_ {
        self.models.iter().map(|a| a.hetero_residues()).flatten()
    }

    pub fn hetero_residues_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Residue> + '_ {
        self.models
            .iter_mut()
            .map(|a| a.hetero_residues_mut())
            .flatten()
    }

    pub fn hetero_atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.models.iter().map(|a| a.hetero_atoms()).flatten()
    }

    pub fn hetero_atoms_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.models
            .iter_mut()
            .map(|a| a.hetero_atoms_mut())
            .flatten()
    }

    pub fn all_chains(&self) -> impl DoubleEndedIterator<Item = &Chain> + '_ {
        self.models.iter().map(|a| a.all_chains()).flatten()
    }

    pub fn all_chains_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Chain> + '_ {
        self.models.iter_mut().map(|a| a.all_chains_mut()).flatten()
    }

    pub fn all_residues(&self) -> impl DoubleEndedIterator<Item = &Residue> + '_ {
        self.models.iter().map(|a| a.all_residues()).flatten()
    }

    pub fn all_residues_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Residue> + '_ {
        self.models
            .iter_mut()
            .map(|a| a.all_residues_mut())
            .flatten()
    }

    pub fn all_atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.models.iter().map(|a| a.all_atoms()).flatten()
    }

    pub fn all_atoms_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.models.iter_mut().map(|a| a.all_atoms_mut()).flatten()
    }

    pub fn scale(&mut self) -> &mut Scale {
        match &mut self.scale {
            Some(s) => s,
            None => panic!("Expected a Scale but it was not in place (it was None)."),
        }
    }

    pub fn origx(&mut self) -> &mut OrigX {
        match &mut self.origx {
            Some(o) => o,
            None => panic!("Expected a OrigX but it was not in place (it was None)."),
        }
    }

    pub fn mtrix(&mut self, index: usize) -> Option<&mut MtriX> {
        self.mtrix.get_mut(index)
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

    pub fn renumber(&mut self) {
        let mut model_counter = 1;
        for model in self.models_mut() {
            model.set_serial_number(model_counter);
            model_counter += 1;

            let mut counter = 1;
            for atom in model.atoms_mut() {
                atom.set_serial_number(counter);
                counter += 1;
            }
            counter = 1;
            for residue in model.residues_mut() {
                residue.set_serial_number(counter);
                counter += 1;
            }
            counter = 0;
            for chain in model.chains_mut() {
                chain.set_id(std::char::from_u32((65 + counter % 26) as u32).unwrap());
                counter += 1;
            }
        }
        let mut counter = 1;
        for mtrix in &mut self.mtrix {
            mtrix.serial_number = counter;
            counter += 1;
        }
    }

    pub fn apply_transformation(&mut self, transformation: &TransformationMatrix) {
        for atom in self.all_atoms_mut() {
            atom.apply_transformation(transformation);
        }
    }
}

use std::fmt;
impl fmt::Display for PDB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PDB Models: {}", self.models.len())
    }
}

impl Clone for PDB {
    fn clone(&self) -> Self {
        let mut pdb = PDB::new();
        pdb.remarks = self.remarks.clone();
        pdb.scale = self.scale.clone();
        pdb.origx = self.origx.clone();
        pdb.mtrix = self.mtrix.clone();
        pdb.symmetry = self.symmetry.clone();
        pdb.unit_cell = self.unit_cell.clone();

        for model in self.models() {
            pdb.add_model(model.clone());
        }

        pdb
    }
}
