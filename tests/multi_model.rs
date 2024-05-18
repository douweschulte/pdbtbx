// Test only reading the first model from PDB files.

use pdbtbx::*;

#[test]
fn main() {
    // PDB parser
    assert_eq!(50, count_models("example-pdbs/pTLS-6484.pdb", false));
    assert_eq!(1, count_models("example-pdbs/pTLS-6484.pdb", true));

    // mmCIF parser
    assert_eq!(50, count_models("example-pdbs/pTLS-6484.cif", false));
    assert_eq!(1, count_models("example-pdbs/pTLS-6484.cif", true));
}

fn count_models(filename: &str, only_first_model: bool) -> usize {
    let (structure, _errors) = pdbtbx::open_with_options(
        filename,
        ReadOptions::default()
            .set_level(StrictnessLevel::Loose)
            .set_only_first_model(only_first_model),
    )
    .unwrap();

    structure.model_count()
}
