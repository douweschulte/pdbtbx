//! Example on using rtree to do spatial lookups

use rayon::iter::ParallelIterator;

use pdbtbx::*;

fn main() {
    let (pdb, _errors) = ReadOptions::new()
        .set_level(StrictnessLevel::Loose)
        .set_format(Format::Pdb)
        .read("example-pdbs/1ubq.pdb")
        .unwrap();

    atom_sphere(&pdb);
    residue_sphere(&pdb);
    find_clashes(&pdb);
}

/// Find all Atoms in a sphere around a single origin Atom with a user-defined radius
/// This is using the features `rstar` and `rayon`.
fn atom_sphere(pdb: &PDB) {
    let (origin_id, radius): (usize, f64) = (12, 3.5);

    // Leverage parallel searching
    let origin_atom = pdb
        .par_atoms()
        .find_first(|atom| atom.serial_number() == origin_id)
        .unwrap();
    let tree = pdb.create_atom_rtree();
    let mut sphere_atoms: Vec<&&Atom> = tree
        .locate_within_distance(origin_atom.pos(), radius.powi(2))
        .collect();

    // Since the rtree is not ordered, the resulting Vec won't be either.
    sphere_atoms.sort_unstable();
    assert_eq!(sphere_atoms.len(), 16);
}

/// Find all Atoms belonging to a Residue that has at least one Atom within a sphere of
/// user-defined origin and radius.
/// This is using the features `rstar` and `rayon`.
fn residue_sphere(pdb: &PDB) {
    let (origin_id, radius): (usize, f64) = (12, 3.5);

    let sphere_origin = pdb
        .atoms_with_hierarchy()
        .find(|a| a.atom().serial_number() == origin_id)
        .unwrap();
    // Create a tree of atoms containing their respective hierarchies.
    let tree = pdb.create_hierarchy_rtree();
    let mut sphere_atoms: Vec<&Atom> = tree
        // This finds all Atoms with their hierarchy within the given sphere.
        .locate_within_distance(sphere_origin.atom().pos(), radius.powi(2))
        // Find the Residues each found Atom belongs to and return all the Atoms they contain.
        .flat_map(|atom_hier| atom_hier.residue().atoms())
        // Collect the flattened iterator into a Vec
        .collect();

    // The resulting Vec contains duplicates because each hierarchical Atom found was queried for
    // all Atoms within the same Residue so sorting and deduplicating is necessary.
    // Note: this can be done a bit more elegantly with the `Itertools` crate:
    // ```
    // use itertools::Itertools;
    //
    // let mut sphere_atoms: Vec<&Atom> = tree
    //     .locate_within_distance(sphere_origin.atom().pos(), radius.powf(2.0))
    //     .flat_map(|atom_hier| atom_hier.residue().atoms())
    //     .unique()
    //     .collect();
    // ```
    sphere_atoms.sort_unstable();
    sphere_atoms.dedup();
    assert_eq!(sphere_atoms.len(), 37);
}

/// Determine whether any Atoms have other Atoms within they atomic radius and collect
/// the results in a Vector holding a tuple of the Atoms (plus hierarchy) in question.
/// This can be used to find any clashes or close contacts.
/// Results for Atoms within the same Residue are excluded as well as those from the C and N Atoms
/// constituting the peptide bond of neighbouring amino acids.
/// Also, Atoms are not counted twice.
fn find_clashes(pdb: &PDB) {
    let tree = pdb.create_hierarchy_rtree();

    let mut clashing_atoms = Vec::new();
    for atom_hier in pdb.atoms_with_hierarchy() {
        let radius = atom_hier
            .atom()
            .element()
            .unwrap()
            .atomic_radius()
            .unbound
            .unwrap()
            .powi(2);
        let contacts = tree.locate_within_distance(atom_hier.atom().pos(), radius);

        for other_atom_hier in contacts {
            // This eliminates duplicate entries
            if other_atom_hier.atom() < atom_hier.atom()
            // This eliminates atoms from same residue
                && other_atom_hier.residue() != atom_hier.residue()
                // This eliminates peptide bonds
                && !(other_atom_hier.atom().name() == "C"
                    && atom_hier.atom().name() == "N"
                    && other_atom_hier.residue().serial_number() + 1
                        == atom_hier.residue().serial_number())
            {
                clashing_atoms.push((atom_hier.clone(), other_atom_hier.clone()));
            }
        }
    }
    assert_eq!(clashing_atoms.len(), 1);
    assert_eq!(clashing_atoms[0].0.residue().name(), Some("HOH"));
    assert_eq!(clashing_atoms[0].1.residue().name(), Some("LYS"));
    assert_eq!(clashing_atoms[0].0.atom().name(), "O");
    assert_eq!(clashing_atoms[0].1.atom().name(), "HZ3");
}
