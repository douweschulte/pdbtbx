#![allow(dead_code)]
use crate::structs::*;
use crate::transformation::*;

#[derive(Debug)]
/// A Model containing multiple Chains
pub struct Model {
    /// The serial number of this Model
    serial_number: usize,
    /// The Chains making up this model
    chains: Vec<Chain>,
    /// The Chains with Hetero Atoms making up this model
    hetero_chains: Vec<Chain>,
    /// The parent PDB of this Model, if available
    pdb: Option<*mut PDB>,
}

impl Model {
    /// Create a new Model
    ///
    /// ## Arguments
    /// * `serial_number` - the serial number
    /// * `pdb` - if available the parent of the Model
    pub fn new(serial_number: usize, pdb: Option<*mut PDB>) -> Model {
        Model {
            serial_number,
            chains: Vec::new(),
            hetero_chains: Vec::new(),
            pdb,
        }
    }

    /// The serial number of this Model
    pub fn serial_number(&self) -> usize {
        self.serial_number
    }

    /// Set the serial number of this Model
    pub fn set_serial_number(&mut self, new_number: usize) {
        self.serial_number = new_number;
    }

    /// Get the amount of Chains making up this Model.
    /// This disregards all Hetero Chains.
    pub fn chain_count(&self) -> usize {
        self.chains.len()
    }

    /// Get the amount of Residues making up this Model.
    /// This disregards all Hetero Residues.
    pub fn residue_count(&self) -> usize {
        self.chains()
            .fold(0, |sum, chain| chain.residue_count() + sum)
    }

    /// Get the amount of Atoms making up this Model.
    /// This disregards all Hetero Atoms.
    pub fn atom_count(&self) -> usize {
        self.chains().fold(0, |sum, chain| chain.atom_count() + sum)
    }

    /// Get the amount of Chains making up this Model.
    /// This includes all Hetero Chains.
    pub fn total_chain_count(&self) -> usize {
        self.chains.len() + self.hetero_chains.len()
    }

    /// Get the amount of Residues making up this Model.
    /// This includes all Hetero Residues.
    pub fn total_residue_count(&self) -> usize {
        self.all_chains()
            .fold(0, |sum, chain| chain.residue_count() + sum)
    }

    /// Get the amount of Atoms making up this Model.
    /// This includes all Hetero Atoms.
    pub fn total_atom_count(&self) -> usize {
        self.all_chains()
            .fold(0, |sum, chain| chain.atom_count() + sum)
    }

    /// Get a specific Chain from list of Chains making up this Model.
    ///
    /// ## Arguments
    /// * `index` - the index of the Chain
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn chain(&self, index: usize) -> Option<&Chain> {
        self.all_chains().nth(index)
    }

    /// Get a specific Chain as a mutable reference from list of Chains making up this Model.
    ///
    /// ## Arguments
    /// * `index` - the index of the Chain
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn chain_mut(&mut self, index: usize) -> Option<&mut Chain> {
        self.all_chains_mut().nth(index)
    }

    /// Get a specific Residue from the Residues making up this Model.
    ///
    /// ## Arguments
    /// * `index` - the index of the Residue
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn residue(&self, index: usize) -> Option<&Residue> {
        self.all_residues().nth(index)
    }

    /// Get a specific Residue as a mutable reference from the Residues making up this Model.
    ///
    /// ## Arguments
    /// * `index` - the index of the Residue
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn residue_mut(&mut self, index: usize) -> Option<&mut Residue> {
        self.all_residues_mut().nth(index)
    }

    /// Get a specific Atom from the Atoms making up this Model.
    ///
    /// ## Arguments
    /// * `index` - the index of the Atom
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn atom(&self, index: usize) -> Option<&Atom> {
        self.all_atoms().nth(index)
    }

    /// Get a specific Atom as a mutable reference from the Atoms making up this Model.
    ///
    /// ## Arguments
    /// * `index` - the index of the Atom
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn atom_mut(&mut self, index: usize) -> Option<&mut Atom> {
        self.all_atoms_mut().nth(index)
    }

    /// Get the list of Chains making up this Model.
    /// This disregards all Hetero Chains.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn chains(&self) -> impl DoubleEndedIterator<Item = &Chain> + '_ {
        self.chains.iter()
    }

    /// Get the list of Chains as mutable references making up this Model.
    /// This disregards all Hetero Chains.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn chains_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Chain> + '_ {
        self.chains.iter_mut()
    }

    /// Get the list of Residues making up this Model.
    /// This disregards all Hetero Residues.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn residues(&self) -> impl DoubleEndedIterator<Item = &Residue> + '_ {
        self.chains.iter().flat_map(|a| a.residues())
    }

    /// Get the list of Residues as mutable references making up this Model.
    /// This disregards all Hetero Residues.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn residues_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Residue> + '_ {
        self.chains.iter_mut().flat_map(|a| a.residues_mut())
    }

    /// Get the list of Atoms making up this Model.
    /// This disregards all Hetero Atoms.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.chains.iter().flat_map(|a| a.atoms())
    }

    /// Get the list of Atoms as mutable references making up this Model.
    /// This disregards all Hetero Atoms.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn atoms_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.chains.iter_mut().flat_map(|a| a.atoms_mut())
    }

    /// Get the list of Chains making up this Model.
    /// This disregards all Normal Chains.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn hetero_chains(&self) -> impl DoubleEndedIterator<Item = &Chain> + '_ {
        self.hetero_chains.iter()
    }

    /// Get the list of Chains as mutable references making up this Model.
    /// This disregards all Normal Chains.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn hetero_chains_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Chain> + '_ {
        self.hetero_chains.iter_mut()
    }

    /// Get the list of Residues making up this Model.
    /// This disregards all Normal Residues.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn hetero_residues(&self) -> impl DoubleEndedIterator<Item = &Residue> + '_ {
        self.hetero_chains.iter().flat_map(|a| a.residues())
    }

    /// Get the list of Residues as mutable references making up this Model.
    /// This disregards all Normal Residues
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn hetero_residues_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Residue> + '_ {
        self.hetero_chains
            .iter_mut()
            .map(|a| a.residues_mut())
            .flatten()
    }

    /// Get the list of Atoms making up this Model.
    /// This disregards all Normal Atoms.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn hetero_atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.hetero_chains.iter().flat_map(|a| a.atoms())
    }

    /// Get the list of Atoms as mutable references making up this Model.
    /// This disregards all Normal Atoms.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn hetero_atoms_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.hetero_chains
            .iter_mut()
            .map(|a| a.atoms_mut())
            .flatten()
    }

    /// Get the list of Chains making up this Model.
    /// This includes all Normal and Hetero Chains.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn all_chains(&self) -> impl DoubleEndedIterator<Item = &Chain> + '_ {
        self.chains.iter().chain(self.hetero_chains.iter())
    }

    /// Get the list of Chains as mutable references making up this Model.
    /// This includes all Normal and Hetero Chains.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn all_chains_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Chain> + '_ {
        self.chains.iter_mut().chain(self.hetero_chains.iter_mut())
    }

    /// Get the list of Residues making up this Model.
    /// This includes all Normal and Hetero Residues.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn all_residues(&self) -> impl DoubleEndedIterator<Item = &Residue> + '_ {
        self.chains
            .iter()
            .map(|a| a.residues())
            .flatten()
            .chain(self.hetero_chains.iter().flat_map(|a| a.residues()))
    }

    /// Get the list of Residues as mutable references making up this Model.
    /// This includes all Normal and Hetero Residues
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn all_residues_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Residue> + '_ {
        self.chains
            .iter_mut()
            .flat_map(|a| a.residues_mut())
            .chain(self.hetero_chains.iter_mut().flat_map(|a| a.residues_mut()))
    }

    /// Get the list of Atoms making up this Model.
    /// This includes all Normal and Hetero Atoms.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn all_atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.chains
            .iter()
            .flat_map(|a| a.atoms())
            .chain(self.hetero_chains.iter().flat_map(|a| a.atoms()))
    }

    /// Get the list of Atoms as mutable references making up this Model.
    /// This includes all Normal and Hetero Atoms.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn all_atoms_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.chains
            .iter_mut()
            .flat_map(|a| a.atoms_mut())
            .chain(self.hetero_chains.iter_mut().flat_map(|a| a.atoms_mut()))
    }

    /// Add a new Atom to this Model. It finds if there already is a Chain with the given `chain_id` if there is it will add this atom to that Chain, otherwise it will create a new Chain and add that to the list of Chains making up this Model. It does the same for the Residue, so it will create a new one if there does not yet exist a Residue with the given serial number.
    ///
    /// ## Arguments
    /// * `new_atom` - the new Atom to add
    /// * `chain_id` - the id of the Chain to add the Atom to
    /// * `residue_serial_number` - the serial number of the Residue to add the Atom to
    /// * `residue_name` - the name of the Residue to add the Atom to, only used to create a new Residue if needed
    ///
    /// ## Panics
    /// It panics if the Chain id or Residue name contains any invalid characters.
    pub fn add_atom(
        &mut self,
        new_atom: Atom,
        chain_id: char,
        residue_serial_number: usize,
        residue_name: [u8; 3],
    ) {
        let mut found = false;
        let mut new_chain =
            Chain::new(chain_id, Some(self)).expect("Invalid characters in chain creation");
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

    /// Add a new Atom to the hetero Atoms of this Model. It finds if there already is a Chain with the given `chain_id` if there is it will add this atom to that Chain, otherwise it will create a new Chain and add that to the list of Chains making up this Model. It does the same for the Residue, so it will create a new one if there does not yet exist a Residue with the given serial number.
    ///
    /// ## Arguments
    /// * `new_atom` - the new Atom to add
    /// * `chain_id` - the id of the Chain to add the Atom to
    /// * `residue_serial_number` - the serial number of the Residue to add the Atom to
    /// * `residue_name` - the name of the Residue to add the Atom to, only used to create a new Residue if needed
    ///
    /// ## Panics
    /// It panics if the Chain id or Residue name contains any invalid characters.
    pub fn add_hetero_atom(
        &mut self,
        new_atom: Atom,
        chain_id: char,
        residue_serial_number: usize,
        residue_name: [u8; 3],
    ) {
        let mut found = false;
        let mut new_chain = Chain::new(chain_id, Some(self))
            .unwrap_or_else(|| panic!("Invalid characters in chain creation ({})", chain_id));
        let mut current_chain = &mut new_chain;
        for chain in &mut self.hetero_chains {
            if chain.id() == chain_id {
                current_chain = chain;
                found = true;
                break;
            }
        }
        if !found {
            self.hetero_chains.push(new_chain);
            current_chain = self.hetero_chains.last_mut().unwrap();
        }

        current_chain.add_atom(new_atom, residue_serial_number, residue_name);
    }

    /// Add a Chain to the list of Chains making up this Model. This does not detect any duplicates of names or serial numbers in the list of Chains.
    fn add_chain(&mut self, mut chain: Chain) {
        chain.set_model(self);
        self.chains.push(chain);
    }

    /// Add a Chain to the list of Hetero Chains making up this Model. This does not detect any duplicates of names or serial numbers in the list of Chains.
    fn add_hetero_chain(&mut self, mut chain: Chain) {
        chain.set_model(self);
        self.hetero_chains.push(chain);
    }

    /// Set the parent PDB. This is used to link back to the parent to read its properties.
    /// This function should only be used when you are sure what you do, in normal cases it is not needed.
    pub fn set_pdb(&mut self, new_pdb: &mut PDB) {
        self.pdb = Some(new_pdb);
    }

    /// Set the parent PDB. This is used to link back to the parent to read its properties.
    /// This function should only be used when you are sure what you do, in normal cases it is not needed.
    pub fn set_pdb_pointer(&mut self, new_pdb: *mut PDB) {
        self.pdb = Some(new_pdb);
    }

    /// Get the parent PDB.
    /// ## Panics
    /// It panics if there is no parent PDB set.
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

    /// Get the parent PDB.
    /// ## Fails
    /// It fails if there is no parent PDB set.
    pub fn pdb_safe(&self) -> Option<&PDB> {
        if let Some(reference) = self.pdb {
            Some(unsafe { &*reference })
        } else {
            None
        }
    }

    /// Get the parent PDB mutably, pretty unsafe so you need to make sure yourself the use case is correct.
    /// ## Panics
    /// It panics if there is no parent PDB set.
    #[allow(clippy::mut_from_ref)]
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

    /// Get the parent PDB mutably, pretty unsafe so you need to make sure yourself the use case is correct.
    /// ## Fails
    /// It fails if there is no parent PDB set.
    fn pdb_mut_safe(&self) -> Option<&mut PDB> {
        if let Some(reference) = self.pdb {
            Some(unsafe { &mut *reference })
        } else {
            None
        }
    }

    /// This sets the parent of all structs contained by this Model.
    /// This should not be needed to run as a user of the library.
    pub fn fix_pointers_of_children(&mut self) {
        let reference: *mut Model = self;
        for chain in &mut self.chains {
            chain.set_model_pointer(reference);
            chain.fix_pointers_of_children();
        }
        for chain in &mut self.hetero_chains {
            chain.set_model_pointer(reference);
            chain.fix_pointers_of_children();
        }
    }

    /// Remove the Chain specified.
    ///
    /// ## Arguments
    /// * `index` - the index of the Chain to remove
    ///
    /// ## Panics
    /// It panics when the index is outside bounds.
    pub fn remove_chain(&mut self, index: usize) {
        self.chains.remove(index);
    }

    /// Remove the Chain specified. It returns `true` if it found a matching Chain and removed it.
    /// It removes the first matching Chain from the list.
    ///
    /// ## Arguments
    /// * `id` - the id of the Chain to remove
    pub fn remove_chain_id(&mut self, id: char) -> bool {
        let index = self.chains.iter().position(|a| a.id() == id);

        if let Some(i) = index {
            self.remove_chain(i);
            true
        } else {
            false
        }
    }

    /// Remove this Model from its parent PDB
    pub fn remove(&mut self) {
        self.pdb_mut()
            .remove_model_serial_number(self.serial_number());
    }

    /// Apply a transformation to the position of all atoms (Normal and Hetero) making up this Model, the new position is immediately set.
    pub fn apply_transformation(&mut self, transformation: &TransformationMatrix) {
        for atom in self.all_atoms_mut() {
            atom.apply_transformation(transformation);
        }
    }

    /// Join this Model with another Model, this moves all atoms from the other Model
    /// to this Model. All other (meta) data of this Model will stay the same.
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
            self.chains.len() + self.hetero_chains.len()
        )
    }
}

impl Clone for Model {
    fn clone(&self) -> Self {
        let mut model = Model::new(self.serial_number, None);

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

impl PartialEq for Model {
    fn eq(&self, other: &Self) -> bool {
        self.serial_number == other.serial_number
            && self.chains == other.chains
            && self.hetero_chains == other.hetero_chains
    }
}
