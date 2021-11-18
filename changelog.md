# Changelog
All versions are properly annotated on [github](https://github.com/nonnominandus/pdbtbx/releases) so there the source code for each version can be retrieved.

### upcoming v0.8.0
* Added support for residue serial numbers over 9999 and atom serial numbers over 99999 for PDB files.
* Changed argument type of `save_pdb` from `PDB` to `&PDB`
* Allow lack of chain name in PDB files
* Added mutable structs to extend the use of `AtomWithHierarchy` alongside a refactor which created a struct for every hierarchy level. See the docs for more information.
* Removed `Atom.pos_array()` and moved the `rstar::rtree` to use `(f64, f64, f64)` instead of `[f64; 3]`. This was made possible by the adoption of tuples as points in rstar. 

### v0.7.0 'Ecosystem integration'
* Added parallel iterators based on [Rayon](https://crates.io/crates/rayon) (Thanks to DocKDE)
* Added support for generating r*trees from [Rstar](https://crates.io/crates/rstar), this has to be opted in by using the feature `rstar`
* Added support for serialization using [Serde](https://crates.io/crates/serde), this has to be opted in using the feature `serde`
* Added a new struct `AtomWithHierarchy` to have access to the containing layers of an atom in an easy way and added functions to generate and work with this struct
* Added `binary_find_atom` to all hierarchies to find atoms in less time
* Added more names for amino acids and backbone atoms (Thanks to DocKDE)
* Added support for bonds (can only read Disulfide bonds from PDBs for now)
* And many more small fixes and docs updates

### v0.6.3
* Added Anisotropic temperature factor support for mmCIF files
* Fixed an issue in the aniso matrix
* Added a `full_sort` function on PDB
* Fixed small bugs in the PDB saving logic

### v0.6.2
* Fixed a bug in PDB b factor and occupancy validation which showed an error when the value was 0.00
* Fixed a bug in `atom.atomic_radius()`, it used to give the radius of the previous atom in the periodic table
* Added more atomic radii (vanderwaals and covalent bonds)

### v0.6.1 
* Fixed a bug arbitrarily constraining the maximum value of atom serial numbers in PDB files

### v0.6.0 'Hetero atoms remastered'
* Reworked the library to handle Hetero atoms as normal atoms (with the `atom.hetero()` function returning `true`) instead of saving them in `model.hetero_chains()`
* Implemented the standard traits (Clone/PartialEq/Eq/PartialOrd/Ord) for most structures
* Fixed conformers in `pdb.renumber()` they were disregarded before
* The symmetry structure now also accepts and provides Hall symbols
* Fixed multiple bugs
* Added many more unit tests and started tracking test coverage

### v0.5.1
* Fixed bugs in `.remove_empty` to work better with hetero chains
* Added support for negative residue sequence numbers
* Standardised the precision of floating points in the mmCIF output, at least 1 and at most 5 decimals will be shown
* Fixed an issue with the occupancy of atoms shared between multiple conformers, it will now add up to the original value 

### v0.5.0 'Alternative location support'
* Added `Conformer` which sits between `Residue` and `Atom` and is analogous to `atom_group` in cctbx
* Added editing functions for Conformers
* Added `HEADER` identifier support for parsing PDB and saving PDB and mmCIF
* Reverted mmCIF output atom_site column ordering to v0.3.3, the newly introduced ordering gave issues with Phenix
* Added `remove_empty` functions on all structs, to remove all empty layers after large scale deletions

### v0.4.1
* All string based properties for atom/residue/chain are trimmed and converted to uppercase before being set
* A `.extend` function is provided for residue/chain/model/pdb to easily add an iterator to the list of children 

### v0.4.0 'basic mmCIF support'
* Added mmCIF/PDBx open support
* Changed `open` and `save` to determine the filetype based on the extension
* Added `validate_pdb` to validate a PDB model before saving it in a PDB file
* Added support for bigger serial numbers and names to allow for bigger models to be saved in mmCIF files
* Fixed some issues with mmCIF output

### v0.3.3
* Added very basic exporting to mmCIF, it will only export the unit cell, symmetry and atomic data. 

### v0.3.2
* Added saving of `DBREF`/`SEQADV`/`SEQRES`
* Generates default matrices for `SCALE` and `ORIGX` if not available and the strictness level on save is `Strict`
* Added constructor `scale` to `TransformationMatrix` to have a magnifying matrix with 3 different factors

### v0.3.1
* Added `distance_wrapping` and `overlaps_wrapping` functions to Atom which wrap around the unit cell to find the shortest distance

### v0.3.0 'Primary Sequence Support'
* Added support for parsing and validating `DBREF`/`SEQADV`/`SEQRES`/`MODRES`
* Added saving of `MODRES` records, the other primary structure sections will follow soon
* Added differential saving, which changes the output based on the `StrictnessLevel` provided
* Redefined `overlaps` function on Atoms, the calculation was faulty before
* Added `distance` function between two Atoms
* Removed renumber on save

### v0.2.1
* Exported `save_raw` was created in v0.2.0 but not accessible
* Added `ENDMDL` records after model definitions while saving making saved ensemble files valid in other software
* Extended warnings for validation of ensemble files, it will now generate a `LooseWarning` if `HETATM`s do not correspond
* Changed the implementation of the `.remove_*_by` functions to be 75% faster

### v0.2.0
* Made `add_child` methods for model/chain/residue public.
* Extended saving it now validates and renumbers the given PDB. It fails upon generation of validation errors, while the user can specify the error levels to allow
* Added save_raw to save to a BufWriter. This function is called `save_raw`.
* Extended parser error generation and handling. It now fails upon generation of errors, while the user can specify the error levels to allow
* Added parser from BufReader. This function is called `parse` and the function previously called `parse` is renamed to `open`.
* Rewrote `pdb.total_*_count()` as the previous version was inaccurate
* Saved 21.4 MB in the published crate by ignoring certain files (thanks [Byron](https://github.com/Byron)!)

### v0.1.5
* Finally fixed the full bug encountered in v0.1.3

### v0.1.4 (yanked)
* Fixed a bug in which strings that are too short cause setter functions of various character based properties of atoms/residues/chains to panic

### v0.1.3 (yanked)
* Fixed a mistake witch prevented valid characters from being used to set various character based properties of atoms/residues/chains

### v0.1.2
* Added `.join()` on PDB. 
* Added atomic data lookup (number & radius) on Atoms
* Added `.overlaps()` function to Atom, which uses the atomic radius to determine if two atoms overlap
* Added support for the `MASTER` PDB Record both while reading and saving
* Fixed the behaviour of `.join()` on Model/Chain/Residue

### v0.1.1
Textual changes to documentation

### v0.1.0
Initial release
* Basic PDB parsing, editing and saving functionality
