use crate::{Context, PDBError, StrictnessLevel, PDB};

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
//
/// ```
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
    /// If you want to open a file from memory see [`ReadOptions::read_raw`].
    pub fn read(&self, path: impl AsRef<str>) -> ReadResult {
        // open_with_options(path, self)
        if self.decompress {
            super::general::open_gz_with_options(path, self)
        } else {
            match self.format {
                Format::Pdb => super::pdb::open_pdb_with_options(path, self),
                Format::Mmcif => super::mmcif::open_mmcif_with_options(path, self),
                Format::Auto => {
                    if let Some(file_ext) = path.as_ref().rsplit('.').next() {
                        match file_ext {
                            "pdb" | "pdb1" => super::pdb::open_pdb_with_options(path, self),
                            "cif" | "mmcif" => super::mmcif::open_mmcif_with_options(path, self),
                            _ => Err(vec![PDBError::new(
                                crate::ErrorLevel::BreakingError,
                                "Incorrect extension",
                                "Could not determine the type of the given file extension, make it .pdb or .cif",
                                Context::show(path.as_ref()),

                            )])
                        }
                    } else {
                        Err(vec![PDBError::new(
                            crate::ErrorLevel::BreakingError,
                            "Missing extension",
                            "The given file does not have an extension, make it .pdb or .cif",
                            Context::show(path.as_ref()),
                        )])
                    }
                }
            }
        }
    }

    /// Parse the input stream into a [`PDB`] struct. To allow for direct streaming from sources, like from RCSB.org.
    /// Returns a PDBError if a BreakingError is found. Otherwise it returns the PDB with all errors/warnings found while parsing it.
    ///
    /// ## Arguments
    /// * `input` - the input stream
    /// * `context` - the context of the full stream, to place error messages correctly, for files this is `Context::show(filename)`.
    /// * `options` - the options for configuring how a file/stream is parsed.
    ///
    /// # Related
    /// If you want to open a file see [`crate::open_pdb`] and [`crate::open_mmcif`].
    /// If you want to open a general file with no knowledge about the file type, see [`ReadOptions::read`].
    pub fn read_raw<T>(
        &self,
        input: std::io::BufReader<T>,
    ) -> Result<(PDB, Vec<PDBError>), Vec<PDBError>>
    where
        T: std::io::Read,
    {
        match self.format {
            Format::Pdb => super::pdb::open_pdb_raw_with_options(input, Context::None, self),
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
