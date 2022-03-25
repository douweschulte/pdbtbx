/// Give a high level interface for users
mod general;
/// Parse mmCIF/PDBx files
mod mmcif;
/// Parse PDB files
mod pdb;
use super::check_extension;

pub use general::open;
pub use mmcif::open_mmcif;
pub use pdb::{open_pdb, open_pdb_raw};
