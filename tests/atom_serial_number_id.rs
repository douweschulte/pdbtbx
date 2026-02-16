//! Test serial numbers

use context_error::{BoxedError, StaticErrorContent};
use itertools::Itertools;
use pdbtbx::*;

fn get_structure_with_errors(filename: &str) -> (PDB, Vec<BoxedError<'static, ErrorLevel>>) {
    let read_result = ReadOptions::default()
        .set_level(StrictnessLevel::Loose)
        .read(filename);
    assert!(read_result.is_ok());
    read_result.unwrap()
}

fn get_atoms_by_model(filename: &str) -> Vec<Vec<Atom>> {
    get_structure_with_errors(filename)
        .0
        .models()
        .map(|model| model.atoms().cloned().collect())
        .collect()
}

fn get_atoms(filename: &str) -> Vec<Atom> {
    get_structure_with_errors(filename)
        .0
        .atoms()
        .cloned()
        .collect()
}

#[test]
fn atom_serial_number_and_id_unique_for_single_model_mmcif() {
    let filename_regular = "example-pdbs/rosetta_model.cif";
    let (atom_serial_numbers, atom_ids): (Vec<usize>, Vec<String>) = get_atoms(filename_regular)
        .into_iter()
        .map(|atom| (atom.serial_number(), atom.id().to_string()))
        .unzip();
    assert!(atom_serial_numbers.iter().all_unique());
    assert!(atom_ids.iter().all_unique());
}

#[test]
fn atom_serial_number_and_id_unique_for_single_model_pdb() {
    let filename_regular = "example-pdbs/rosetta_model.pdb";
    let (atom_serial_numbers, atom_ids): (Vec<usize>, Vec<String>) = get_atoms(filename_regular)
        .into_iter()
        .map(|atom| (atom.serial_number(), atom.id().to_string()))
        .unzip();
    assert!(atom_serial_numbers.iter().all_unique());
    assert!(atom_ids.iter().all_unique());
}

#[test]
fn atom_serial_number_and_id_unique_for_single_model_large_pdb() {
    // check for bugs caused by atom numbers larger than PDB format allows
    let filename_large = "example-pdbs/large.pdb";
    let (atom_serial_numbers, atom_ids): (Vec<usize>, Vec<String>) = get_atoms(filename_large)
        .into_iter()
        .map(|atom| (atom.serial_number(), atom.id().to_string()))
        .unzip();
    assert!(atom_serial_numbers.iter().all_unique());
    assert!(atom_ids.iter().all_unique());
}

#[test]
fn atom_serial_number_only_unique_within_model_mmcif() {
    let filename_shared_ids = "example-pdbs/pTLS-6484.cif";
    assert!(get_atoms_by_model(filename_shared_ids)
        .iter()
        .all(|atoms| atoms.iter().map(Atom::serial_number).all_unique()));
    assert!(!get_atoms(filename_shared_ids)
        .iter()
        .map(Atom::serial_number)
        .all_unique());

    let filename_unique_ids = "example-pdbs/3pdz.cif";
    assert!(get_atoms_by_model(filename_unique_ids)
        .iter()
        .all(|atoms| atoms.iter().map(Atom::serial_number).all_unique()));
    assert!(!get_atoms(filename_unique_ids)
        .iter()
        .map(Atom::serial_number)
        .all_unique());
}

#[test]
fn atom_serial_number_only_unique_within_model_pdb() {
    let filename = "example-pdbs/pTLS-6484.pdb";
    assert!(get_atoms_by_model(filename)
        .iter()
        .all(|atoms| atoms.iter().map(Atom::serial_number).all_unique()));
    assert!(!get_atoms(filename)
        .iter()
        .map(Atom::serial_number)
        .all_unique());
}

#[test]
fn atom_id_unique_globally_pdb() {
    let filename = "example-pdbs/pTLS-6484.pdb";
    assert!(get_atoms(filename).iter().map(Atom::id).all_unique());
}

#[test]
fn atom_id_unique_if_unique_in_input_mmcif() {
    let filename_unique_ids = "example-pdbs/3pdz.cif";
    let (structure, _errors) = get_structure_with_errors(filename_unique_ids);
    assert!(structure.atoms().map(Atom::id).all_unique());
}

#[test]
fn warning_if_atom_ids_not_unique_mmcif() {
    let filename_shared_ids = "example-pdbs/pTLS-6484.cif";
    let (structure, errors) = get_structure_with_errors(filename_shared_ids);
    assert!(!structure.atoms().map(Atom::id).all_unique());
    assert!(errors
        .iter()
        .any(|err| err.get_short_description().contains("Duplicated atom IDs")));
}
