use crate::structs::*;

/// Validate a given PDB file in terms of invariants that should be held up.
/// It prints warning massages and returns a bool indicating success.
///
/// ## Invariants Tested
/// * With multiple models the models should all contain atoms that correspond.
/// * All matrix type PDB records (SCALEn, ORIGXn, MTRIXn) have to be fully specified, so all rows set.
///
/// ## Invariants Not Tested
/// * Numbering of all structs, serial numbers should be unique. To enforce this the `renumber()` function should be called on the PDB struct.
pub fn validate(pdb: &PDB) -> bool {
    // Print warnings/errors and return a bool for success
    let mut output = true;
    if pdb.model_count() > 1 {
        output = output && validate_models(pdb)
    }
    if pdb.has_scale() {
        output = output && pdb.scale().valid();
    }
    if pdb.has_origx() {
        output = output && pdb.origx().valid();
    }
    for m in pdb.mtrix() {
        output = output && m.valid();
    }
    output
}

/// Validate the models by enforcing that all models should contain the same atoms (with possibly different data).
/// It checks this by matching all atoms (not hetatoms) for each model to see if they correspond (`Atom::correspond`).
fn validate_models(pdb: &PDB) -> bool {
    let total_atoms = pdb.model(0).unwrap().atom_count();
    for model in pdb.models().skip(1) {
        if model.atom_count() != total_atoms {
            println!(
                "{} does not have the same amount of atoms as the first model ({} (this model) vs {} (first)).",
                model,
                model.atom_count(),
                total_atoms
            );
            return false;
        }
        for index in 0..model.atom_count() {
            let current_atom = model.atom(index).unwrap();
            let standard_atom = pdb.model(0).unwrap().atom(index).unwrap();
            if !standard_atom.corresponds(current_atom) {
                println!("Atom (index {}) in {} is not corresponding to the atom in the first model.\n    First model: {}\n    This model:  {}\n", index, model, standard_atom, current_atom);
                return false;
            }
        }
    }
    true
}
