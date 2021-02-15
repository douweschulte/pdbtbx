![Compile & Test](https://github.com/nonnominandus/rust-pdb/workflows/Compile%20&%20Test/badge.svg)

## Description
This is a Rust library helping to parse, edit and save crystallographic PDB/PDBx files. It can read most atomic data from PDB/PDBx files, some is missing but will be added over time. Its high level goal is to create a stable, efficient and easy to use interface to PDB/PDBx files. 

## Contributing
As this is a library in active development, feel free to share your thoughts, ideas, hopes, and criticisms. Every comment will be read and discussed so that this library is as useful as possible for all users. Of course we all like a civilised discussion so please follow the community guidelines, but over all please be a civilised human being.

## License
MIT, just use it if you can use it. If you use it for something cool I would like to hear, but no obligations!

## PDB format
PDB 3.30 as published by wwPDB in 2008.

mmCIF, basic support to retrieve and save atomic data, will be extended.

## Why
Just for fun, to play with fancy abstractions. But at the same time I think that using Rust in scientific computing would be really cool and this library would be needed if I where to be doing my internship in Rust. So by creating it I hope to extend the usability of Rust a little bit more. Since Nature published an [article](https://www.nature.com/articles/d41586-020-03382-2) (technology feature) which laid out the benefits of using Rust and showed that Rust is used more and more, recently I am planning on working more with Rust in scientific projects. And I think that the best way to help Rust move forward (in the scientific community) is by creating more support for scientific projects in Rust.

Also because it is written in Rust it is much faster then anything written in Python even if there is a C/C++ backend. Based on some benchmarks (details are in benches/benchmark_results.csv) PDBTBX is as fast in opening and saving PDB files while being on average 240 times faster in 'editing' tasks (iterating over atoms/renumbering a pdb file/cloning a pdb file/transforming atoms/removing atoms) compared to CCTBX. As Rust is a compiled language build for speed this was to be expected but still the effect size is quite big. If anything taking 24 hours could be sped up 240 times it would only take 6 minutes.

## Contributors
* Douwe Schulte
* [Tianyi Shi](https://github.com/TianyiShi2001)

## Last update
### v0.4.0 'basic mmCIF support'
* Added mmCIF/PDBx open support
* Changed `open` and `save` to determine the filetype based on the extension
* Added `validate_pdb` to validate a PDB model before saving it in a PDB file
* Added support for bigger serial numbers and names to allow for bigger models to be saved in mmCIF files
* Fixed some issues with mmCIF output

Also see [changelog](https://github.com/nonnominandus/pdbtbx/blob/master/changelog.md).