//! Test insertion codes

use pdbtbx::*;

#[test]
fn insertion_codes() {
    let root = env!("CARGO_MANIFEST_DIR");
    let path = format!("{}/{}", root, "example-pdbs/insertion_codes.pdb");
    let dump_dir = format!("{}/{}", root, "dump");
    // make dumps directory
    std::fs::create_dir_all(dump_dir).unwrap();

    let (pdb, errors) = ReadOptions::default()
        .set_level(StrictnessLevel::Strict)
        .read(path)
        .unwrap();
    let pdb_errors = save(&pdb, "dump/insertion_codes.pdb", StrictnessLevel::Loose);
    let (pdb2, _) = ReadOptions::default()
        .set_level(StrictnessLevel::Strict)
        .read("dump/insertion_codes.pdb")
        .unwrap();
    print!("{errors:?}");
    print!("{pdb_errors:?}");
    // See that the original file is the same as saved and reopened
    assert_eq!(pdb, pdb2);
    assert_eq!(pdb.residues().count(), 2);
    assert_eq!(pdb.residue(0).unwrap().insertion_code().unwrap(), "A");
    assert_eq!(pdb.residue(1).unwrap().insertion_code().unwrap(), "B");
    assert_eq!(pdb2.residues().count(), 2);
    assert_eq!(pdb2.residue(0).unwrap().insertion_code().unwrap(), "A");
    assert_eq!(pdb2.residue(1).unwrap().insertion_code().unwrap(), "B");
}
