use crate::structs::*;

pub fn validate(pdb: &PDB) -> bool {
    // Print warnings/errors and return a bool for success
    let mut output = true;
    if pdb.amount_models() > 1 {
        output = output && validate_models(pdb)
    }
    if pdb.scale.is_some() {
        output = output && pdb.scale.as_ref().unwrap().valid();
    }
    if pdb.origx.is_some() {
        output = output && pdb.origx.as_ref().unwrap().valid();
    }
    for m in &pdb.mtrix {
        output = output && m.valid();
    }
    return output;
}

fn validate_models(pdb: &PDB) -> bool {
    let total_atoms = pdb.model(0).unwrap().total_amount_atoms();
    for model in pdb.models().skip(1) {
        if model.total_amount_atoms() != total_atoms {
            println!(
                "{} does not have the same amount of atoms as the first model ({} (this model) vs {} (first)).",
                model,
                model.total_amount_atoms(),
                total_atoms
            );
            return false;
        }
        for index in 0..model.total_amount_atoms() {
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
