use pdbtbx::*;
use rayon::iter::ParallelIterator;

fn main() {
    atom_sphere();
    residue_sphere();
}

/// Find all Atoms in a sphere around a single origin Atom with a user-defined radius
/// This is using the features `rstar` and `rayon`.
fn atom_sphere() {
    let (pdb, _errors) = open_pdb("example-pdbs/1ubq.pdb", StrictnessLevel::Loose).unwrap();
    let (origin_id, radius): (usize, f64) = (12, 3.5);

    // Leverage parallel searching
    let origin_atom = pdb
        .par_atoms()
        .find_first(|atom| atom.serial_number() == origin_id)
        .unwrap();
    let tree = pdb.create_atom_rtree();
    let mut sphere_atoms: Vec<&&Atom> = tree
        .locate_within_distance(origin_atom.pos(), radius.powf(2.0))
        .collect();

    // Since the rtree is not ordered, the resulting Vec won't be either.
    sphere_atoms.sort_unstable();
    assert_eq!(sphere_atoms.len(), 16)
}

/// Find all Atoms belonging to a Residue that has at least one Atom within a sphere of
/// user-defined origin and radius.
/// This is using the features `rstar` and `rayon`.
fn residue_sphere() {
    let (pdb, _errors) = open_pdb("example-pdbs/1ubq.pdb", StrictnessLevel::Loose).unwrap();
    let (origin_id, radius): (usize, f64) = (12, 3.5);

    let sphere_origin = pdb
        .atoms_with_hierarchy()
        .find(|a| a.atom().serial_number() == origin_id)
        .unwrap();
    // Create a tree of atoms containing their respective hierarchies.
    let tree = pdb.create_hierarchy_rtree();
    let mut sphere_atoms: Vec<&Atom> = tree
        // This finds all Atoms with their hierarchy within the given sphere.
        .locate_within_distance(sphere_origin.atom().pos(), radius.powf(2.0))
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
    assert_eq!(sphere_atoms.len(), 37)
}
