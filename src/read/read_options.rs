use std::{ffi::OsStr, path::Path};

use crate::{Context, PDBError, StrictnessLevel};

use super::general::ReadResult;

/// Used to set which format to read the file in.
#[derive(Debug, Clone, Copy, Default)]
pub enum Format {
    /// Load PDB files
    Pdb,
    /// Load mmCIF files
    Mmcif,
    /// Automatically detect the format
    #[default]
    Auto,

    Pdbqt,
}

impl From<&str> for Format {
    fn from(s: &str) -> Self {
        match s {
            "pdb" => Self::Pdb,
            "mmcif" => Self::Mmcif,
            _ => panic!("Unknown format: {}", s),
        }
    }
}

/// Options and flags which can be used to configure how a structure file is
/// opened.
///
/// This builder exposes the ability to configure how a [`PDB`] is loaded.
///
/// Generally speaking, when using `ReadOptions`, you'll first call
/// [`ReadOptions::new`], then chain calls to methods to set each option, then
/// call [`ReadOptions::read`]. All Boolean options are `false` by default.
///
/// # Examples
///
/// Opening a file to read:
///
/// ```no_run
/// use pdbtbx::*;
///
/// let pdb = ReadOptions::new()
///     .set_format(Format::Auto)
///     .set_level(StrictnessLevel::Loose)
///     .set_discard_hydrogens(true)
///     .read("1CRN.pdb");
/// ```
///
/// The format of the file is inferred by [`ReadOptions::guess_format`]
/// when it is not set explicitly with [`ReadOptions::set_format`].
#[derive(Debug, Default)]
pub struct ReadOptions {
    /// The format to read the file in.
    pub(crate) format: Format,

    /// The strictness level to use when reading the file.
    pub(crate) level: StrictnessLevel,

    /// Controls whether to capitalise the chains in the structure.
    pub(crate) capitalise_chains: bool,

    /// Decompress
    #[cfg(feature = "compression")]
    pub(crate) decompress: bool,

    /// Discard hydrogens
    pub(crate) discard_hydrogens: bool,

    /// Only read the first model
    pub(crate) only_first_model: bool,

    /// Only read atomic coordinates
    pub(crate) only_atomic_coords: bool,
}

impl ReadOptions {
    /// Constructs a new [`ReadOptions`] object with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the format to read the file in.
    pub fn set_format(&mut self, format: Format) -> &mut Self {
        self.format = format;
        self
    }

    /// Guess the file format based on the file name extensions.
    pub fn guess_format(&mut self, filename: &str) -> &mut Self {
        if let Some((file_format, is_compressed)) = guess_format(filename) {
            self.set_decompress(is_compressed).set_format(file_format)
        } else {
            self
        }
    }

    /// Sets the strictness level to use when reading the file.
    pub fn set_level(&mut self, level: StrictnessLevel) -> &mut Self {
        self.level = level;
        self
    }

    /// Sets whether to capitalise the chains in the structure.
    pub fn set_capitalise_chains(&mut self, capitalise_chains: bool) -> &mut Self {
        self.capitalise_chains = capitalise_chains;
        self
    }

    /// Sets whether to decompress the file.
    #[cfg(feature = "compression")]
    pub fn set_decompress(&mut self, decompress: bool) -> &mut Self {
        self.decompress = decompress;
        self
    }

    /// Sets whether to discard hydrogens.
    pub fn set_discard_hydrogens(&mut self, discard_hydrogens: bool) -> &mut Self {
        self.discard_hydrogens = discard_hydrogens;
        self
    }

    /// Sets whether to only keep the first model.
    pub fn set_only_first_model(&mut self, only_first_model: bool) -> &mut Self {
        self.only_first_model = only_first_model;
        self
    }

    /// Sets whether to only parse `ATOM` records in the model file.
    pub fn set_only_atomic_coords(&mut self, only_atomic_coords: bool) -> &mut Self {
        self.only_atomic_coords = only_atomic_coords;
        self
    }

    /// Open an atomic data file, either PDB or mmCIF/PDBx, into a [`PDB`] structure.
    /// The correct type will be determined based on the file extension.
    ///
    /// # Errors
    /// Returns a `PDBError` if a `BreakingError` is found. Otherwise it returns the PDB with all errors/warnings found while parsing it.
    ///
    /// # Related
    /// If you want to open a file from memory, see [`ReadOptions::read_raw`].
    /// If your file extensions are not canonical, set the format explicitly with [`ReadOptions::set_format`].
    pub fn read(&self, path: impl AsRef<str>) -> ReadResult {
        if self.decompress {
            // open a decompression stream
            let filename = path.as_ref();

            self.read_auto(filename)
        } else {
            match self.format {
                Format::Pdb | Format::Pdbqt => super::pdb::open_pdb_with_options(path, self),
                Format::Mmcif => super::mmcif::open_mmcif_with_options(path, self),
                Format::Auto => self.read_auto(path),
            }
        }
    }

    /// Open an atomic data file, either PDB or mmCIF/PDBx, into a [`PDB`] structure
    /// and automatically determine the file type based on the extension of `path`.
    fn read_auto(&self, path: impl AsRef<str>) -> ReadResult {
        let filename = path.as_ref();
        if let Some((file_format, is_compressed)) = guess_format(filename) {
            if is_compressed {
                let file = std::fs::File::open(filename).map_err(|_| {
                    vec![PDBError::new(
                        crate::ErrorLevel::BreakingError,
                        "Could not open file",
                        "Could not open the given file, make sure it exists and you have the correct permissions",
                        Context::show(filename),
                    )]
                })?;
                let decompressor = flate2::read::GzDecoder::new(file);
                let reader = std::io::BufReader::new(decompressor);
                match file_format {
                    Format::Pdb | Format::Pdbqt => {
                        super::pdb::open_pdb_raw_with_options(reader, Context::None, self)
                    }
                    Format::Mmcif => super::mmcif::open_mmcif_raw_with_options(reader, self),
                    Format::Auto => Err(vec![PDBError::new(
                        crate::ErrorLevel::BreakingError,
                        "Could not determine file type",
                        "Could not determine the type of the gzipped file, use .pdb.gz or .cif.gz",
                        Context::show(filename),
                    )]),
                }
            } else {
                match file_format {
                    Format::Pdb | Format::Pdbqt => super::pdb::open_pdb_with_options(path, self),
                    Format::Mmcif => super::mmcif::open_mmcif_with_options(path, self),
                    _ => Err(vec![PDBError::new(
                        crate::ErrorLevel::BreakingError,
                        "Incorrect extension",
                        "Could not determine the type of the given file extension, make it .pdb or .cif",
                        Context::show(path.as_ref()),
                    )])
                }
            }
        } else {
            Err(vec![PDBError::new(
                crate::ErrorLevel::BreakingError,
                "Missing extension",
                "The given file does not have an extension, make it .pdbqt",
                Context::show(path.as_ref()),
            )])
        }
    }

    /// Parse the input stream into a [`PDB`] struct. To allow for direct streaming from sources, like from RCSB.org.
    /// The file format **must** be set explicitly with [`ReadOptions::set_format`].
    /// Returns a PDBError if a BreakingError is found. Otherwise it returns the PDB with all errors/warnings found while parsing it.
    ///
    /// # Related
    /// If you want to open a file, see [`ReadOptions::read`].
    pub fn read_raw<T>(&self, input: std::io::BufReader<T>) -> ReadResult
    where
        T: std::io::Read,
    {
        match self.format {
            Format::Pdb | Format::Pdbqt => {
                super::pdb::open_pdb_raw_with_options(input, Context::None, self)
            }
            Format::Mmcif => super::mmcif::open_mmcif_raw_with_options(input, self),
            Format::Auto => Err(vec![PDBError::new(
                crate::ErrorLevel::BreakingError,
                "Could not determine file type",
                "Could not determine the type of the input stream, set self.format",
                Context::None,
            )]),
        }
    }
}

/// Guess the file format based on the file name extensions.
fn guess_format(filename: &str) -> Option<(Format, bool)> {
    let path = Path::new(filename);

    match path.extension().and_then(OsStr::to_str) {
        Some("pdbqt") => Some((Format::Pdbqt, false)),
        Some("pdb") | Some("pdb1") => Some((Format::Pdb, false)),
        Some("cif") | Some("mmcif") => Some((Format::Mmcif, false)),
        Some("gz") => {
            let path_ext = Path::new(path.file_stem().and_then(OsStr::to_str).unwrap_or(""));
            match path_ext.extension().and_then(OsStr::to_str) {
                Some("pdb") | Some("pdb1") => Some((Format::Pdb, true)),
                Some("cif") | Some("mmcif") => Some((Format::Mmcif, true)),
                _ => None,
            }
        }
        _ => None,
    }
}
