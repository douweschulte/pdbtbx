use crate::error::*;
use crate::structs::*;

/// Validate a given PDB file in terms of invariants that should be held up.
/// It returns `PDBError`s with the warning messages.
///
/// ## Invariants Tested
/// * With multiple models the models should all contain atoms that correspond.
///
/// ## Invariants Not Tested
/// * Numbering of all structs, serial numbers should be unique. To enforce this the `renumber()` function should be called on the PDB struct.
#[must_use]
pub fn validate(pdb: &PDB) -> Vec<PDBError> {
    let mut errors = Vec::new();
    if pdb.model_count() > 1 {
        errors.append(&mut validate_models(pdb));
    }

    if pdb.atoms().next().is_none() {
        errors.push(PDBError::new(
            ErrorLevel::BreakingError,
            "No Atoms",
            "No Atoms in the given PDB struct while validating.",
            Context::None,
        ));
    }
    errors
}

/// Validates this models specifically for the PDB format.
/// It returns `PDBError`s with the warning messages.
/// It extends the validation specified in the [`validate`] function with PDB specific validations.
///
/// ## Invariants Tested
/// * Values fitting in the range of the PDB format columns, both numbers and textual values.
///
/// ## Invariants Not Tested
/// * Numbering of all structs, serial numbers should be unique. To enforce this the `renumber()` function should be called on the PDB struct.
#[must_use]
pub fn validate_pdb(pdb: &PDB) -> Vec<PDBError> {
    let mut errors = validate(pdb);
    for model in pdb.models() {
        if model.serial_number() > 9999 {
            errors.push(PDBError::new(
                ErrorLevel::LooseWarning,
                "Model serial number too high",
                format!(
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
                    format!(
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
                        format!(
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
                            format!(
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
                            format!(
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
                                format!(
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
                            format!(
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
                            format!(
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
                                format!(
                                    "Atom {} has a name which is too long, max 4 chars.",
                                    atom.serial_number()
                                ),
                                Context::None,
                            ));
                        }
                        if atom.serial_number() > 99999 {
                            errors.push(PDBError::new(
                                ErrorLevel::LooseWarning,
                                "Atom serial number too high",
                                format!(
                                    "Atom {} has a serial number which is too high, max is 99999.",
                                    atom.serial_number()
                                ),
                                Context::None,
                            ));
                        }
                        if atom.occupancy() > 999.99 {
                            errors.push(PDBError::new(
                                ErrorLevel::LooseWarning,
                                "Atom occupancy out of bounds",
                                format!(
                                "Atom {} has a occupancy which is out of bounds, max is 999.99.",
                                atom.serial_number()
                            ),
                                Context::None,
                            ));
                        }
                        if atom.b_factor() > 999.99 {
                            errors.push(PDBError::new(
                                ErrorLevel::LooseWarning,
                                "Atom b factor out of bounds",
                                format!(
                                    "Atom {} has a b factor which is out of bounds, max is 999.99.",
                                    atom.serial_number()
                                ),
                                Context::None,
                            ));
                        }
                        if atom.x() > 9999.999 || atom.x() < -999.999 {
                            errors.push(PDBError::new(
                            ErrorLevel::LooseWarning,
                            "Atom x position out of bounds",
                            format!(
                                "Atom {} has an x which is out of bounds, max is 9999.999 min is -999.999.",
                                atom.serial_number()
                            ),
                            Context::None,
                        ));
                        }
                        if atom.y() > 9999.999 || atom.y() < -999.999 {
                            errors.push(PDBError::new(
                            ErrorLevel::LooseWarning,
                            "Atom y position out of bounds",
                            format!(
                                "Atom {} has a y which is out of bounds, max is 9999.999 min is -999.999.",
                                atom.serial_number()
                            ),
                            Context::None,
                        ));
                        }
                        if atom.z() > 9999.999 || atom.z() < -999.999 {
                            errors.push(PDBError::new(
                            ErrorLevel::LooseWarning,
                            "Atom z position out of bounds",
                            format!(
                                "Atom {} has a z which is out of bounds, max is 9999.999 min is -999.999.",
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
    let normal_atoms = pdb
        .model(0)
        .unwrap()
        .atoms()
        .filter(|a| !a.hetero())
        .count();
    for model in pdb.models().skip(1) {
        if model.atom_count() != total_atoms {
            errors.push(PDBError::new(
                ErrorLevel::LooseWarning,
                "Invalid Model",
                format!(
                    "Model {} does not have the same amount of atoms (Normal + Hetero) ({}) as the first model ({}).",
                    model.serial_number(),
                    model.atom_count(),
                    total_atoms
                ),
                Context::None,
            ));
            continue;
        } else if model.atoms().filter(|a| !a.hetero()).count() != normal_atoms {
            errors.push(PDBError::new(
                ErrorLevel::StrictWarning,
                "Invalid Model",
                format!(
                    "Model {} does not have the same amount of atoms ({}) as the first model ({}).",
                    model.serial_number(),
                    model.atoms().filter(|a| !a.hetero()).count(),
                    normal_atoms
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
                    format!(
                        "Atom {} in Model {} does not correspond to the respective Atom in the first model.",
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
        let count = residue.conformer_count();
        if count > 1 {
            let mut blank = None;
            for (index, conformer) in residue.conformers().enumerate() {
                if conformer.alternative_location().is_none() {
                    blank = Some(index);
                }
            }
            #[allow(clippy::unwrap_used, clippy::cast_precision_loss)]
            if let Some(index) = blank {
                let mut shared = residue.conformer(index).unwrap().clone();
                shared
                    .atoms_mut()
                    .for_each(|a| a.set_occupancy(a.occupancy() / (count as f64)).unwrap());
                residue.remove_conformer(index);
                for conformer in residue.conformers_mut() {
                    conformer.join(shared.clone());
                }
            }
        }
    }
}
