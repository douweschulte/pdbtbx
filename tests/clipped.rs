//! Open a test file containing more than 9999 residues and 99999 atoms, save it and check if the
//! saved file was properly clipped.

use pdbtbx::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[test]
fn clipped() {
    let root = env!("CARGO_MANIFEST_DIR");
    let path = format!("{}/{}", root, "example-pdbs/large.pdb");
    let dump_dir = format!("{}/{}", root, "dump");

    // make dumps directory
    std::fs::create_dir_all(dump_dir).unwrap();

    let (pdb, errors) = ReadOptions::default()
        .set_level(StrictnessLevel::Strict)
        .read(path)
        .unwrap();
    let pdb_errors = save(&pdb, "dump/large.pdb", StrictnessLevel::Loose);
    print!("{errors:?}");
    print!("{pdb_errors:?}");
    let file = File::open("dump/large.pdb").unwrap();
    let mut buffer = BufReader::new(file).lines();
    let target = "ATOM  8662  H2   WAT C5372       7.739  79.053  26.313  1.00  0.00          H";
    let target_line = buffer.find(|l| l.as_ref().map_or(false, |line| line.trim() == target));
    assert!(target_line.is_some());
}
