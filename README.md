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
### v0.1.4
* Fixed a bug in which strings that are too short cause setter functions of various character based properties of atoms/residues/chains to panic

### v0.1.3
* Fixed a mistake witch prevented valid characters from being used to set various character based properties of atoms/residues/chains

### v0.1.2
* Added `.join()` on PDB. 
* Added atomic data lookup (number & radius) on Atoms
* Added `.overlaps()` function to Atom, which uses the atomic radius to determine if two atoms overlap
* Added support for the `MASTER` PDB Record both while reading and saving
* Fixed the behaviour of `.join()` on Model/Chain/Residue

### v0.1.1
Textual changes to documentation