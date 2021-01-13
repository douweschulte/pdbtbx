![Compile & Test](https://github.com/nonnominandus/rust-pdb/workflows/Compile%20&%20Test/badge.svg)

## Description
This is a Rust library helping to parse, edit and save crystallographic PDB files. It can read most crystallographic PDB lines, some are missing which we are hopeful will be added over time.

## Contributing
As this is a library in active development feel free to share your thoughts, ideas, hopes, and criticisms. Every comment will be read and discussed so that the final result of this library is as useful as possible for all users. Of course we all like a civilised discussion so please follow the community guidelines, but over all please be a civilised human being.

## License
MIT, just use it if you can use it, if you use it for something cool I would like to hear, but no obligations!

## PDB format
PDB 3.30 as published by wwPDB in 2008.

No mmCIF support, but that would be cool to include in the future.

## Why
Just for fun, to play with fancy abstractions. But at the same time I think that using Rust in scientific computing would be really cool and this library would be needed if I where to be doing my internship in Rust. So by creating it I hope to extend the usability of Rust a little bit more. Since Nature published an article (https://www.nature.com/articles/d41586-020-03382-2) (technology feature) which laid out the benefits of using Rust and showed that Rust is used more and more, recently I am planning on working more with Rust in scientific projects. And I think that the best way to help Rust move forward (in the scientific community) is by creating more support for scientific projects in Rust.

## Contributors
* Douwe Schulte
* [Tianyi Shi](https://github.com/TianyiShi2001)

## Changelog
### v0.2.1
_API changes_
* Exported `save_raw` was created in v0.2.0 but not accessible
* Added `ENDMDL` records after model definitions while saving making saved ensemble files valid in other software
* Extended warnings for validation of ensemble files, it will now generate a `LooseWarning` if `HETATM`s do not correspond
* Changed the implementation of the `.remove_*_by` functions to be 75% faster
### v0.2.0
_API changes + additions_
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
_API additions_
* Added `.join()` on PDB. 
* Added atomic data lookup (number & radius) on Atoms
* Added `.overlaps()` function to Atom, which uses the atomic radius to determine if two atoms overlap
* Added support for the `MASTER` PDB Record both while reading and saving
* Fixed the behaviour of `.join()` on Model/Chain/Residue

### v0.1.1
Textual changes to documentation