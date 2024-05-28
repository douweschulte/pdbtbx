use super::*;
use crate::error::*;
use crate::structs::PDB;
use crate::StrictnessLevel;

/// Standard return type for reading a file.
pub type ReadResult = Result<(PDB, Vec<PDBError>), Vec<PDBError>>;

/// Open an atomic data file, either PDB or mmCIF/PDBx. The correct type will be
/// determined based on the file extension. This function is equivalent to
/// [`ReadOptions::read()`] with default options.
///
/// # Errors
/// Returns a `PDBError` if a `BreakingError` is found. Otherwise it returns the PDB with all errors/warnings found while parsing it.
///
/// # Related
/// If you want to open a file from memory see [`open_raw`]. There are also function to open a specified file type directly
/// see [`open_pdb`] and [`open_mmcif`] respectively.
pub fn open(filename: impl AsRef<str>) -> ReadResult {
    open_with_options(filename, &ReadOptions::default())
}

/// Opens a files based on the given options.
///
/// # Related
/// See [`open`] for a version of this function with sane defaults.
pub fn open_with_options(filename: impl AsRef<str>, options: &ReadOptions) -> ReadResult {
    options.read(filename)
}

/// Open a compressed atomic data file, either PDB or mmCIF/PDBx. The correct type will be
/// determined based on the file extension (.pdb.gz or .cif.gz).
///
/// # Errors
/// Returns a `PDBError` if a `BreakingError` is found. Otherwise it returns the PDB with all errors/warnings found while parsing it.
///
/// # Related
/// If you want to open a file from memory see [`open_raw`], [`open_pdb_raw`] and [`open_mmcif_bufread`].
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
/// see [`open_pdb_raw`] and [`open_mmcif_raw`] respectively.
pub fn open_raw<T: std::io::Read + std::io::Seek>(input: std::io::BufReader<T>) -> ReadResult {
    ReadOptions::default().read_raw(input)
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
