//! Test only reading the first model from PDB files.

use pdbtbx::*;

#[test]
fn correct_model_count() {
    // atom IDs shared between models, PDB parser
    assert_eq!(50, count_models("example-pdbs/pTLS-6484.pdb", false));

    // atom IDs shared between models, mmCIF parser
    assert_eq!(50, count_models("example-pdbs/pTLS-6484.cif", false));

    // atom IDs unique over all models, mmCIF parser
    assert_eq!(30, count_models("example-pdbs/3pdz.cif", false));
}

#[test]
fn only_read_first_model() {
    // atom IDs shared between models, PDB parser
    assert_eq!(1, count_models("example-pdbs/pTLS-6484.pdb", true));

    // atom IDs shared between models, mmCIF parser
    assert_eq!(1, count_models("example-pdbs/pTLS-6484.cif", true));

    // atom IDs unique over all models, mmCIF parser
    assert_eq!(1, count_models("example-pdbs/3pdz.cif", true));
}

fn count_models(filename: &str, only_first_model: bool) -> usize {
    let read_result = ReadOptions::default()
        .set_level(StrictnessLevel::Loose)
        .set_only_first_model(only_first_model)
        .read(filename);
    assert!(read_result.is_ok());
    let (structure, _errors) = read_result.unwrap();

    structure.model_count()
}
