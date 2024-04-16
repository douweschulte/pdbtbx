use crate::StrictnessLevel;

use super::general::{open_with_options, ReadResult};

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
/// call [`ReadOptions::read`].
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

    /// Reads a file into a [`PDB`] structure.
    pub fn read(&self, path: &str) -> ReadResult {
        open_with_options(path, self)
    }
}
