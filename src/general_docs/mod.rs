//! Here you can find high level documentation for this crate.
//! * About saving: [`mod@save`]
//! * About the PDB hierarchy used: [`structs`]
//!
//! Good references for more in depth information:
//! * PDB: [spec](https://www.wwpdb.org/documentation/file-format-content/format33/v3.3.html)
//! * mmCIF: [spec](https://mmcif.wwpdb.org/dictionaries/mmcif_pdbx_v50.dic/Index/) [docs](https://mmcif.wwpdb.org/dictionaries/mmcif_pdbx_v50.dic/Groups/index.html)
//! * PDB to mmCIF conversion: [wwpdb](https://mmcif.wwpdb.org/docs/pdb_to_pdbx_correspondences.html)

use crate::*;

#[doc = include_str!("../save/general.md")]
pub mod save {}

#[doc = include_str!("../structs/general.md")]
pub mod structs {}
