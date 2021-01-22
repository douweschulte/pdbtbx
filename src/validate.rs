use crate::error::*;
use crate::structs::*;

/// Validate a given PDB file in terms of invariants that should be held up.
/// It returns PDBErrors with the warning messages.
///
/// ## Invariants Tested
/// * With multiple models the models should all contain atoms that correspond.
/// * All matrix type PDB records (SCALEn, ORIGXn, MTRIXn) have to be fully specified, so all rows set.
///
/// ## Invariants Not Tested
/// * Numbering of all structs, serial numbers should be unique. To enforce this the `renumber()` function should be called on the PDB struct.
pub fn validate(pdb: &PDB) -> Vec<PDBError> {
    // Print warnings/errors and return a bool for success
    let mut errors = Vec::new();
    if pdb.model_count() > 1 {
        errors.append(&mut validate_models(pdb));
    }
    if pdb.has_scale() && !pdb.scale().valid() {
        errors.push(PDBError::new(
            ErrorLevel::InvalidatingError,
            "Row not set",
            "A row was not set for SCALEn in the PDB.",
            Context::None,
        ));
    }
    if pdb.has_origx() && !pdb.origx().valid() {
        errors.push(PDBError::new(
            ErrorLevel::InvalidatingError,
            "Row not set",
            "A row was not set for ORIGXn in the PDB.",
            Context::None,
        ));
    }
    for m in pdb.mtrix() {
        if !m.valid() {
            errors.push(PDBError::new(
                ErrorLevel::InvalidatingError,
                "Row not set",
                &format!(
                    "A row was not set for MTRIXn Serial number {} in the PDB.",
                    m.serial_number()
                ),
                Context::None,
            ));
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
