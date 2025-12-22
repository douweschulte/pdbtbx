//! Test ParsingLevel to selectively parse different PDB record types.

use pdbtbx::*;

#[test]
fn test_parsing_level_all() {
    let (structure, _errors) = ReadOptions::default()
        .set_level(StrictnessLevel::Loose)
        .set_parsing_level(&ParsingLevel::all())
        .read("example-pdbs/1ubq.pdb")
        .unwrap();

    assert!(structure.atom_count() > 0);
}

#[test]
fn test_parsing_level_none() {
    let result = ReadOptions::default()
        .set_level(StrictnessLevel::Loose)
        .set_parsing_level(&ParsingLevel::none())
        .read("example-pdbs/1ubq.pdb");

    assert!(
        result.is_err(),
        "ParsingLevel::none() should fail validation due to no atoms"
    );
}

#[test]
fn test_parsing_level_atom_only_no_hetatm() {
    let mut parsing_level = ParsingLevel::none();
    parsing_level.set_atom(true);

    let (structure, _errors) = ReadOptions::default()
        .set_level(StrictnessLevel::Loose)
        .set_parsing_level(&parsing_level)
        .read("example-pdbs/1ubq.pdb")
        .unwrap();

    assert!(
        structure.atom_count() > 0,
        "Should have atoms from ATOM records"
    );

    for atom in structure.atoms() {
        assert!(
            !atom.hetero(),
            "Found HETATM atom when only ATOM parsing was enabled: {}",
            atom.serial_number()
        );
    }
}

#[test]
fn test_parsing_level_hetatm_only_no_atom() {
    let mut parsing_level = ParsingLevel::none();
    parsing_level.set_hetatm(true);

    let (structure, _errors) = ReadOptions::default()
        .set_level(StrictnessLevel::Loose)
        .set_parsing_level(&parsing_level)
        .read("example-pdbs/1ubq.pdb")
        .unwrap();

    for atom in structure.atoms() {
        assert!(
            atom.hetero(),
            "Found ATOM record when only HETATM parsing was enabled: {}",
            atom.serial_number()
        );
    }
}

#[test]
fn test_parsing_level_no_hetatm() {
    let mut parsing_level = ParsingLevel::all();
    parsing_level.set_hetatm(false);

    let (structure_no_hetatm, _errors) = ReadOptions::default()
        .set_level(StrictnessLevel::Loose)
        .set_parsing_level(&parsing_level)
        .read("example-pdbs/1ubq.pdb")
        .unwrap();

    for atom in structure_no_hetatm.atoms() {
        assert!(
            !atom.hetero(),
            "Found HETATM atom when hetatm parsing was disabled: {}",
            atom.serial_number()
        );
    }

    let (structure_all, _errors) = ReadOptions::default()
        .set_level(StrictnessLevel::Loose)
        .read("example-pdbs/1ubq.pdb")
        .unwrap();

    let no_hetatm_count = structure_no_hetatm.atom_count();
    let all_count = structure_all.atom_count();

    assert!(
        no_hetatm_count < all_count,
        "Should have fewer atoms without HETATM records (no_hetatm: {}, all: {})",
        no_hetatm_count,
        all_count
    );
}

#[test]
fn test_parsing_level_no_atom() {
    let mut parsing_level = ParsingLevel::all();
    parsing_level.set_atom(false);

    let (structure_no_atom, _errors) = ReadOptions::default()
        .set_level(StrictnessLevel::Loose)
        .set_parsing_level(&parsing_level)
        .read("example-pdbs/1ubq.pdb")
        .unwrap();

    for atom in structure_no_atom.atoms() {
        assert!(
            atom.hetero(),
            "Found ATOM record when atom parsing was disabled: {}",
            atom.serial_number()
        );
    }

    let (structure_all, _errors) = ReadOptions::default()
        .set_level(StrictnessLevel::Loose)
        .read("example-pdbs/1ubq.pdb")
        .unwrap();

    let no_atom_count = structure_no_atom.atom_count();
    let all_count = structure_all.atom_count();

    assert!(
        no_atom_count < all_count,
        "Should have fewer atoms without ATOM records (no_atom: {}, all: {})",
        no_atom_count,
        all_count
    );
}

#[test]
fn test_parsing_level_atom_and_hetatm() {
    let mut parsing_level = ParsingLevel::none();
    parsing_level.set_atom(true).set_hetatm(true);

    let (structure, _errors) = ReadOptions::default()
        .set_level(StrictnessLevel::Loose)
        .set_parsing_level(&parsing_level)
        .read("example-pdbs/1ubq.pdb")
        .unwrap();

    assert!(structure.atom_count() > 0, "Should have atoms");

    let (structure_default, _errors) = ReadOptions::default()
        .set_level(StrictnessLevel::Loose)
        .read("example-pdbs/1ubq.pdb")
        .unwrap();

    assert_eq!(
        structure.atom_count(),
        structure_default.atom_count(),
        "Enabling both ATOM and HETATM should match default behavior"
    );

    let has_atom = structure.atoms().any(|atom| !atom.hetero());
    let has_hetatm = structure.atoms().any(|atom| atom.hetero());

    assert!(has_atom, "Should have at least one ATOM record");
    assert!(has_hetatm, "Should have at least one HETATM record");
}
