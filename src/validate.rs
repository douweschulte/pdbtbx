use crate::error::*;
use crate::structs::*;

/// Validate a given PDB file in terms of invariants that should be held up.
/// It returns PDBErrors with the warning messages.
///
/// ## Invariants Tested
/// * With multiple models the models should all contain atoms that correspond.
///
/// ## Invariants Not Tested
/// * Numbering of all structs, serial numbers should be unique. To enforce this the `renumber()` function should be called on the PDB struct.
pub fn validate(pdb: &PDB) -> Vec<PDBError> {
    // Print warnings/errors and return a bool for success
    let mut errors = Vec::new();
    if pdb.model_count() > 1 {
        errors.append(&mut validate_models(pdb));
    }
    errors
}

/// Validates this models specifically for the PDB format
pub fn validate_pdb(pdb: &PDB) -> Vec<PDBError> {
    let mut errors = Vec::new();
    for model in pdb.models() {
        if model.serial_number() > 9999 {
            errors.push(PDBError::new(
                ErrorLevel::LooseWarning,
                "Model serial number too high",
                &format!(
                    "Model {} has a serial number which is too high, max 9999.",
                    model.serial_number()
                ),
                Context::None,
            ));
        }
        for chain in model.chains() {
            if chain.id().len() > 1 {
                errors.push(PDBError::new(
                    ErrorLevel::LooseWarning,
                    "Chain id too long",
                    &format!(
                        "Chain {} has a name which is too long, max 1 character.",
                        chain.id()
                    ),
                    Context::None,
                ));
            }
            for residue in chain.residues() {
                if residue.serial_number() > 9999 {
                    errors.push(PDBError::new(
                        ErrorLevel::LooseWarning,
                        "Residue serial number too high",
                        &format!(
                            "Residue {:?} has a serial number which is too high, max is 9999.",
                            residue.id()
                        ),
                        Context::None,
                    ));
                }
                if let Some(ic) = residue.insertion_code() {
                    if ic.len() > 1 {
                        errors.push(PDBError::new(
                            ErrorLevel::LooseWarning,
                            "Residue insertion code too long",
                            &format!(
                                "Residue {:?} has an insertion code which is too long, max 1 char.",
                                residue.id()
                            ),
                            Context::None,
                        ));
                    }
                }
                for conformer in residue.conformers() {
                    if conformer.name().len() > 3 {
                        errors.push(PDBError::new(
                            ErrorLevel::LooseWarning,
                            "Conformer name too long",
                            &format!(
                                "Conformer {:?} has a name which is too long, max 3 chars.",
                                conformer.id()
                            ),
                            Context::None,
                        ));
                    }
                    if let Some(alt_loc) = conformer.alternative_location() {
                        if alt_loc.len() > 1 {
                            errors.push(PDBError::new(
                                ErrorLevel::LooseWarning,
                                "Conformer alternative location too long",
                                &format!(
                                    "Conformer {:?} has an alternative location which is too long, max 1 char.",
                                    conformer.id()
                                ),
                                Context::None,
                            ));
                        }
                    }
                    if let Some((n, comment)) = conformer.modification() {
                        if n.len() > 3 {
                            errors.push(PDBError::new(
                            ErrorLevel::LooseWarning,
                            "Residue modification name too long",
                            &format!(
                                "Residue {} has a modification name which is too long, max 3 chars.",
                                residue.serial_number()
                            ),
                            Context::None,
                        ));
                        }
                        if comment.len() > 41 {
                            errors.push(PDBError::new(
                            ErrorLevel::LooseWarning,
                            "Residue modification comment too long",
                            &format!(
                                "Residue {} has a modification comment which is too long, max 41 chars.",
                                residue.serial_number()
                            ),
                            Context::None,
                        ));
                        }
                    }
                    for atom in conformer.atoms() {
                        if atom.name().len() > 4 {
                            errors.push(PDBError::new(
                                ErrorLevel::LooseWarning,
                                "Atom name too long",
                                &format!(
                                    "Atom {} has a name which is too long, max 4 chars.",
                                    atom.serial_number()
                                ),
                                Context::None,
                            ));
                        }
                        if atom.element().len() > 2 {
                            errors.push(PDBError::new(
                                ErrorLevel::LooseWarning,
                                "Atom element too long",
                                &format!(
                                    "Atom {} has a element which is too long, max 2 chars.",
                                    atom.serial_number()
                                ),
                                Context::None,
                            ));
                        }
                        if atom.serial_number() > 9999 {
                            errors.push(PDBError::new(
                                ErrorLevel::LooseWarning,
                                "Atom serial number too high",
                                &format!(
                                    "Atom {} has a serial number which is too high, max is 9999.",
                                    atom.serial_number()
                                ),
                                Context::None,
                            ));
                        }
                        if atom.charge() > 9 || atom.charge() < -9 {
                            errors.push(PDBError::new(
                                ErrorLevel::LooseWarning,
                                "Atom charge out of bounds",
                                &format!(
                                "Atom {} has a charge which is out of bounds, max is 9 min is -9.",
                                atom.serial_number()
                            ),
                                Context::None,
                            ));
                        }
                        if atom.occupancy() > 999.99 || atom.occupancy() < 0.01 {
                            errors.push(PDBError::new(
                            ErrorLevel::LooseWarning,
                            "Atom occupancy out of bounds",
                            &format!(
                                "Atom {} has a occupancy which is out of bounds, max is 999.99 min is 0.01.",
                                atom.serial_number()
                            ),
                            Context::None,
                        ));
                        }
                        if atom.b_factor() > 999.99 || atom.b_factor() < 0.01 {
                            errors.push(PDBError::new(
                            ErrorLevel::LooseWarning,
                            "Atom b factor out of bounds",
                            &format!(
                                "Atom {} has a b factor which is out of bounds, max is 999.99 min is 0.01.",
                                atom.serial_number()
                            ),
                            Context::None,
                        ));
                        }
                        if atom.x() > 9999.999 || atom.x().abs() < 0.001 || atom.x() < -999.999 {
                            errors.push(PDBError::new(
                            ErrorLevel::LooseWarning,
                            "Atom x position out of bounds",
                            &format!(
                                "Atom {} has an x which is out of bounds, max is 9999.999 min is -999.999 and the smallest value is 0.001.",
                                atom.serial_number()
                            ),
                            Context::None,
                        ));
                        }
                        if atom.y() > 9999.999 || atom.y().abs() < 0.001 || atom.y() < -999.999 {
                            errors.push(PDBError::new(
                            ErrorLevel::LooseWarning,
                            "Atom y position out of bounds",
                            &format!(
                                "Atom {} has a y which is out of bounds, max is 9999.999 min is -999.999 and the smallest value is 0.001.",
                                atom.serial_number()
                            ),
                            Context::None,
                        ));
                        }
                        if atom.z() > 9999.999 || atom.z().abs() < 0.001 || atom.z() < -999.999 {
                            errors.push(PDBError::new(
                            ErrorLevel::LooseWarning,
                            "Atom z position out of bounds",
                            &format!(
                                "Atom {} has a z which is out of bounds, max is 9999.999 min is -999.999 and the smallest value is 0.001.",
                                atom.serial_number()
                            ),
                            Context::None,
                        ));
                        }
                    }
                }
            }
        }
    }
    errors
}

/// Validate the models by enforcing that all models should contain the same atoms (with possibly different data).
/// It checks this by matching all atoms (not hetatoms) for each model to see if they correspond (`Atom::correspond`).
#[allow(clippy::unwrap_used)]
fn validate_models(pdb: &PDB) -> Vec<PDBError> {
    let mut errors = Vec::new();
    let total_atoms = pdb.model(0).unwrap().atom_count();
    let total_hetero_atoms =
        pdb.model(0).unwrap().total_atom_count() - pdb.model(0).unwrap().atom_count();
    for model in pdb.models().skip(1) {
        if model.atom_count() != total_atoms {
            errors.push(PDBError::new(
                ErrorLevel::StrictWarning,
                "Invalid Model",
                &format!(
                    "Model {} does not have the same amount of atoms as the first model.",
                    model.serial_number()
                ),
                Context::None,
            ));
            continue;
        }
        if model.total_atom_count() - model.atom_count() != total_hetero_atoms {
            errors.push(PDBError::new(
                ErrorLevel::LooseWarning,
                "Invalid Model",
                &format!(
                    "Model {} does not have the same amount of HETATMs as the first model.",
                    model.serial_number()
                ),
                Context::None,
            ));
            continue;
        }
        for index in 0..model.atom_count() {
            let current_atom = model.atom(index).unwrap();
            let standard_atom = pdb.model(0).unwrap().atom(index).unwrap();
            if !standard_atom.corresponds(current_atom) {
                errors.push(PDBError::new(
                    ErrorLevel::StrictWarning,
                    "Atoms in Models not corresponding",
                    &format!(
                        "Atom {} in Model {} does not correspond to the respective Atom in the first model.",
                        current_atom.serial_number(),
                        model.serial_number()
                    ),
                    Context::None,
                ));
            }
        }
        for offset in 0..model.total_atom_count() - model.atom_count() {
            let index = model.atom_count() + offset;
            let current_atom = model.atom(index).unwrap();
            let standard_atom = pdb.model(0).unwrap().atom(index).unwrap();
            if !standard_atom.corresponds(current_atom) {
                errors.push(PDBError::new(
                    ErrorLevel::LooseWarning,
                    "HETATMs in Models not corresponding",
                    &format!(
                        "HETATM {} in Model {} does not correspond to the respective Atom in the first model.",
                        current_atom.serial_number(),
                        model.serial_number()
                    ),
                    Context::None,
                ));
            }
        }
    }
    errors
}

/// Copy all atoms in blank alternative conformers into the other conformers.
/// So if there is a A and B conformer with one atom different, based on the
/// PDB file the generated structs will contain a blank, an A, and a B Conformer
/// so the atoms in the blank constructs will have to be copied to the A and B
/// Conformers.
pub fn reshuffle_conformers(pdb: &mut PDB) {
    for residue in pdb.residues_mut() {
        if residue.conformer_count() > 1 {
            let mut blank = None;
            for (index, conformer) in residue.conformers().enumerate() {
                if conformer.alternative_location().is_none() {
                    blank = Some(index);
                }
            }
            if let Some(index) = blank {
                #[allow(clippy::unwrap_used)]
                let shared = residue.conformer(index).unwrap().clone();
                residue.remove_conformer(index);
                for conformer in residue.conformers_mut() {
                    conformer.join(shared.clone());
                }
            }
        }
    }
}
