use pdbtbx::*;

#[test]
fn insertion_codes() {
    let (pdb, errors) =
        pdbtbx::open("example-pdbs/insertion_codes.pdb", StrictnessLevel::Strict).unwrap();
    let pdb_errors = save(
        pdb.clone(),
        &("dump/insertion_codes.pdb"),
        StrictnessLevel::Loose,
    );
    let (pdb2, _) = pdbtbx::open("dump/insertion_codes.pdb", StrictnessLevel::Strict).unwrap();
    print!("{:?}", errors);
    print!("{:?}", pdb_errors);
    // See that the original file is the same as saved and reopened
    assert_eq!(pdb, pdb2);
    assert_eq!(pdb.residues().count(), 2);
    assert_eq!(pdb.residue(0).unwrap().insertion_code().unwrap(), "A");
    assert_eq!(pdb.residue(1).unwrap().insertion_code().unwrap(), "B");
    assert_eq!(pdb2.residues().count(), 2);
    assert_eq!(pdb2.residue(0).unwrap().insertion_code().unwrap(), "A");
    assert_eq!(pdb2.residue(1).unwrap().insertion_code().unwrap(), "B");
}
