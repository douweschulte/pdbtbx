use std::io::{BufRead, Read, Seek};

use super::*;
use crate::error::*;
use crate::structs::PDB;
use crate::StrictnessLevel;

/// Open an atomic data file, either PDB or mmCIF/PDBx. The correct type will be
/// determined based on the file extension.
///
/// # Errors
/// Returns a `PDBError` if a `BreakingError` is found. Otherwise it returns the PDB with all errors/warnings found while parsing it.
///
/// # Related
/// If you want to open a file from memory see [`open_raw`]. There are also function to open a specified file type directly
/// see [`crate::open_pdb`] and [`crate::open_mmcif`] respectively.
pub fn open(
    filename: impl AsRef<str>,
    level: StrictnessLevel,
) -> Result<(PDB, Vec<PDBError>), Vec<PDBError>> {
    if check_extension(&filename, "pdb") {
        open_pdb(filename, level)
    } else if check_extension(&filename, "cif") {
        open_mmcif(filename, level)
    } else {
        Err(vec![PDBError::new(
            ErrorLevel::BreakingError,
            "Incorrect extension",
            "Could not determine the type of the given file, make it .pdb or .cif",
            Context::show(filename.as_ref()),
        )])
    }
}

/// Open a stream with either PDB or mmCIF data. The distinction is made on the start of the first line.
/// If it starts with `HEADER` it is a PDB file, if it starts with `data_` it is a mmCIF file.
///
/// # Errors
/// Returns a `PDBError` if a `BreakingError` is found. Otherwise it returns the PDB with all errors/warnings found while parsing it.
/// It returns a breaking error if the buffer could not be read, the file type could not be determined form the first line, or there was a breaking error in the file itself.
/// See the `PDBError` for more details.
///
/// # Related
/// If you want to open a file see [`open`]. There are also function to open a specified file type directly
/// see [`crate::open_pdb_raw`] and [`crate::open_mmcif_raw`] respectively.
pub fn open_raw<T: std::io::Read + std::io::Seek>(
    mut input: std::io::BufReader<T>,
    level: StrictnessLevel,
) -> Result<(PDB, Vec<PDBError>), Vec<PDBError>> {
    let mut first_line = String::new();
    if input.read_line(&mut first_line).is_err() {
        return Err(vec![PDBError::new(
            ErrorLevel::BreakingError,
            "Buffer could not be read",
            "The buffer provided to `open_raw` could not be read.",
            Context::None,
        )]);
    }
    if input.rewind().is_err() {
        return Err(vec![PDBError::new(
            ErrorLevel::BreakingError,
            "Buffer could not be read",
            "The buffer provided to `open_raw` could not be rewound to the start.",
            Context::None,
        )]);
    }
    if first_line.starts_with("HEADER") {
        open_pdb_raw(input, Context::None, level)
    } else if first_line.starts_with("data_") {
        let mut contents = String::new();
        if input.read_to_string(&mut contents).is_ok() {
            open_mmcif_raw(&contents, level)
        } else {
            Err(vec![PDBError::new(
                ErrorLevel::BreakingError,
                "Buffer could not be read",
                "The buffer provided to `open_raw` could not be read to end.",
                Context::show(&first_line),
            )])
        }
    } else {
        Err(vec![PDBError::new(
            ErrorLevel::BreakingError,
            "Could not determine file type",
            "Could not determine the type of the given file, make it .pdb or .cif",
            Context::show(&first_line),
        )])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_invalid() {
        assert!(open("file.png", StrictnessLevel::Medium).is_err());
        assert!(open("file.mmcif", StrictnessLevel::Medium).is_err());
        assert!(open("file.pdbml", StrictnessLevel::Medium).is_err());
        assert!(open("file.pd", StrictnessLevel::Medium).is_err());
    }

    #[test]
    fn open_not_existing() {
        let pdb = open("file.pdb", StrictnessLevel::Medium).unwrap_err();
        assert_eq!(pdb[0].short_description(), "Could not open file");
        let cif = open("file.cif", StrictnessLevel::Medium).unwrap_err();
        assert_eq!(cif[0].short_description(), "Could not open file");
    }
}
