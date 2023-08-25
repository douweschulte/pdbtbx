use std::io::{BufRead, Read, Seek};

use super::*;
use crate::error::*;
use crate::structs::PDB;
use crate::StrictnessLevel;

#[cfg(feature = "compression")]
use super::mmcif::open_mmcif_bufread;
#[cfg(feature = "compression")]
use flate2::read::GzDecoder;
#[cfg(feature = "compression")]
use std::fs;

/// Standard return type for reading a file.
pub type ReadResult = std::result::Result<(PDB, Vec<PDBError>), Vec<PDBError>>;

/// Open an atomic data file, either PDB or mmCIF/PDBx. The correct type will be
/// determined based on the file extension. This function is equivalent to
/// [`ReadOptions::read()`] with default options, apart from the `level` which
/// can be set by the `level` parameter.
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
) -> ReadResult {
    open_with_options(filename, &ReadOptions::new().level(level))
}

/// Opens a files based on the given options.
pub(in crate::read) fn open_with_options(filename: impl AsRef<str>, options: &ReadOptions) -> ReadResult {
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

/// Open a compressed atomic data file, either PDB or mmCIF/PDBx. The correct type will be
/// determined based on the file extension (.pdb.gz or .cif.gz).
///
/// # Errors
/// Returns a `PDBError` if a `BreakingError` is found. Otherwise it returns the PDB with all errors/warnings found while parsing it.
///
/// # Related
/// If you want to open a file from memory see [`open_raw`], [`crate::open_pdb_raw`] and [`crate::open_mmcif_bufread`].
/// These functions are useful if you are using a non-standard compression algorithm or way of
/// storing the data.
#[cfg(feature = "compression")]
pub fn open_gz(
    filename: impl AsRef<str>,
    level: StrictnessLevel,
) -> ReadResult {
    let filename = filename.as_ref();

    if check_extension(filename, "gz") {
        // open a decompression stream
        let file = fs::File::open(filename).map_err(|_| {
            vec![PDBError::new(
                ErrorLevel::BreakingError,
                "Could not open file",
                "Could not open the given file, make sure it exists and you have the correct permissions",
                Context::show(filename),
            )]
        })?;

        let decompressor = GzDecoder::new(file);

        let reader = std::io::BufReader::new(decompressor);

        if check_extension(&filename[..filename.len() - 3], "pdb") {
            open_pdb_raw(reader, Context::show(filename), level)
        } else if check_extension(&filename[..filename.len() - 3], "cif") {
            open_mmcif_bufread(reader, level)
        } else {
            Err(vec![PDBError::new(
                ErrorLevel::BreakingError,
                "Incorrect extension",
                "Could not determine the type of the given file, make it .pdb.gz or .cif.gz",
                Context::show(filename),
            )])
        }
    } else {
        Err(vec![PDBError::new(
            ErrorLevel::BreakingError,
            "Incorrect extension",
            "Could not determine the type of the given file, make it .pdb.gz or .cif.gz",
            Context::show(filename),
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
) -> ReadResult {
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
        let pdb =
            open("file.pdb", StrictnessLevel::Medium).expect_err("This file should not exist.");
        assert_eq!(pdb[0].short_description(), "Could not open file");
        let cif =
            open("file.cif", StrictnessLevel::Medium).expect_err("This file should not exist.");
        assert_eq!(cif[0].short_description(), "Could not open file");
    }
}
