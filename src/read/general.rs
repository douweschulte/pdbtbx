use crate::error::*;
use crate::StrictnessLevel;
use crate::structs::PDB;

use super::*;

/// Standard return type for reading a file.
pub type ReadResult = Result<(PDB, Vec<PDBError>), Vec<PDBError>>;

/// Open an atomic data file, either PDB or mmCIF/PDBx.
///
/// This function is equivalent to [`ReadOptions::read()`] with default options.
/// The correct type will be determined based on the file extension.
/// Gzipped files can also be opened directly if file extensions are
/// `.pdb.gz`, `.pdb1.gz`, `.mmcif.gz`, or `.cif.gz`.
///
/// # Errors
/// Returns a `PDBError` if a `BreakingError` is found. Otherwise it returns the PDB with all errors/warnings found while parsing it.
///
/// # Related
/// If you want to open a file from memory see [`ReadOptions::read_raw`].
/// The file type can be set explicitly with [`ReadOptions::set_format`].
/// These functions are useful if you are using a non-standard compression algorithm or way of
/// storing the data.
pub fn open(filename: impl AsRef<str>) -> ReadResult {
    ReadOptions::default().read(filename)
}

/// Open a compressed atomic data file, either PDB or mmCIF/PDBx. The correct type will be
/// determined based on the file extension (.pdb.gz or .cif.gz).
///
/// # Errors
/// Returns a `PDBError` if a `BreakingError` is found. Otherwise it returns the PDB with all errors/warnings found while parsing it.
///
/// # Related
/// If you want to open a file from memory see [`ReadOptions::read_raw`].
/// The file type can be set explicitly with [`ReadOptions::set_format`].
/// These functions are useful if you are using a non-standard compression algorithm or way of
/// storing the data.
#[cfg(feature = "compression")]
#[deprecated(
    since = "0.12.0",
    note = "Please use `ReadOptions::default().set_decompress(true).read(filename)` instead"
)]
pub fn open_gz(filename: impl AsRef<str>, level: StrictnessLevel) -> ReadResult {
    ReadOptions::default()
        .set_level(level)
        .guess_format(filename.as_ref())
        .read(filename)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_invalid() {
        assert!(open("file.png").is_err());
        assert!(open("file.mmcif").is_err());
        assert!(open("file.pdbml").is_err());
        assert!(open("file.pd").is_err());
    }

    #[test]
    fn open_not_existing() {
        let pdb = open("file.pdb").expect_err("This file should not exist.");
        assert_eq!(pdb[0].short_description(), "Could not open file");
        let cif = open("file.cif").expect_err("This file should not exist.");
        assert_eq!(cif[0].short_description(), "Could not open file");
    }
}
