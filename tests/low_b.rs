//! Test low b factor errors

use pdbtbx::*;
use std::env;
use std::path::Path;

#[test]
fn low_b_factor_messages() {
    let filename = env::current_dir()
        .unwrap()
        .as_path()
        .join(Path::new("example-pdbs"))
        .join(Path::new("low_b.pdb"))
        .into_os_string()
        .into_string()
        .unwrap();

    let (pdb, errors) = ReadOptions::default()
        .set_level(StrictnessLevel::Strict)
        .read(filename)
        .unwrap();
    let pdb_errors = validate_pdb(&pdb);
    print!("{errors:?}");
    print!("{pdb_errors:?}");
    assert_eq!(errors.len(), 0);
    assert_eq!(pdb_errors.len(), 0);
    assert_eq!(pdb.atom(0).unwrap().b_factor(), 0.00);
    assert_eq!(pdb.atom(1).unwrap().b_factor(), 0.01);
    assert_eq!(pdb.atom(2).unwrap().b_factor(), 999.99);
    assert_eq!(pdb.atom(3).unwrap().occupancy(), 0.00);
    assert_eq!(pdb.atom(4).unwrap().occupancy(), 0.01);
    assert_eq!(pdb.atom(5).unwrap().occupancy(), 999.99);
}
