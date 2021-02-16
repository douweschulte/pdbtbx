# Changelog
All versions are properly annotated on [github](https://github.com/nonnominandus/pdbtbx/releases) so there the source code for each version can be retrieved.

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