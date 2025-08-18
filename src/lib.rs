//! # pdbtbx (PDB Toolbox)
//!
//! A library to work with crystallographic Protein DataBank files. It can parse the main part
//! of the PDB and mmCIF format (it is actively in development so more will follow). The resulting structure
//! can be used to edit and interrogate the 3D structure of the protein. The changed structures can
//! be saved in a PDB or mmCIF file for use in other software.
//!
//! ## Goals
//! This library is designed to be a dependable, safe, stable and fast way of handling PDB files
//! in idiomatic Rust. It is the goal to be very community driven, to make it into a project that
//! is as useful to everyone while keeping true to its core principles.
//!
//! ## Why
//! As Rust is a recent language so there is not a lot of support for scientific work in Rust
//! compared to languages that are used much longer (like the ubiquitous Python). I think
//! that using Rust would have huge benefits over other languages in bigger scientific projects.
//! It is not just me, more scientists are turning to Rust [`Perkel, J. M.`]. I want to make it
//! easier for scientists to start using Rust by writing this library.
//!
//! ## How to use it
//! The following example opens a pdb file (`1ubq.pdb`). Removes all `H` atoms. Calculates the
//! average B factor (or temperature factor) and prints that. It also saves the resulting PDB
//! to a file.
//!
//! ```rust
//! use pdbtbx::*;
//! let (mut pdb, _errors) = pdbtbx::open("example-pdbs/1ubq.pdb").unwrap();
//!
//! pdb.remove_atoms_by(|atom| atom.element() == Some(&Element::H)); // Remove all H atoms
//!
//! let mut avg_b_factor = 0.0;
//! for atom in pdb.atoms() { // Iterate over all atoms in the structure
//!     avg_b_factor += atom.b_factor();
//! }
//! avg_b_factor /= pdb.atom_count() as f64;
//!
//! println!("The average B factor of the protein is: {}", avg_b_factor);
//! pdbtbx::save(&pdb, "dump/1ubq_no_hydrogens.pdb", pdbtbx::StrictnessLevel::Loose);
//! ```
//!
//! ## High level documentation
//! [general_docs]
//!
//! ## Parallelization
//! [Rayon](https://crates.io/crates/rayon) is used to create parallel iterators for all logical candidates. Use
//! the parallel version of an iterator by prefixing the name with `par_`. Among other the looping iterators,
//! like `atoms()`, `residues()` and `atoms_with_hierarchy()` are implemented as parallel iterators. The Rayon
//! implementations are gated behind the `rayon` [feature](https://doc.rust-lang.org/cargo/reference/features.html)
//! which is enabled by default.
//!
//! ## Serialization
//! Enable the `serde` feature for [Serde](https://crates.io/crates/serde) support.
//!
//! ## Spatial lookup of atoms
//! Enable the `rstar` feature for [rstar](https://crates.io/crates/rstar) support. This enables you to generate
//! R*trees making it possible to do very fast lookup for atoms with spatial queries. So for example finding close
//! atoms is very fast. See the documentation of this crate for more information on how to make use of all of its
//! features.
//!
#![cfg_attr(
    feature = "rstar",
    doc = r#"
```rust
use pdbtbx::*;
let (mut pdb, _errors) = pdbtbx::open("example-pdbs/1ubq.pdb").unwrap();
// You can loop over all atoms within 3.5 Aͦ of a specific atom
// Note: The `locate_within_distance` method takes a squared distance
let tree = pdb.create_atom_rtree();
for atom in tree.locate_within_distance(pdb.atom(42).unwrap().pos(), 3.5 * 3.5) {
    println!("{}", atom);
}

// You can even get information about the hierarchy of these atoms
// (the chain, residue and conformer that contain this atom)
let tree = pdb.create_hierarchy_rtree();
let mut total = 0;
for hierarchy in tree.locate_within_distance(pdb.atom(42).unwrap().pos(), 3.5 * 3.5) {
    if hierarchy.is_backbone() {
        total += 1;
    }
}
println!("There are {} backbone atoms within 3.5Aͦ of the atom at index 42", total);
# assert_eq!(total, 6);
```
"#
)]
#![doc = "## References"]
#![doc = "1. [`Perkel, J. M.`] Perkel, J. M. (2020). Why scientists are turning to Rust. Nature, 588(7836), 185–186. [https://doi.org/10.1038/d41586-020-03382-2](https://doi.org/10.1038/d41586-020-03382-2)"]
// Set linting behaviour
#![allow(clippy::upper_case_acronyms)] // Allow PDB (and derived) names to be used

/// Handle different levels of errors
mod errorlevel;
/// To open PDB files
mod read;
/// Reference tables for constants
mod reference_tables;
/// To save PDB files
mod save;
/// To determine the level of scrutiny that a step should display
mod strictness_level;
mod structs;
/// To handle transformations
mod transformation;
/// To validate certain invariants of PDB files
mod validate;

#[cfg(doc)]
pub mod general_docs;

pub use errorlevel::*;
pub use read::*;
pub use save::*;
pub use strictness_level::StrictnessLevel;
pub use structs::*;
pub use transformation::*;
pub use validate::{validate, validate_pdb};

/// Helper function to check extensions in filenames
fn check_extension(filename: impl AsRef<str>, extension: impl AsRef<str>) -> bool {
    filename
        .as_ref()
        .rsplit('.')
        .next()
        .map(|ext| ext.eq_ignore_ascii_case(extension.as_ref()))
        == Some(true)
}
