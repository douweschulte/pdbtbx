use pdbtbx::*;
use std::env;
use std::path::Path;

#[test]
fn wrapping_residue_number() {
    let (pdb, errors) = pdbtbx::open("example-pdbs/eq.pdb", StrictnessLevel::Strict).unwrap();
    let pdb_errors = save(pdb.clone(), &("dump/eq.pdb"), StrictnessLevel::Loose);
    let (pdb2, _) = pdbtbx::open("dump/eq.pdb", StrictnessLevel::Strict).unwrap();
    print!("{:?}", errors);
    print!("{:?}", pdb_errors);
    assert_eq!(pdb, pdb2);
}
