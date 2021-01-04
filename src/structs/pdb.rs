#![allow(dead_code)]
use crate::structs::*;
use crate::transformation::*;

#[derive(Debug)]
/// A PDB file containing the 3D coordinates of many atoms making up the
/// 3D structure of a protein, but it can also be used for other molecules.
pub struct PDB {
    /// The remarks above the PDB file, containing the remark-type-number and a line of free text
    remarks: Vec<(usize, String)>,
    /// The Scale needed to transform orthogonal coordinates to fractional coordinates, if available
    scale: Option<Scale>,
    /// The OrigX needed to transform orthogonal coordinates to submitted coordinates, if available
    origx: Option<OrigX>,
    /// The MtriXs needed to transform the Models to the full assymetric subunit, if needed to contain the non-crystallographic symmetry
    mtrix: Vec<MtriX>,
    /// The unit cell of the crystal, containing its size and shape, if available
    unit_cell: Option<UnitCell>,
    /// The Symmetry or space group of the crystal, if available
    symmetry: Option<Symmetry>,
    /// The Models making up this PDB
    models: Vec<Model>,
}

impl PDB {
    /// Create an empty PDB struct
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

    /// Get the remarks, containing the remark-type-number and a line of free text
    pub fn remarks(&self) -> impl DoubleEndedIterator<Item = &(usize, String)> + '_ {
        self.remarks.iter()
    }

    /// Get the remarks as mutable references, containing the remark-type-number and a line of free text
    pub fn remarks_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut (usize, String)> + '_ {
        self.remarks.iter_mut()
    }

    /// Add a remark
    ///
    /// ## Arguments
    /// * `remark_type` - the remark-type-number
    /// * `remark_text` - the free line of text, containing the actual remark
    ///
    /// ## Panics
    /// It panics if the text if too long, the text contains invalid characters or the remark-type-number is not valid (wwPDB v3.30).
    pub fn add_remark(&mut self, remark_type: usize, remark_text: String) {
        const REMARK_TYPES: [usize; 40] = [
            0, 1, 2, 3, 4, 5, 100, 205, 210, 215, 217, 230, 240, 245, 247, 250, 265, 280, 285, 290,
            300, 350, 375, 450, 465, 470, 475, 480, 500, 525, 600, 610, 615, 620, 630, 650, 700,
            800, 900, 999,
        ];
        // TODO: assume valid?
        // if !REMARK_TYPES.contains(&remark_type) {
        //     panic!(format!("The given remark-type-number is not valid: {}, see wwPDB v3.30 for valid remark-type-numbers", remark_type));
        // }
        // if !check_chars(remark_text.clone()) {
        //     panic!("The given remark text contains invalid characters.");
        // }
        // As the text can only contain ASCII len() on strings is fine (it returns the length in bytes)
        if remark_text.len() > 68 {
            println!("WARNING: The given remark text is too long, the maximal length is 68 characters, the given string is {} characters.", remark_text.len());
        }

        self.remarks.push((remark_type, remark_text));
    }

    /// Returns `true` if the PDB has a Scale
    pub fn has_scale(&self) -> bool {
        self.scale.is_some()
    }

    /// Get the Scale from this PDB
    /// ## Panics
    /// It panics when there is no scale
    pub fn scale(&self) -> &Scale {
        match &self.scale {
            Some(u) => u,
            None => panic!("PDB has no scale"),
        }
    }

    /// Get the Scale from this PDB as a mutable reference
    /// ## Panics
    /// It panics when there is no scale
    pub fn scale_mut(&mut self) -> &mut Scale {
        match &mut self.scale {
            Some(u) => u,
            None => panic!("PDB has no scale"),
        }
    }

    /// Set the Scale fro this PDB
    pub fn set_scale(&mut self, scale: Scale) {
        self.scale = Some(scale);
    }

    /// Returns `true` if the PDB has an OrigX
    pub fn has_origx(&self) -> bool {
        self.origx.is_some()
    }

    /// Get the OrigX from this PDB
    /// ## Panics
    /// It panics when there is no OrigX
    pub fn origx(&self) -> &OrigX {
        match &self.origx {
            Some(u) => u,
            None => panic!("PDB has no origx"),
        }
    }

    /// Get the OrigX from this PDB as a mutable reference
    /// ## Panics
    /// It panics when there is no OrigX
    pub fn origx_mut(&mut self) -> &mut OrigX {
        match &mut self.origx {
            Some(u) => u,
            None => panic!("PDB has no origx"),
        }
    }

    /// Set the OrigX fro this PDB
    pub fn set_origx(&mut self, origx: OrigX) {
        self.origx = Some(origx);
    }

    /// Get the MtriX records for this PDB
    pub fn mtrix(&self) -> impl DoubleEndedIterator<Item = &MtriX> + '_ {
        self.mtrix.iter()
    }

    /// Get the MtriX records for this PDB, as mutable references
    pub fn mtrix_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut MtriX> + '_ {
        self.mtrix.iter_mut()
    }

    /// Get a specific MtriX.
    ///
    /// ## Arguments
    /// * `index` - the index of the MtriX to return
    ///
    /// ## Fails
    /// It fails when the index is out of bounds.
    pub fn get_mtrix(&self, index: usize) -> Option<&MtriX> {
        self.mtrix.get(index)
    }

    /// Get a specific MtriX as a mutable reference.
    ///
    /// ## Arguments
    /// * `index` - the index of the MtriX to return
    ///
    /// ## Fails
    /// It fails when the index is out of bounds.
    pub fn get_mtrix_mut(&mut self, index: usize) -> Option<&mut MtriX> {
        self.mtrix.get_mut(index)
    }

    /// Add a MtriX to this PDB
    pub fn add_mtrix(&mut self, mtrix: MtriX) {
        self.mtrix.push(mtrix);
    }

    /// Returns `true` if the PDB has a UnitCell
    pub fn has_unit_cell(&self) -> bool {
        self.unit_cell.is_some()
    }

    /// Get the UnitCell from this PDB
    /// ## Panics
    /// It panics when there is no UnitCell
    pub fn unit_cell(&self) -> &UnitCell {
        match &self.unit_cell {
            Some(u) => u,
            None => panic!("PDB has no unit cell"),
        }
    }

    /// Get the UnitCell from this PDB as a mutable reference
    /// ## Panics
    /// It panics when there is no UnitCell
    pub fn unit_cell_mut(&mut self) -> &mut UnitCell {
        match &mut self.unit_cell {
            Some(u) => u,
            None => panic!("PDB has no unit cell"),
        }
    }

    /// Set the UnitCell fro this PDB
    pub fn set_unit_cell(&mut self, cell: UnitCell) {
        self.unit_cell = Some(cell);
    }

    /// Returns `true` if the PDB has a Symmetry
    pub fn has_symmetry(&self) -> bool {
        self.symmetry.is_some()
    }

    /// Get the Symmetry from this PDB
    /// ## Panics
    /// It panics when there is no Symmetry
    pub fn symmetry(&self) -> &Symmetry {
        match &self.symmetry {
            Some(u) => u,
            None => panic!("PDB has no symmetry"),
        }
    }

    /// Get the Symmetry from this PDB as a mutable reference
    /// ## Panics
    /// It panics when there is no Symmetry
    pub fn symmetry_mut(&mut self) -> &mut Symmetry {
        match &mut self.symmetry {
            Some(u) => u,
            None => panic!("PDB has no symmetry"),
        }
    }

    /// Set the Symmetry fro this PDB
    pub fn set_symmetry(&mut self, symmetry: Symmetry) {
        self.symmetry = Some(symmetry);
    }

    /// Adds a Model to this PDB
    pub fn add_model(&mut self, mut new_model: Model) {
        new_model.set_pdb(self);
        self.models.push(new_model);
        self.models.last_mut().unwrap().fix_pointers_of_children();
    }

    /// Get the amount of Models making up this PDB
    pub fn model_count(&self) -> usize {
        self.models.len()
    }

    /// Get the amount of Chains making up this PDB.
    /// Disregarding Hetero Chains.
    pub fn chain_count(&self) -> usize {
        if !self.models.is_empty() {
            self.models[0].chain_count()
        } else {
            0
        }
    }

    /// Get the amount of Residues making up this PDB.
    /// Disregarding Hetero Residues.
    pub fn residue_count(&self) -> usize {
        if !self.models.is_empty() {
            self.models[0].residue_count()
        } else {
            0
        }
    }

    /// Get the amount of Atoms making up this PDB.
    /// Disregarding Hetero Atoms.
    pub fn atom_count(&self) -> usize {
        if !self.models.is_empty() {
            self.models[0].atom_count()
        } else {
            0
        }
    }

    /// Get the amount of Chains making up this PDB.
    /// Including Hetero Chains.
    pub fn total_chain_count(&self) -> usize {
        self.models.len() * self.chain_count()
    }

    /// Get the amount of Residues making up this PDB.
    /// Including Hetero Residues.
    pub fn total_residue_count(&self) -> usize {
        self.models.len() * self.residue_count()
    }

    /// Get the amount of Atoms making up this PDB.
    /// Including Hetero Atoms.
    pub fn total_atom_count(&self) -> usize {
        self.models.len() * self.atom_count()
    }

    /// Get a specific Model from list of Models making up this PDB.
    ///
    /// ## Arguments
    /// * `index` - the index of the Model
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn model(&self, index: usize) -> Option<&Model> {
        self.models.get(index)
    }

    /// Get a specific Model as a mutable reference from list of Models making up this PDB.
    ///
    /// ## Arguments
    /// * `index` - the index of the Model
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn model_mut(&mut self, index: usize) -> Option<&mut Model> {
        self.models.get_mut(index)
    }

    /// Get a specific Chain from the Chains making up this PDB. Including Hetero Atoms.
    ///
    /// ## Arguments
    /// * `index` - the index of the Chain
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn chain(&self, index: usize) -> Option<&Chain> {
        self.all_chains().nth(index)
    }

    /// Get a specific Chain as a mutable reference from the Chains making up this PDB. Including Hetero Atoms.
    ///
    /// ## Arguments
    /// * `index` - the index of the Chain
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn chain_mut(&mut self, index: usize) -> Option<&mut Chain> {
        self.all_chains_mut().nth(index)
    }

    /// Get a specific Residue from the Residues making up this PDB. Including Hetero Atoms.
    ///
    /// ## Arguments
    /// * `index` - the index of the Residue
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn residue(&self, index: usize) -> Option<&Residue> {
        self.all_residues().nth(index)
    }

    /// Get a specific Residue as a mutable reference from the Residues making up this PDB. Including Hetero Atoms.
    ///
    /// ## Arguments
    /// * `index` - the index of the Residue
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn residue_mut(&mut self, index: usize) -> Option<&mut Residue> {
        self.all_residues_mut().nth(index)
    }

    /// Get a specific Atom from the Atoms making up this PDB. Including Hetero Atoms.
    ///
    /// ## Arguments
    /// * `index` - the index of the Atom
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn atom(&self, index: usize) -> Option<&Atom> {
        self.all_atoms().nth(index)
    }

    /// Get a specific Atom as a mutable reference from the Atoms making up this PDB. Including Hetero Atoms.
    ///
    /// ## Arguments
    /// * `index` - the index of the Atom
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn atom_mut(&mut self, index: usize) -> Option<&mut Atom> {
        self.all_atoms_mut().nth(index)
    }

    /// Get the list of Models making up this PDB.
    pub fn models(&self) -> impl DoubleEndedIterator<Item = &Model> + '_ {
        self.models.iter()
    }

    /// Get the list of Models as mutable references making up this PDB.
    pub fn models_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Model> + '_ {
        self.models.iter_mut()
    }

    /// Get the list of Chains making up this PDB.
    /// This disregards all Hetero Chains.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn chains(&self) -> impl DoubleEndedIterator<Item = &Chain> + '_ {
        self.models.iter().flat_map(|a| a.chains())
    }

    /// Get the list of Chains as mutable references making up this PDB.
    /// This disregards all Hetero Chains.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn chains_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Chain> + '_ {
        self.models.iter_mut().flat_map(|a| a.chains_mut())
    }

    /// Get the list of Residue making up this PDB.
    /// This disregards all Hetero Residue.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn residues(&self) -> impl DoubleEndedIterator<Item = &Residue> + '_ {
        self.models.iter().flat_map(|a| a.residues())
    }

    /// Get the list of Residue as mutable references making up this PDB.
    /// This disregards all Hetero Residue.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn residues_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Residue> + '_ {
        self.models.iter_mut().flat_map(|a| a.residues_mut())
    }

    /// Get the list of Atom making up this PDB.
    /// This disregards all Hetero Atom.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.models.iter().flat_map(|a| a.atoms())
    }

    /// Get the list of Atom as mutable references making up this PDB.
    /// This disregards all Hetero Atom.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn atoms_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.models.iter_mut().flat_map(|a| a.atoms_mut())
    }

    /// Get the list of Chains making up this Model.
    /// This disregards all Normal Chains.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn hetero_chains(&self) -> impl DoubleEndedIterator<Item = &Chain> + '_ {
        self.models.iter().flat_map(|a| a.hetero_chains())
    }

    /// Get the list of Chains as mutable references making up this Model.
    /// This disregards all Normal Chains.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn hetero_chains_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Chain> + '_ {
        self.models
            .iter_mut()
            .map(|a| a.hetero_chains_mut())
            .flatten()
    }

    /// Get the list of Residues making up this Model.
    /// This disregards all Normal Residues.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn hetero_residues(&self) -> impl DoubleEndedIterator<Item = &Residue> + '_ {
        self.models.iter().flat_map(|a| a.hetero_residues())
    }

    /// Get the list of Residues as mutable references making up this Model.
    /// This disregards all Normal Residues.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn hetero_residues_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Residue> + '_ {
        self.models
            .iter_mut()
            .map(|a| a.hetero_residues_mut())
            .flatten()
    }

    /// Get the list of Atoms making up this Model.
    /// This disregards all Normal Atoms.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn hetero_atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.models.iter().flat_map(|a| a.hetero_atoms())
    }

    /// Get the list of Atoms as mutable references making up this Model.
    /// This disregards all Normal Atoms.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn hetero_atoms_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.models
            .iter_mut()
            .map(|a| a.hetero_atoms_mut())
            .flatten()
    }

    /// Get the list of Chains making up this Model.
    /// This includes all Normal and Hetero Chains.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn all_chains(&self) -> impl DoubleEndedIterator<Item = &Chain> + '_ {
        self.models.iter().flat_map(|a| a.all_chains())
    }

    /// Get the list of Chains as mutable references making up this Model.
    /// This includes all Normal and Hetero Chains.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn all_chains_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Chain> + '_ {
        self.models.iter_mut().flat_map(|a| a.all_chains_mut())
    }

    /// Get the list of Residues making up this Model.
    /// This includes all Normal and Hetero Residues.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn all_residues(&self) -> impl DoubleEndedIterator<Item = &Residue> + '_ {
        self.models.iter().flat_map(|a| a.all_residues())
    }

    /// Get the list of Residues as mutable references making up this Model.
    /// This includes all Normal and Hetero Residues.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn all_residues_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Residue> + '_ {
        self.models
            .iter_mut()
            .map(|a| a.all_residues_mut())
            .flatten()
    }

    /// Get the list of Atoms making up this Model.
    /// This includes all Normal and Hetero Atoms.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn all_atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.models.iter().flat_map(|a| a.all_atoms())
    }

    /// Get the list of Atoms as mutable references making up this Model.
    /// This includes all Normal and Hetero Atoms.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn all_atoms_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.models.iter_mut().flat_map(|a| a.all_atoms_mut())
    }

    /// This sets the parent of all structs contained by this PDB.
    /// This should not be needed to run as a user of the library.
    pub fn fix_pointers_of_children(&mut self) {
        let reference: *mut PDB = self;
        for model in &mut self.models {
            model.set_pdb_pointer(reference);
            model.fix_pointers_of_children();
        }
    }

    /// Remove the Model specified.
    ///
    /// ## Arguments
    /// * `index` - the index of the Model to remove
    ///
    /// ## Panics
    /// It panics when the index is outside bounds.
    pub fn remove_model(&mut self, index: usize) {
        self.models.remove(index);
    }

    /// Remove the Model specified. It returns `true` if it found a matching Model and removed it.
    /// It removes the first matching Model from the list.
    ///
    /// ## Arguments
    /// * `serial_number` - the serial number of the Model to remove
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

    /// This renumbers all numbered structs in the PDB.
    /// So it renumbers models, atoms, residues, chains and MtriXs.
    pub fn renumber(&mut self) {
        let mut model_counter = 1;
        for model in self.models_mut() {
            model.set_serial_number(model_counter);
            model_counter += 1;

            let mut counter = 1;
            for atom in model.all_atoms_mut() {
                atom.set_serial_number(counter);
                counter += 1;
            }
            counter = 1;
            for residue in model.all_residues_mut() {
                residue.set_serial_number(counter);
                counter += 1;
            }
            counter = 0;
            for chain in model.all_chains_mut() {
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

    /// Apply a transformation to the position of all atoms (Normal and Hetero) making up this PDB, the new position is immediately set.
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
        pdb.fix_pointers_of_children();
        pdb
    }
}

impl PartialEq for PDB {
    fn eq(&self, other: &Self) -> bool {
        self.scale == other.scale
            && self.origx == other.origx
            && self.mtrix == other.mtrix
            && self.unit_cell == other.unit_cell
            && self.symmetry == other.symmetry
            && self.models == other.models
    }
}

impl Default for PDB {
    fn default() -> Self {
        Self::new()
    }
}
