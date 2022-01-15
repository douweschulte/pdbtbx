![Compile & Test](https://github.com/douweschulte/pdbtbx/actions/workflows/rust.yml/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/pdbtbx.svg)](https://crates.io/crates/pdbtbx)
[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.4671031.svg)](https://doi.org/10.5281/zenodo.4671031)

## Description
This is a Rust library helping to parse, edit and save crystallographic PDB/mmCIF files. It can read most atomic data from PDB/mmCIF files. Its high level goal is to create a stable, efficient and easy to use interface to PDB/mmCIF files written in pure Rust.

## Contributing
As this is a library in active development, feel free to share your thoughts, ideas, hopes, and criticisms. Every comment will be read and discussed so that this library is as useful as possible for all users. Of course we all like a civilised discussion so please follow the community guidelines, but over all please be a civilised human being.

## License
MIT, just use it if you can use it. If you use it for something cool I would like to hear, but no obligations!

## Why
It started as a way to use Rust in a scientific project. But it moved to an open source project because I think that using Rust in scientific computing is be really helpful and a great addition alongside the ubiquitous Python. So by creating it I hope to extend the usability of Rust a little bit more. Since Nature published an [article](https://www.nature.com/articles/d41586-020-03382-2) (technology feature) which laid out the benefits of using Rust and showed that Rust is used more and more, I am planning on working more with Rust in scientific projects. And I think that the best way to help Rust move forward (in the scientific community) is by creating more support for scientific projects in Rust.

## Contributors
* Douwe Schulte
* [Tianyi Shi](https://github.com/TianyiShi2001)
* [DocKDE](https://github.com/DocKDE)

## Supported features
As the main goal of this library is to allow access to the atomical data many metadata features of both PDB and mmCIF are unsupported. For both file formats the recent versions (PDB v3.30 and mmcif v5.338) are used, but as both are quite stable file formats the exact version should not matter to end users.

![supported features table](pictures/supported_features.png)

The features where support is planned are planned to be included in the 1.0 release. The features where support is envisioned are candidates to be included, but not necessarily in the 1.0 release. The features which are planned are thought to be unnecessary for atomic data computations on theses files. If any of these are really needed for your use case please raise an issue then we can discuss its inclusion. Also if you need a feature that is 'planned' or 'envisioned' feel free to raise an issue to guide development to spots where it can make a real life impact.

## Additional crate features
* The crate leverages the power of the crate Rayon to provide multithreaded iterators over PDB structures.
* The crate leverages the power of the crate rstar to provide very efficient spatial lookup.
* The crate has a performant way of selecting atom(s), see `Search`.
* The crate has many nice helper methods for common PDB operations (renumbering, sorting, atomic properties lookup).
* The crate has many ways of iterating over the PDB structure to allow for convenient access and control over the performance.

## Latest update
### v0.8.0 'All the hierarchy'
* Added support for residue serial numbers over 9999 and atom serial numbers over 99999 for PDB files. (Thanks to DocKDE)
* Changed argument type of `save_pdb` from `PDB` to `&PDB`. (Thanks to DocKDE)
* Allow lack of chain name in PDB files. (Thanks to DocKDE)
* Added mutable structs to extend the use of `AtomWithHierarchy` alongside a refactor which created a struct for every hierarchy level. See the docs for more information.
* Removed `Atom.pos_array()` and moved the `rstar::rtree` to use `(f64, f64, f64)` instead of `[f64; 3]`. This was made possible by the adoption of tuples as points in rstar. 

Also see [changelog](https://github.com/douweschulte/pdbtbx/blob/master/changelog.md).

## Support and development
When I am actively using this crate in my own projects this crate is actively worked on and extended. I am more than happy to receive and work PRs and Issues even if the project seems a bit stale. But if anyone finds this project stale and wants to take over moderation and/or main development feel free to reach out and we can discuss. I would be happy to transfer to project and access to crates.io if that means the project will live on.
