use pdbtbx::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Open a test file containing more than 9999 residues and 99999 atoms, save it and check if the
/// saved file was properly clipped.
#[test]
fn clipped() {
    let (pdb, errors) = pdbtbx::open("example-pdbs/large.pdb", StrictnessLevel::Strict).unwrap();
    let pdb_errors = save(pdb.clone(), &("dump/large.pdb"), StrictnessLevel::Loose);
    print!("{:?}", errors);
    print!("{:?}", pdb_errors);
    let file = File::open("dump/large.pdb").unwrap();
    let mut buffer = BufReader::new(file).lines();
    let target = "ATOM  8662  H2   WAT  5372       7.739  79.053  26.313  1.00  0.00          H";
    let target_line = buffer.find(|l| {
        if let Ok(line) = l {
            line.trim() == target
        } else {
            false
        }
    });
    assert!(target_line.is_some())
}
