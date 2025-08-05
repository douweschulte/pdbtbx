#![allow(deprecated)] // Keep exporting them until fully removed

pub use general::{open, open_gz};
pub use mmcif::{open_mmcif, open_mmcif_bufread, open_mmcif_raw};
pub use pdb::{open_pdb, open_pdb_raw};
pub use read_options::{Format, ReadOptions};

/// Give a high level interface for users
mod general;
/// Parse mmCIF/PDBx files
mod mmcif;
/// Read options
mod read_options;

/// Parse PDB files
mod pdb;
