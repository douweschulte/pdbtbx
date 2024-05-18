/// Give a high level interface for users
mod general;
/// Parse mmCIF/PDBx files
mod mmcif;
/// Read options
mod read_options;

/// Parse PDB files
mod pdb;
use super::check_extension;

pub use general::{open, open_gz, open_gz_with_options, open_raw, open_with_options};
pub use mmcif::{
    open_mmcif, open_mmcif_bufread, open_mmcif_bufread_with_options, open_mmcif_raw,
    open_mmcif_raw_with_options, open_mmcif_with_options,
};
pub use pdb::{open_pdb, open_pdb_raw, open_pdb_raw_with_options, open_pdb_with_options};
pub use read_options::{Format, ReadOptions};
