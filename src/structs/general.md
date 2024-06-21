# PDB Hierarchy
As explained in depth in the [documentation of CCTBX](https://cci.lbl.gov/cctbx_docs/iotbx/iotbx.pdb.html#iotbx-pdb-hierarchy)
it can be quite hard to properly define a hierarchy for PDB files which works for all files.
This library follows the hierarchy presented by CCTBX [`Grosse-Kunstleve, R. W. et al`], but renames the `residue_group` and
`atom_group` constructs. This gives the following hierarchy, with the main identifying characteristics annotated per level.
* [PDB]
    * [Model] \
      Serial number
        * [Chain] \
          Id
            * [Residue] (analogous to `residue_group` in CCTBX) \
              Serial number \
              Insertion code
                * [Conformer] (analogous to `atom_group` in CCTBX) \
                  Name \
                  Alternative location
                    * [Atom] \
                      Serial number \
                      Name

# Iterating over the PDB Hierarchy
```rust
use pdbtbx::*;
let (mut pdb, _errors) = pdbtbx::open("example-pdbs/1ubq.pdb").unwrap();
// Iterating over all levels
for model in pdb.models() {
    for chain in model.chains() {
        for residue in chain.residues() {
            for conformer in residue.conformers() {
                for atom in conformer.atoms() {
                    // Do the calculations
                }
            }
        }
    }
}
// Or only over a couple of levels (just like in the example above)
for residue in pdb.residues() {
    for atom in residue.atoms() {
        // Do the calculations
    }
}
// Or with access to the information with a single line
for hierarchy in pdb.atoms_with_hierarchy() {
    println!("Atom {} in Conformer {} in Residue {} in Chain {} in Model {}",
        hierarchy.atom().serial_number(),
        hierarchy.conformer().name(),
        hierarchy.residue().serial_number(),
        hierarchy.chain().id(),
        hierarchy.model().serial_number()
    );
}
// Or with mutable access to the members of the hierarchy
for mut hierarchy in pdb.atoms_with_hierarchy_mut() {
    let new_x = hierarchy.atom().x() * 1.5;
    hierarchy.atom_mut().set_x(new_x);
}
```

# References
1. [`Grosse-Kunstleve, R. W. et al`] Grosse-Kunstleve, R. W., Sauter, N. K., Moriarty, N. W., & Adams, P. D. (2002). TheComputational Crystallography Toolbox: crystallographic algorithms in a reusable software framework. Journal of Applied Crystallography, 35(1), 126–136. [https://doi.org/10.1107/s0021889801017824](https://doi.org/10.1107/s0021889801017824)
