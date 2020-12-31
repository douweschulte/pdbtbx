use crate::structs::*;

/// Validate a given PDB file in terms of invariants that should be held up.
/// It prints warning massages and returns a bool indicating success.
pub fn validate(pdb: &PDB) -> bool {
    // Print warnings/errors and return a bool for success
    let mut output = true;
    if pdb.amount_models() > 1 {
        output = output && validate_models(pdb)
    }
    if pdb.scale.is_some() {
        output = output && pdb.scale().valid();
    }
    if pdb.origx.is_some() {
        output = output && pdb.origx().valid();
    }
    for m in &pdb.mtrix {
        output = output && m.valid();
    }
    return output;
}

/// Validate the models by enforcing that all models should contain the same atoms (with possibly different data).
/// It checks this by matching all atoms (not hetatoms) for each model to see if they correspond (`Atom::correspond`).
fn validate_models(pdb: &PDB) -> bool {
    let total_atoms = pdb.model(0).unwrap().amount_atoms();
    for model in pdb.models().skip(1) {
        if model.amount_atoms() != total_atoms {
            println!(
                "{} does not have the same amount of atoms as the first model ({} (this model) vs {} (first)).",
                model,
                model.amount_atoms(),
                total_atoms
            );
            return false;
        }
        for index in 0..model.amount_atoms() {
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
