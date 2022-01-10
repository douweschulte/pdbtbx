use pdbtbx::*;
use std::env;
use std::path::Path;
use std::time::Instant;

fn main() {
    let pdb = open_pdb("example-pdbs/1ubq.pdb", StrictnessLevel::Loose)
        .unwrap()
        .0;
    //ATOM    750  C   GLY A  47      20.027  28.708  23.336  1.00 16.31           C
    let sel1 = pdb.find(
        FindModel::NoInfo,
        FindChain::ID("A".to_owned()),
        FindResidue::NoInfo,
        FindConformer::Name("GLY".to_owned()),
        FindAtom::SerialNumber(750),
    );
    let sel2 = pdb.find(
        FindModel::NoInfo,
        FindChain::NoInfo,
        FindResidue::SerialNumber(47),
        FindConformer::NoInfo,
        FindAtom::Name("C".to_owned()),
    );
    assert_eq!(sel1, sel2);
    //ATOM   1111 HD13 LEU A  69      32.170  32.079  18.138  1.00 10.72           H
    let sel1 = pdb.find(
        FindModel::NoInfo,
        FindChain::ID("A".to_owned()),
        FindResidue::NoInfo,
        FindConformer::Name("LEU".to_owned()),
        FindAtom::SerialNumber(1111),
    );
    let sel2 = pdb.find(
        FindModel::NoInfo,
        FindChain::NoInfo,
        FindResidue::SerialNumber(69),
        FindConformer::NoInfo,
        FindAtom::Name("HD13".to_owned()),
    );
    assert_eq!(sel1, sel2);
}
