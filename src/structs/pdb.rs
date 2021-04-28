#![allow(dead_code)]
use crate::reference_tables;
use crate::structs::*;
use crate::transformation::*;
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq)]
/// A PDB file containing the 3D coordinates of many atoms making up the
/// 3D structure of a protein, but it can also be used for other molecules.
pub struct PDB {
    /// The identifier as posed in the PDB Header or mmCIF entry.id, normally a 4 char string like '1UBQ'
    pub identifier: Option<String>,
    /// The remarks above the PDB file, containing the remark-type-number and a line of free text
    remarks: Vec<(usize, String)>,
    /// The Scale needed to transform orthogonal coordinates to fractional coordinates, if available
    pub scale: Option<TransformationMatrix>,
    /// The OrigX needed to transform orthogonal coordinates to submitted coordinates, if available
    pub origx: Option<TransformationMatrix>,
    /// The MtriXs needed to transform the Models to the full asymmetric subunit, if needed to contain the non-crystallographic symmetry
    mtrix: Vec<MtriX>,
    /// The unit cell of the crystal, containing its size and shape, if available
    pub unit_cell: Option<UnitCell>,
    /// The Symmetry or space group of the crystal, if available
    pub symmetry: Option<Symmetry>,
    /// The Models making up this PDB
    models: Vec<Model>,
}

impl<'a> PDB {
    /// Create an empty PDB struct
    pub fn new() -> PDB {
        PDB {
            identifier: None,
            remarks: Vec::new(),
            scale: None,
            origx: None,
            mtrix: Vec::new(),
            unit_cell: None,
            symmetry: None,
            models: Vec::new(),
        }
    }

    /// Get the number of REMARK records in the PDB file
    pub fn remark_count(&self) -> usize {
        self.remarks.len()
    }

    /// Get the remarks, containing the remark-type-number and a line of free text
    pub fn remarks(&self) -> impl DoubleEndedIterator<Item = &(usize, String)> + '_ {
        self.remarks.iter()
    }

    /// Get the remarks in parallel, containing the remark-type-number and a line of free text
    pub fn par_remarks(&self) -> impl ParallelIterator<Item = &(usize, String)> + '_ {
        self.remarks.par_iter()
    }

    /// Get the remarks as mutable references, containing the remark-type-number and a line of free text
    pub fn remarks_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut (usize, String)> + '_ {
        self.remarks.iter_mut()
    }

    /// Get the parallel remarks as mutable references, containing the remark-type-number and a line of free text
    pub fn par_remarks_mut(&mut self) -> impl ParallelIterator<Item = &mut (usize, String)> + '_ {
        self.remarks.par_iter_mut()
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
        if !reference_tables::valid_remark_type_number(remark_type) {
            panic!("The given remark-type-number is not valid, see wwPDB v3.30 for valid remark-type-numbers");
        }
        if !valid_text(&remark_text) {
            panic!("The given remark text contains invalid characters.");
        }
        // As the text can only contain ASCII len() on strings is fine (it returns the length in bytes)
        if remark_text.len() > 70 {
            println!("WARNING: The given remark text is too long, the maximal length is 68 characters, the given string is {} characters.", remark_text.len());
        }

        self.remarks.push((remark_type, remark_text));
    }

    /// Get the MtriX records for this PDB
    pub fn mtrix(&self) -> impl DoubleEndedIterator<Item = &MtriX> + '_ {
        self.mtrix.iter()
    }

    /// Get the parallel MtriX records for this PDB
    pub fn par_mtrix(&self) -> impl ParallelIterator<Item = &MtriX> + '_ {
        self.mtrix.par_iter()
    }

    /// Get the MtriX records for this PDB, as mutable references
    pub fn mtrix_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut MtriX> + '_ {
        self.mtrix.iter_mut()
    }

    /// Get the parallel MtriX records for this PDB, as mutable references
    pub fn par_mtrix_mut(&mut self) -> impl ParallelIterator<Item = &mut MtriX> + '_ {
        self.mtrix.par_iter_mut()
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

    /// Adds a Model to this PDB
    pub fn add_model(&mut self, new_model: Model) {
        self.models.push(new_model);
    }

    /// Get the amount of Models making up this PDB
    pub fn model_count(&self) -> usize {
        self.models.len()
    }

    /// Get the amount of Chains making up this PDB.
    pub fn chain_count(&self) -> usize {
        if !self.models.is_empty() {
            self.models[0].chain_count()
        } else {
            0
        }
    }

    /// Get the amount of Residues making up this PDB.
    pub fn residue_count(&self) -> usize {
        if !self.models.is_empty() {
            self.models[0].residue_count()
        } else {
            0
        }
    }

    /// Get the amount of Residues making up this PDB in parallel.
    pub fn par_residue_count(&self) -> usize {
        if !self.models.is_empty() {
            self.models[0].par_residue_count()
        } else {
            0
        }
    }

    /// Get the amount of Conformers making up this PDB.
    pub fn conformer_count(&self) -> usize {
        if !self.models.is_empty() {
            self.models[0].conformer_count()
        } else {
            0
        }
    }

    /// Get the amount of Conformers making up this PDB in parallel.
    pub fn par_conformer_count(&self) -> usize {
        if !self.models.is_empty() {
            self.models[0].par_conformer_count()
        } else {
            0
        }
    }

    /// Get the amount of Atoms making up this PDB.
    pub fn atom_count(&self) -> usize {
        if !self.models.is_empty() {
            self.models[0].atom_count()
        } else {
            0
        }
    }

    /// Get the amount of Atoms making up this PDB in parallel.
    pub fn par_atom_count(&self) -> usize {
        if !self.models.is_empty() {
            self.models[0].par_atom_count()
        } else {
            0
        }
    }

    /// Get the amount of Chains making up this PDB. Including all models.
    pub fn total_chain_count(&self) -> usize {
        self.models
            .iter()
            .fold(0, |acc, item| acc + item.chain_count())
    }

    /// Get the amount of Chains making up this PDB in parallel. Including all models.
    pub fn par_total_chain_count(&self) -> usize {
        self.models.par_iter().map(|model| model.chain_count()).sum()
    }

    /// Get the amount of Residues making up this PDB. Including all models.
    pub fn total_residue_count(&self) -> usize {
        self.models
            .iter()
            .fold(0, |acc, item| acc + item.residue_count())
    }

    /// Get the amount of Residues making up this PDB in parallel. Including all models.
    pub fn par_total_residue_count(&self) -> usize {
        self.models.par_iter().map(|model| model.par_residue_count()).sum()
    }

    /// Get the amount of Conformer making up this PDB. Including all models.
    pub fn total_conformer_count(&self) -> usize {
        self.models
            .iter()
            .fold(0, |acc, item| acc + item.conformer_count())
    }

    /// Get the amount of Conformer making up this PDB in parallel. Including all models.
    pub fn par_total_conformer_count(&self) -> usize {
        self.models.par_iter().map(|model| model.par_conformer_count()).sum()
    }

    /// Get the amount of Atoms making up this PDB. Including all models.
    pub fn total_atom_count(&self) -> usize {
        self.models
            .iter()
            .fold(0, |acc, item| acc + item.atom_count())
    }

    /// Get the amount of Atoms making up this PDB in parallel. Including all models.
    pub fn par_total_atom_count(&self) -> usize {
        self.models.par_iter().map(|model| model.par_atom_count()).sum()
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

    /// Get a specific Chain from the Chains making up this PDB.
    ///
    /// ## Arguments
    /// * `index` - the index of the Chain
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn chain(&self, index: usize) -> Option<&Chain> {
        self.chains().nth(index)
    }

    /// Get a specific Chain as a mutable reference from the Chains making up this PDB.
    ///
    /// ## Arguments
    /// * `index` - the index of the Chain
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn chain_mut(&mut self, index: usize) -> Option<&mut Chain> {
        self.chains_mut().nth(index)
    }

    /// Get a specific Residue from the Residues making up this PDB.
    ///
    /// ## Arguments
    /// * `index` - the index of the Residue
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn residue(&self, index: usize) -> Option<&Residue> {
        self.residues().nth(index)
    }

    /// Get a specific Residue as a mutable reference from the Residues making up this PDB.
    ///
    /// ## Arguments
    /// * `index` - the index of the Residue
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn residue_mut(&mut self, index: usize) -> Option<&mut Residue> {
        self.residues_mut().nth(index)
    }

    /// Get a specific Conformer from the Conformers making up this PDB.
    ///
    /// ## Arguments
    /// * `index` - the index of the Conformer
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn conformer(&self, index: usize) -> Option<&Conformer> {
        self.conformers().nth(index)
    }

    /// Get a specific Conformer as a mutable reference from the Conformers making up this PDB.
    ///
    /// ## Arguments
    /// * `index` - the index of the Conformer
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn conformer_mut(&mut self, index: usize) -> Option<&mut Conformer> {
        self.conformers_mut().nth(index)
    }

    /// Get a specific Atom from the Atoms making up this PDB.
    ///
    /// ## Arguments
    /// * `index` - the index of the Atom
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn atom(&self, index: usize) -> Option<&Atom> {
        self.atoms().nth(index)
    }

    /// Get a specific Atom as a mutable reference from the Atoms making up this PDB.
    ///
    /// ## Arguments
    /// * `index` - the index of the Atom
    ///
    /// ## Fails
    /// It fails when the index is outside bounds.
    pub fn atom_mut(&mut self, index: usize) -> Option<&mut Atom> {
        self.atoms_mut().nth(index)
    }

    /// Get the specified atom, its uniqueness is guaranteed by including the
    /// alternative_location, with its full hierarchy. The algorithm is based
    /// on binary search so it is faster than an exhaustive search, but the
    /// full structure is assumed to be sorted. This assumption can be enforced
    /// by using `pdb.full_sort()`.
    pub fn binary_find_atom(
        &'a self,
        serial_number: usize,
        alternative_location: Option<&str>,
    ) -> Option<FullHierarchy<'a>> {
        if let Some(model) = self.models().next() {
            model.binary_find_atom(serial_number, alternative_location)
        } else {
            None
        }
    }

    /// Get the list of Models making up this PDB.
    pub fn models(&self) -> impl DoubleEndedIterator<Item = &Model> + '_ {
        self.models.iter()
    }

    /// Get the list of Models making up this PDB in parallel.
    pub fn par_models(&self) -> impl ParallelIterator<Item = &Model> + '_ {
        self.models.par_iter()
    }

    /// Get the list of Models as mutable references making up this PDB.
    pub fn models_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Model> + '_ {
        self.models.iter_mut()
    }

    /// Get the list of Models as mutable references making up this PDB in parallel.
    pub fn par_models_mut(&mut self) -> impl ParallelIterator<Item = &mut Model> + '_ {
        self.models.par_iter_mut()
    }

    /// Get the list of Chains making up this PDB.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn chains(&self) -> impl DoubleEndedIterator<Item = &Chain> + '_ {
        self.models.iter().flat_map(|a| a.chains())
    }

    /// Get the list of Chains making up this PDB in parallel.
    pub fn par_chains(&self) -> impl ParallelIterator<Item = &Chain> + '_ {
        self.models.par_iter().flat_map(|a| a.par_chains())
    }

    /// Get the list of Chains as mutable references making up this PDB.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn chains_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Chain> + '_ {
        self.models.iter_mut().flat_map(|a| a.chains_mut())
    }

    /// Get the list of Chains as mutable references making up this PDB in parallel.
    pub fn par_chains_mut(&mut self) -> impl ParallelIterator<Item = &mut Chain> + '_ {
        self.models.par_iter_mut().flat_map(|a| a.par_chains_mut())
    }

    /// Get the list of Residues making up this PDB.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn residues(&self) -> impl DoubleEndedIterator<Item = &Residue> + '_ {
        self.models.iter().flat_map(|a| a.residues())
    }

    /// Get the list of Residues making up this PDB in parallel.
    pub fn par_residues(&self) -> impl ParallelIterator<Item = &Residue> + '_ {
        self.models.par_iter().flat_map(|a| a.par_residues())
    }

    /// Get the list of Residues making up this PDB in parallel.
    // pub fn par_residues(&self) -> impl ParallelIterator<Item = &Residue> + '_ {
    //     self.models.par_iter().flat_map(|a| a.residues())
    // }

    /// Get the list of Residue as mutable references making up this PDB.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn residues_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Residue> + '_ {
        self.models.iter_mut().flat_map(|a| a.residues_mut())
    }

    /// Get the list of Residue as mutable references making up this PDB in parallel.
    pub fn par_residues_mut(&mut self) -> impl ParallelIterator<Item = &mut Residue> + '_ {
        self.models.par_iter_mut().flat_map(|a| a.par_residues_mut())
    }

    /// Get the list of Conformers making up this PDB.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn conformers(&self) -> impl DoubleEndedIterator<Item = &Conformer> + '_ {
        self.models.iter().flat_map(|a| a.conformers())
    }

    /// Get the list of Conformers making up this PDB in parallel.
    pub fn par_conformers(&self) -> impl ParallelIterator<Item = &Conformer> + '_ {
        self.models.par_iter().flat_map(|a| a.par_conformers())
    }

    /// Get the list of Conformers as mutable references making up this PDB.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn conformers_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Conformer> + '_ {
        self.models.iter_mut().flat_map(|a| a.conformers_mut())
    }

    /// Get the list of Conformers as mutable references making up this PDB in parallel.
    pub fn par_conformers_mut(&mut self) -> impl ParallelIterator<Item = &mut Conformer> + '_ {
        self.models.par_iter_mut().flat_map(|a| a.par_conformers_mut())
    }

    /// Get the list of Atom making up this PDB.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn atoms(&self) -> impl DoubleEndedIterator<Item = &Atom> + '_ {
        self.models.iter().flat_map(|a| a.atoms())
    }

    /// Get the list of Atom making up this PDB in parallel.
    pub fn par_atoms(&self) -> impl ParallelIterator<Item = &Atom> + '_ {
        self.models.par_iter().flat_map(|a| a.par_atoms())
    }

    /// Get the list of Atom as mutable references making up this PDB.
    /// Double ended so iterating from the end is just as fast as from the start.
    pub fn atoms_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Atom> + '_ {
        self.models.iter_mut().flat_map(|a| a.atoms_mut())
    }

    /// Get the list of Atom as mutable references making up this PDB in parallel.
    pub fn par_atoms_mut(&mut self) -> impl ParallelIterator<Item = &mut Atom> + '_ {
        self.models.par_iter_mut().flat_map(|a| a.par_atoms_mut())
    }

    /// Remove all Atoms matching the given predicate. The predicate will be run on all Atoms.
    /// As this is done in place this is the fastest way to remove Atoms from this PDB.
    pub fn remove_atoms_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Atom) -> bool,
    {
        for residue in self.residues_mut() {
            residue.remove_atoms_by(&predicate);
        }
    }

    /// Remove all Conformers matching the given predicate. The predicate will be run on all Conformers.
    /// As this is done in place this is the fastest way to remove Conformers from this PDB.
    pub fn remove_conformers_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Conformer) -> bool,
    {
        for chain in self.chains_mut() {
            chain.remove_conformers_by(&predicate);
        }
    }

    /// Remove all Residues matching the given predicate. The predicate will be run on all Residues.
    /// As this is done in place this is the fastest way to remove Residues from this PDB.
    pub fn remove_residues_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Residue) -> bool,
    {
        for chain in self.chains_mut() {
            chain.remove_residues_by(&predicate);
        }
    }

    /// Remove all Residues matching the given predicate. The predicate will be run on all Residues.
    /// As this is done in place this is the fastest way to remove Residues from this PDB.
    pub fn remove_chains_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Chain) -> bool,
    {
        for model in self.models_mut() {
            model.remove_chains_by(&predicate);
        }
    }

    /// Remove all Chains matching the given predicate. The predicate will be run on all Chains.
    /// As this is done in place this is the fastest way to remove Chains from this PDB.
    pub fn remove_models_by<F>(&mut self, predicate: F)
    where
        F: Fn(&Model) -> bool,
    {
        self.models.retain(|model| !predicate(model));
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

    /// Remove the Model specified. It returns `true` if it found a matching Model and removed it.
    /// It removes the first matching Model from the list.
    /// Done in parallel.
    ///
    /// ## Arguments
    /// * `serial_number` - the serial number of the Model to remove
    pub fn par_remove_model_serial_number(&mut self, serial_number: usize) -> bool {
        let index = self
            .models
            .par_iter()
            .position_first(|a| a.serial_number() == serial_number);

        if let Some(i) = index {
            self.remove_model(i);
            true
        } else {
            false
        }
    }

    /// Remove all empty Models from this PDB, and all empty Chains from the Model, and all empty Residues from the Chains.
    pub fn remove_empty(&mut self) {
        self.models.iter_mut().for_each(|m| m.remove_empty());
        self.models.retain(|m| m.chain_count() > 0);
    }

    /// Remove all empty Models from this PDB, and all empty Chains from the Model, and all empty Residues from the Chains.
    /// Done in parallel.
    pub fn par_remove_empty(&mut self) {
        self.models.par_iter_mut().for_each(|m| m.remove_empty());
        self.models.retain(|m| m.chain_count() > 0);
    }

    /// This renumbers all numbered structs in the PDB.
    /// So it renumbers models, atoms, residues, chains and MtriXs.
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
            let mut counter_i = 1;
            for residue in model.residues_mut() {
                residue.set_serial_number(counter_i);
                residue.remove_insertion_code();
                counter_i += 1;

                #[allow(clippy::comparison_chain)] // Using match here is kind of weird
                if residue.conformer_count() > 1 {
                    counter = 0;
                    for conformer in residue.conformers_mut() {
                        conformer.set_alternative_location(&number_to_base26(counter));
                        counter += 1;
                    }
                } else if residue.conformer_count() == 1 {
                    #[allow(clippy::unwrap_used)]
                    residue
                        .conformer_mut(0)
                        .unwrap()
                        .remove_alternative_location();
                }
            }
            counter = 0;
            for chain in model.chains_mut() {
                chain.set_id(&number_to_base26(counter));
                counter += 1;
            }
        }
    }

    /// Apply a transformation to the position of all atoms making up this PDB, the new position is immediately set.
    pub fn apply_transformation(&mut self, transformation: &TransformationMatrix) {
        for atom in self.atoms_mut() {
            atom.apply_transformation(transformation);
        }
    }

    /// Apply a transformation to the position of all atoms making up this PDB, the new position is immediately set.
    /// Done in parallel.
    pub fn par_apply_transformation(&mut self, transformation: &TransformationMatrix) {
        self.par_atoms_mut().for_each(|atom| atom.apply_transformation(transformation))
    }

    /// Joins two PDBs. If one has multiple models it extends the models of this PDB with the models of the other PDB. If this PDB does
    /// not have any models it moves the models of the other PDB to this PDB. If both have one model it moves all chains/residues/atoms
    /// form the first model of the other PDB to the first model of this PDB. Effectively the same as calling join on those models.
    pub fn join(&mut self, mut other: PDB) {
        #[allow(clippy::unwrap_used)]
        if self.model_count() > 1 || other.model_count() > 1 {
            self.models.extend(other.models);
        } else if self.model_count() == 0 {
            self.models = other.models;
        } else if other.model_count() == 0 {
            // There is nothing to join
        } else {
            self.model_mut(0).unwrap().join(other.models.remove(0))
        }
    }

    /// Extend the Models on this PDB by the given iterator.
    pub fn extend<T: IntoIterator<Item = Model>>(&mut self, iter: T) {
        self.models.extend(iter);
    }

    /// Sort the Models of this PDB
    pub fn sort(&mut self) {
        self.models.sort();
    }

    /// Sort the Models of this PDB in parallel
    pub fn par_sort(&mut self) {
        self.models.par_sort();
    }

    /// Sort all structs in this PDB
    pub fn full_sort(&mut self) {
        self.sort();
        for model in self.models_mut() {
            model.sort();
        }
        for chain in self.chains_mut() {
            chain.sort();
        }
        for residue in self.residues_mut() {
            residue.sort();
        }
        for conformer in self.conformers_mut() {
            conformer.sort();
        }
    }

    /// Sort all structs in this PDB in parallel
    pub fn par_full_sort(&mut self) {
        self.par_sort();
        self.par_models_mut().for_each(|model| model.par_sort());
        self.par_chains_mut().for_each(|chain| chain.par_sort());
        self.par_residues_mut().for_each(|residue| residue.par_sort());
        self.par_conformers_mut().for_each(|conformer| conformer.par_sort());
    }

    /// Create an R star tree which can be used for fast lookup of
    /// spatial close atoms. See the crate rstar for documentation
    /// on how to use the tree. (https://crates.io/crates/rstar)
    ///
    /// Keep in mind that this creates a tree that is separate from
    /// the original PDB, so any changes to one of the data
    /// structures is not seen in the other data structure (until
    /// you generate a new tree of course).
    pub fn create_rtree(&self) -> rstar::RTree<&Atom> {
        rstar::RTree::bulk_load(self.atoms().collect::<Vec<&Atom>>())
    }

    /// Create an R star tree from parallel Iterator over atoms.
    pub fn par_create_rtree(&self) -> rstar::RTree<&Atom> {
        rstar::RTree::bulk_load(self.par_atoms().collect::<Vec<&Atom>>())
    }

    /// Create an R star tree which can be used for fast lookup of
    /// spatial close atoms. See the crate rstar for documentation
    /// on how to use the tree. (https://crates.io/crates/rstar)
    ///
    /// Keep in mind that this creates a tree that is separate from
    /// the original PDB, so any changes to one of the data
    /// structures is not seen in the other data structure (until
    /// you generate a new tree of course).
    pub fn create_full_hierarchy_rtree(&'a self) -> Option<rstar::RTree<FullHierarchy<'a>>> {
        if let Some(model) = self.models().next() {
            Some(rstar::RTree::bulk_load(
                model.atoms_full_hierarchy().collect(),
            ))
        } else {
            None
        }
    }

    /// Create an R star Tree from parallel Iterator over full hierarchy struct.
    pub fn par_create_full_hierarchy_rtree(&'a self) -> Option<rstar::RTree<FullHierarchy<'a>>> {
        if let Some(model) = self.models().next() {
            Some(rstar::RTree::bulk_load(
                model.par_atoms_full_hierarchy().collect(),
            ))
        } else {
            None
        }
    }
}

use std::fmt;
impl fmt::Display for PDB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PDB Models: {}", self.models.len())
    }
}

impl Default for PDB {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn sort_atoms() {
        let a = Atom::new(false, 0, "", 0.0, 0.0, 0.0, 0.0, 0.0, "", 0).unwrap();
        let b = Atom::new(false, 1, "", 0.0, 0.0, 0.0, 0.0, 0.0, "", 0).unwrap();
        let mut model = Model::new(0);
        model.add_atom(b, "A", (0, None), ("LYS", None));
        model.add_atom(a, "A", (0, None), ("LYS", None));
        let mut pdb = PDB::new();
        pdb.add_model(model);
        assert_eq!(pdb.atom(0).unwrap().serial_number(), 1);
        assert_eq!(pdb.atom(1).unwrap().serial_number(), 0);
        pdb.full_sort();
        assert_eq!(pdb.atom(0).unwrap().serial_number(), 0);
        assert_eq!(pdb.atom(1).unwrap().serial_number(), 1);
    }

    #[test]
    fn spatial_lookup() {
        let mut model = Model::new(0);
        model.add_atom(
            Atom::new(false, 0, "", 0.0, 0.0, 0.0, 0.0, 0.0, "", 0).unwrap(),
            "A",
            (0, None),
            ("MET", None),
        );
        model.add_atom(
            Atom::new(false, 1, "", 1.0, 1.0, 1.0, 0.0, 0.0, "", 0).unwrap(),
            "A",
            (0, None),
            ("MET", None),
        );
        model.add_atom(
            Atom::new(false, 2, "", 0.0, 1.0, 1.0, 0.0, 0.0, "", 0).unwrap(),
            "A",
            (0, None),
            ("MET", None),
        );
        let mut pdb = PDB::new();
        pdb.add_model(model);
        let tree = pdb.create_rtree();
        assert_eq!(tree.size(), 3);
        assert_eq!(
            tree.nearest_neighbor(&[1.0, 1.0, 1.0])
                .unwrap()
                .serial_number(),
            1
        );
        assert_eq!(
            tree.locate_within_distance([1.0, 1.0, 1.0], 1.0)
                .fold(0, |acc, _| acc + 1),
            2
        );
        let mut neighbors = tree.nearest_neighbor_iter(&pdb.atom(0).unwrap().pos_array());
        assert_eq!(neighbors.next().unwrap().serial_number(), 0);
        assert_eq!(neighbors.next().unwrap().serial_number(), 2);
        assert_eq!(neighbors.next().unwrap().serial_number(), 1);
    }

    #[test]
    fn binary_lookup() {
        let mut model = Model::new(0);
        model.add_atom(
            Atom::new(false, 1, "", 0.0, 0.0, 0.0, 0.0, 0.0, "", 0).unwrap(),
            "A",
            (0, None),
            ("MET", Some("A")),
        );
        model.add_atom(
            Atom::new(false, 1, "", 1.0, 0.0, 0.0, 0.0, 0.0, "", 0).unwrap(),
            "A",
            (0, None),
            ("MET", Some("B")),
        );
        model.add_atom(
            Atom::new(false, 1, "", 2.0, 0.0, 0.0, 0.0, 0.0, "", 0).unwrap(),
            "A",
            (0, None),
            ("MET", None),
        );
        let mut pdb = PDB::new();
        pdb.add_model(model);
        pdb.full_sort();

        assert_eq!(pdb.binary_find_atom(1, Some("A")).unwrap().atom.x(), 0.0);
        assert_eq!(pdb.binary_find_atom(1, Some("B")).unwrap().atom.x(), 1.0);
        assert_eq!(pdb.binary_find_atom(1, None).unwrap().atom.x(), 2.0);
    }
}
