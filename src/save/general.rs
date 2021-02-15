use super::*;
use crate::error::*;
use crate::structs::PDB;
use crate::StrictnessLevel;

/// Save the given PDB struct to the given file.
/// It validates the PDB. It fails if the validation fails with the given `level`.
/// If validation gives rise to problems use the `save_raw` function. The correct file
/// type (pdb or mmCIF/PDBx) will be determined based on the extension of the file.
pub fn save(pdb: PDB, filename: &str, level: StrictnessLevel) -> Result<(), Vec<PDBError>> {
    if filename.ends_with(".pdb") {
        save_pdb(pdb, filename, level)
    } else if filename.ends_with(".cif") {
        save_mmcif(pdb, filename, level, "NONE")
    } else {
        Err(vec![PDBError::new(
            ErrorLevel::BreakingError,
            "Incorrect extension",
            "Could not determine the type of the given file, make it .pdb or .cif",
            Context::show(filename),
        )])
    }
}
