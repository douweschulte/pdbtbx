//! Test only reading the first model from PDB files.

use pdbtbx::*;

#[test]
fn only_read_atoms() {
    // PDB parser
    assert_eq!(1871, count_atoms("example-pdbs/rosetta_model.pdb", false));
    assert_eq!(1871, count_atoms("example-pdbs/rosetta_model.pdb", true));

    // TODO: mmCIF parser
    // assert_eq!(1871, count_atoms("example-pdbs/rosetta_model.cif", false));
    // assert_eq!(1871, count_atoms("example-pdbs/rosetta_model.cif", true));
}

fn count_atoms(filename: &str, only_atoms: bool) -> usize {
    let (structure, _errors) = ReadOptions::default()
        .set_level(StrictnessLevel::Loose)
        .set_only_atomic_coords(only_atoms)
        .read(filename)
        .unwrap();

    structure.atom_count()
}
