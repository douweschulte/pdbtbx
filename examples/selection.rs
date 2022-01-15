use pdbtbx::*;

fn main() {
    let pdb = open_pdb("example-pdbs/1ubq.pdb", StrictnessLevel::Loose)
        .unwrap()
        .0;
    //ATOM    750  C   GLY A  47      20.027  28.708  23.336  1.00 16.31           C
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
    assert_eq!(sel1, sel2);
    //ATOM   1111 HD13 LEU A  69      32.170  32.079  18.138  1.00 10.72           H
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
    assert_eq!(sel1, sel2);

    assert_eq!(
        pdb.find(Search::Single(Term::ModelSerialNumber(0))).count(),
        pdb.atom_count()
    );

    let search = Term::AtomElement("C".to_owned()) & Term::ConformerName("VAL".to_owned());
    assert!(pdb
        .find(search.clone())
        .all(|s| (s.atom().element() == "C") & (s.conformer().name() == "VAL")));
    assert_eq!(
        pdb.find(search.clone()).count(),
        pdb.atoms_with_hierarchy()
            .filter(|s| (s.atom().element() == "C") & (s.conformer().name() == "VAL"))
            .count()
    );
    let val_and_c = pdb
        .conformers()
        .filter(|c| c.name() == "VAL")
        .map(|c| c.atoms().filter(|a| a.element() == "C"))
        .flatten()
        .count();
    assert_eq!(pdb.find(search).count(), val_and_c);

    let search = Term::AtomElement("C".to_owned()) | Term::ConformerName("VAL".to_owned());
    assert!(pdb
        .find(search.clone())
        .all(|s| (s.atom().element() == "C") | (s.conformer().name() == "VAL")));
    assert_eq!(
        pdb.find(search.clone()).count(),
        pdb.atoms_with_hierarchy()
            .filter(|s| (s.atom().element() == "C") | (s.conformer().name() == "VAL"))
            .count()
    );
    let val = pdb
        .conformers()
        .filter(|c| c.name() == "VAL")
        .map(|c| c.atoms())
        .flatten()
        .count();
    let c_not_val = pdb
        .conformers()
        .filter(|c| c.name() != "VAL")
        .map(|c| c.atoms().filter(|a| a.element() == "C"))
        .flatten()
        .count();
    assert_eq!(pdb.find(search).count(), val + c_not_val);

    let search = Term::AtomElement("C".to_owned()) ^ Term::ConformerName("VAL".to_owned());
    assert!(pdb
        .find(search.clone())
        .all(|s| (s.atom().element() == "C") ^ (s.conformer().name() == "VAL")));
    assert_eq!(
        pdb.find(search.clone()).count(),
        pdb.atoms_with_hierarchy()
            .filter(|s| (s.atom().element() == "C") ^ (s.conformer().name() == "VAL"))
            .count()
    );
    let val_not_c = pdb
        .conformers()
        .filter(|c| c.name() == "VAL")
        .map(|c| c.atoms().filter(|a| a.element() != "C"))
        .flatten()
        .count();
    let c_not_val = pdb
        .conformers()
        .filter(|c| c.name() != "VAL")
        .map(|c| c.atoms().filter(|a| a.element() == "C"))
        .flatten()
        .count();
    assert_eq!(pdb.find(search).count(), val_not_c + c_not_val);
}
