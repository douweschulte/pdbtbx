//! Open a test file containing 87449 waters so with more than 29000 residues which leads to residue serial numbers that are wrapped

use pdbtbx::*;

#[test]
fn wrapping_residue_number() {
    let (pdb, errors) = ReadOptions::default()
        .set_level(StrictnessLevel::Strict)
        .read("example-pdbs/eq.pdb")
        .unwrap();
    let pdb_errors = save(&pdb, "dump/eq.pdb", StrictnessLevel::Loose);
    let (pdb2, _) = ReadOptions::default()
        .set_level(StrictnessLevel::Strict)
        .read("dump/eq.pdb")
        .unwrap();
    print!("{errors:?}");
    print!("{pdb_errors:?}");
    // See that the original file is the same as saved and reopened
    assert_eq!(pdb, pdb2);
    // See that it is possible to select atom with 'impossible' residue serial numbers according to the PDB definition
    // These are made by adding 10000 to the residue serial number every time a wrap is detected (9999 followed by 0)
    assert_eq!(
        pdb.residues()
            .find(|r| r.serial_number() == 10005)
            .unwrap()
            .name()
            .unwrap(),
        "HOH"
    );
    assert_eq!(
        pdb.residues()
            .find(|r| r.serial_number() == 20250)
            .unwrap()
            .name()
            .unwrap(),
        "HOH"
    );
}
