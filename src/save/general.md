# Saving

Once you have your [`PDB`] struct you can save it in a couple of ways. The output can be made in two different file formats: PDB and mmCIF. The saving functions represent this choice: [`save_pdb()`] and [`save_mmcif()`] are clear in the ouput format while [`save()`] chooses the format based on the path given if the extension is `pdb` it will generate a PDB file, if the extension is `cif` it will generate a mmCIF file.

The other extra option is choosing the `*_raw` functions. These do not validate the [`PDB`] structs before saving and output directly to a [`std::io::BufWriter`]. The validation uses the [`validate_pdb()`] or [`validate()`] functions internally.

## All functions
| Format |  Normal | Without validation |
| --- | --- | --- |
| Based on input | [`save()`] | ... |
| PDB | [`save_pdb()`] | [`save_pdb_raw()`] |
| mmCIF | [`save_mmcif()`] | [`save_mmcif_raw()`] | 