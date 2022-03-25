use super::*;
use crate::structs::PDB;
use crate::StrictnessLevel;
use crate::{check_extension, error::*};

/// Save the given PDB struct to the given file, validating it beforehand.
/// If validation gives rise to problems, use the `save_raw` function. The correct file
/// type (pdb or mmCIF/PDBx) will be determined based on the given file extension.
/// # Errors
/// Fails if the validation fails with the given `level`.
pub fn save(pdb: &PDB, filename: &str, level: StrictnessLevel) -> Result<(), Vec<PDBError>> {
    if check_extension(filename, "pdb") {
        save_pdb(pdb, filename, level)
    } else if check_extension(filename, "cif") {
        save_mmcif(pdb, filename, level)
    } else {
        Err(vec![PDBError::new(
            ErrorLevel::BreakingError,
            "Incorrect extension",
            "Could not determine the type of the given file, make it .pdb or .cif",
            Context::show(filename),
        )])
    }
}
