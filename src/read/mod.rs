/// Give a high level interface for users
mod general;
/// Parse mmCIF/PDBx files
mod mmcif;
/// Read options
mod read_options;

/// Parse PDB files
mod pdb;
use super::check_extension;

pub use general::{open, open_gz, open_raw};
pub use mmcif::{open_mmcif, open_mmcif_raw, open_mmcif_bufread};
pub use pdb::{open_pdb, open_pdb_raw};
pub use read_options::{Format, ReadOptions};