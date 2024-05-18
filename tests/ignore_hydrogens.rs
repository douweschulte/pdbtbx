// Test reading PDB file and discarding all hydrogen atoms.

use pdbtbx::*;

#[test]
fn main() {
    let (full_structure, _errors) = pdbtbx::open_with_options(
        "example-pdbs/rosetta_model.pdb",
        ReadOptions::default().set_level(StrictnessLevel::Loose),
    )
    .unwrap();

    let num_h = full_structure.atoms().fold(0, |acc, a| {
        acc + usize::from(a.element() == Some(&Element::H))
    });

    let (structure, _errors) = pdbtbx::open_with_options(
        "example-pdbs/rosetta_model.pdb",
        ReadOptions::default()
            .set_level(StrictnessLevel::Loose)
            .set_discard_hydrogens(true),
    )
    .unwrap();

    let num_without_h = structure.atoms().fold(0, |acc, a| {
        acc + usize::from(a.element() == Some(&Element::H))
    });

    assert_eq!(947, num_h);
    assert_eq!(0, num_without_h);
}
