/// Give a high level interface for users
mod general;
/// Save mmCIF/PDBx files
mod mmcif;
/// Save PDB files
mod pdb;

pub use general::save;
#[cfg(feature = "compression")]
pub use general::save_gz;
#[cfg(feature = "compression")]
pub use mmcif::save_mmcif_gz;
pub use mmcif::{save_mmcif, save_mmcif_raw};
#[cfg(feature = "compression")]
pub use pdb::save_pdb_gz;
pub use pdb::{save_pdb, save_pdb_raw};
