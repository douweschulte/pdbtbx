/// Give a high level interface for users
mod general;
/// Save mmCIF/PDBx files
mod mmcif;
/// Save PDB files
mod pdb;

pub use general::{save, save_gz};
pub use mmcif::{save_mmcif, save_mmcif_gz, save_mmcif_raw};
pub use pdb::{save_pdb, save_pdb_gz, save_pdb_raw};
