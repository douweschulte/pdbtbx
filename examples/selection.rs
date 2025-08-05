//! Examples of how to select things from PDB files

use pdbtbx::*;

fn main() {
    let (pdb, _errors) = ReadOptions::default()
        .set_level(StrictnessLevel::Loose)
        .set_format(Format::Pdb)
        .read("example-pdbs/1ubq.pdb")
        .unwrap();

    // Two ways of selecting the following atom in the PDB file, the first search can be somewhat faster
    // because it can discard other chains which the second search has to test.
    // ```
    // ATOM    750  C   GLY A  47      20.027  28.708  23.336  1.00 16.31           C
    // ```
    let sel1 = pdb
        .find(
            Term::ChainId("A".to_owned())
                & Term::ConformerName("GLY".to_owned())
                & Term::AtomSerialNumber(750),
        )
        .collect::<Vec<_>>();
    let sel2 = pdb
        .find(Term::ResidueSerialNumber(47) & Term::AtomName("C".to_owned()))
        .collect::<Vec<_>>();
    // But both give the same result
    assert_eq!(sel1, sel2);

    // Two ways of selecting the following atom in the PDB file, the first search can be somewhat faster
    // because it can discard other chains which the second search has to test.
    // ```
    // ATOM   1111 HD13 LEU A  69      32.170  32.079  18.138  1.00 10.72           H
    // ```
    let sel1 = pdb
        .find(
            Term::ChainId("A".to_owned())
                & Term::ConformerName("LEU".to_owned())
                & Term::AtomSerialNumber(1111),
        )
        .collect::<Vec<_>>();
    let sel2 = pdb
        .find(Term::ResidueSerialNumber(69) & Term::AtomName("HD13".to_owned()))
        .collect::<Vec<_>>();
    // But both give the same result
    assert_eq!(sel1, sel2);

    // Searching too broadly returns an iterator over all hierarchies like [PDB::atoms_with_hierarchy].
    assert_eq!(
        pdb.find(Search::Single(Term::ModelSerialNumber(0))).count(),
        pdb.atom_count()
    );

    // You can use and `&` to combine a search, this short circuits if possible.
    let search = Term::Element(Element::C) & Term::ConformerName("VAL".to_owned());
    assert!(pdb
        .find(search.clone())
        .all(|s| (s.atom().element() == Some(&Element::C)) & (s.conformer().name() == "VAL")));
    assert_eq!(
        pdb.find(search.clone()).count(),
        pdb.atoms_with_hierarchy()
            .filter(|s| (s.atom().element() == Some(&Element::C)) & (s.conformer().name() == "VAL"))
            .count()
    );
    let val_and_c = pdb
        .conformers()
        .filter(|c| c.name() == "VAL")
        .flat_map(|c| c.atoms().filter(|a| a.element() == Some(&Element::C)))
        .count();
    assert_eq!(pdb.find(search).count(), val_and_c);

    // You can use and `|` to combine a search, this short circuits if possible.
    let search = Term::Element(Element::C) | Term::ConformerName("VAL".to_owned());
    assert!(pdb
        .find(search.clone())
        .all(|s| (s.atom().element() == Some(&Element::C)) | (s.conformer().name() == "VAL")));
    assert_eq!(
        pdb.find(search.clone()).count(),
        pdb.atoms_with_hierarchy()
            .filter(|s| (s.atom().element() == Some(&Element::C)) | (s.conformer().name() == "VAL"))
            .count()
    );
    let val = pdb
        .conformers()
        .filter(|c| c.name() == "VAL")
        .flat_map(Conformer::atoms)
        .count();
    let c_not_val = pdb
        .conformers()
        .filter(|c| c.name() != "VAL")
        .flat_map(|c| c.atoms().filter(|a| a.element() == Some(&Element::C)))
        .count();
    assert_eq!(pdb.find(search).count(), val + c_not_val);

    // You can use and `^` to combine a search, this cannot short circuit, but that has never been a rule (Bryan Cantrill).
    let search = Term::Element(Element::C) ^ Term::ConformerName("VAL".to_owned());
    assert!(pdb
        .find(search.clone())
        .all(|s| (s.atom().element() == Some(&Element::C)) ^ (s.conformer().name() == "VAL")));
    assert_eq!(
        pdb.find(search.clone()).count(),
        pdb.atoms_with_hierarchy()
            .filter(|s| (s.atom().element() == Some(&Element::C)) ^ (s.conformer().name() == "VAL"))
            .count()
    );
    let val_not_c = pdb
        .conformers()
        .filter(|c| c.name() == "VAL")
        .flat_map(|c| c.atoms().filter(|a| a.element() != Some(&Element::C)))
        .count();
    let c_not_val = pdb
        .conformers()
        .filter(|c| c.name() != "VAL")
        .flat_map(|c| c.atoms().filter(|a| a.element() == Some(&Element::C)))
        .count();
    assert_eq!(pdb.find(search).count(), val_not_c + c_not_val);
}
