use crate::StrictnessLevel;

use super::general::{ReadResult, open_with_options};

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
/// Generally speaking, when using `ReaderOptions`, you'll first call
/// [`OpenOptions::new`], then chain calls to methods to set each option, then
/// call [`OpenOptions::open`], passing the path of the file you're trying to
/// open. This will give you a [`io::Result`] with a [`File`] inside that you
/// can further operate on.
///
/// # Examples
///
/// Opening a file to read:
///
/// ```no_run
/// use pdbtbx::*;
///
/// let pdb = ReadOptions::new()
///     .format(Format::Pdb)
///     .level(StrictnessLevel::Loose)
///     .discard_hydrogens(true)
///     .read("1CRN.pdb");
//
/// ```
#[derive(Debug, Default)]
pub struct ReadOptions {
    /// The format to read the file in.
    format: Format,

    /// The strictness level to use when reading the file.
    level: StrictnessLevel,

    /// Controls whether to capitalise the chains in the structure.
    capitalise_chains: bool,

    /// Decompress
    #[cfg(feature = "compression")]
    decompress: bool,

    /// Discard hydrogens
    discard_hydrogens: bool,

    /// Only read the first model
    only_first_model: bool,
}

impl ReadOptions {
    /// Constructs a new [`ReadOptions`] object with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the format to read the file in.
    pub fn format(&mut self, format: Format) -> &mut Self{
        self.format = format;
        self
    }

    /// Sets the strictness level to use when reading the file.
    pub fn level(&mut self, level: StrictnessLevel) -> &mut Self {
        self.level = level;
        self
    }

    /// Sets whether to capitalise the chains in the structure.
    pub fn capitalise_chains(&mut self, capitalise_chains: bool) -> &mut Self{
        self.capitalise_chains = capitalise_chains;
        self
    }

    /// Sets whether to decompress the file.
    #[cfg(feature = "compression")]
    pub fn decompress(&mut self, decompress: bool) -> &mut Self{
        self.decompress = decompress;
        self
    }

    /// Sets whether to discard hydrogens.
    pub fn discard_hydrogens(&mut self, discard_hydrogens: bool) -> &mut Self {
        self.discard_hydrogens = discard_hydrogens;
        self
    }

    /// Sets whether to only keep the first model.
    pub fn only_first_model(&mut self, only_first_model: bool) -> &mut Self {
        self.only_first_model = only_first_model;
        self
    }

    /// Reads a file into a [`PDB`] structure.
    pub fn read(&self, path: &str) -> ReadResult {
        open_with_options(path, self)
    }
}


