//! Test to detect issue raised in #92 by Brian.
//! Description:
//! When trying to parse a nucleic structure in PDB format, it seems than the number of atoms found is
//! bigger than the number of the actual atoms in the PDB file. It seems from a few tests that hydrogen
//! atoms are added multiple times to the internal PDB object.
//! Problem:
//! In the parser the name of the conformer was not trimmed correctly creating a ton of different
//! conformers creating this issue.

use pdbtbx::*;

#[test]
fn main() {
    assert_eq!(22, number_of_h("example-pdbs/nucleic.pdb"));
}

fn number_of_h(file: &str) -> usize {
    let (structure, _errors) = open(file).unwrap();

    println!("{}", structure.atom_count());

    for res in structure.residues() {
        println!("{res}");
        for con in res.conformers() {
            println!("  {con}");
            for a in con.atoms() {
                println!("    {a}");
            }
        }
    }

    structure.atoms().fold(0, |acc, a| {
        acc + usize::from(a.element() == Some(&Element::H))
    })
}
