use context_error::{BoxedError, Context, CreateError};
use flate2::Compression;

use super::*;
use crate::structs::PDB;
use crate::StrictnessLevel;
use crate::{check_extension, ErrorLevel};

/// Save the given PDB struct to the given file, validating it beforehand.
/// If validation gives rise to problems, use the `save_raw` function. The correct file
/// type (pdb or mmCIF/PDBx) will be determined based on the given file extension.
/// # Errors
/// Fails if the validation fails with the given `level`.
pub fn save(
    pdb: &PDB,
    filename: impl AsRef<str>,
    level: StrictnessLevel,
) -> Result<(), Vec<BoxedError<'static, ErrorLevel>>> {
    if check_extension(&filename, "pdb") {
        save_pdb(pdb, filename, level)
    } else if check_extension(&filename, "cif") {
        save_mmcif(pdb, filename, level)
    } else {
        Err(vec![BoxedError::new(
            ErrorLevel::BreakingError,
            "Incorrect extension",
            "Could not determine the type of the given file, make it .pdb or .cif",
            Context::default().source(filename.as_ref().to_string()),
        )])
    }
}

/// Save the given PDB struct to the given file and compressing to gz, validating it beforehand.
/// If validation gives rise to problems, use the `save_raw` function. The correct file
/// type (pdb or mmCIF/PDBx) will be determined based on the given file extension.
/// # Errors
/// Fails if the validation fails with the given `level`.
pub fn save_gz(
    pdb: &PDB,
    filename: impl AsRef<str>,
    level: StrictnessLevel,
    compression_level: Option<Compression>,
) -> Result<(), Vec<BoxedError<'static, ErrorLevel>>> {
    let filename = filename.as_ref();
    if check_extension(filename, "gz") {
        // safety check to prevent out of bounds indexing
        if filename.len() < 3 {
            return Err(vec![BoxedError::new(
                ErrorLevel::BreakingError,
                "Filename too short",
                "Could not determine the type of the given file, make it .pdb.gz or .cif.gz",
                Context::default().source(filename.to_string()),
            )]);
        }

        if check_extension(&filename[..filename.len() - 3], "pdb") {
            save_pdb_gz(pdb, filename, level, compression_level)
        } else if check_extension(&filename[..filename.len() - 3], "cif") {
            save_mmcif_gz(pdb, filename, level, compression_level)
        } else {
            Err(vec![BoxedError::new(
                ErrorLevel::BreakingError,
                "Incorrect extension",
                "Could not determine the type of the given file, make it .pdb.gz or .cif.gz",
                Context::default().source(filename.to_string()),
            )])
        }
    } else {
        Err(vec![BoxedError::new(
            ErrorLevel::BreakingError,
            "Incorrect extension",
            "Could not determine the type of the given file, make it .pdb.gz or .cif.gz",
            Context::default().source(filename.to_string()),
        )])
    }
}
