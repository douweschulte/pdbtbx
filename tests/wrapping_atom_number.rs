//! Open a test file containing more than 9999 residues and 99999 atoms which leads to atom and residue serial numbers that are wrapped

use pdbtbx::*;

#[test]
fn wrapping_residue_number() {
    let (pdb, errors) = ReadOptions::default()
        .set_level(StrictnessLevel::Strict)
        .read("example-pdbs/large.pdb")
        .unwrap();
    let pdb_errors = save(&pdb, "dump/large.pdb", StrictnessLevel::Loose);
    let (pdb2, _) = ReadOptions::default()
        .set_level(StrictnessLevel::Strict)
        .read("dump/large.pdb")
        .unwrap();
    print!("{errors:?}");
    print!("{pdb_errors:?}");
    // See that the original file is the same as saved and reopened
    assert_eq!(pdb, pdb2);
    // See that it is possible to select atom with 'impossible' atom serial numbers according to the PDB definition
    // These are made by adding 100000 to the atom serial number every time a wrap is detected (99999 followed by 0)
    assert_eq!(
        pdb.atoms()
            .find(|a| a.serial_number() == 100_005)
            .unwrap()
            .pos(),
        (28.212, 27.833, 14.033)
    );
    assert_eq!(
        pdb.atoms()
            .find(|a| a.serial_number() == 120_830)
            .unwrap()
            .pos(),
        (14.041, 8.886, 15.800)
    );
}
