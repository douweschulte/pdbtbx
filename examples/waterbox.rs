//! Example for how to create a box of water surrounding the protein

use std::env;
use std::path::Path;
use std::time::Instant;

use pdbtbx::*;

fn main() {
    let filename = env::current_dir()
        .unwrap()
        .as_path()
        .join(Path::new("dump"))
        .join(Path::new("waterbox.pdb"))
        .into_os_string()
        .into_string()
        .unwrap();
    save_pdb(
        &create_waterbox((25.0, 25.0, 25.0)),
        filename,
        StrictnessLevel::Loose,
    )
    .expect("Save not successful");
}

fn create_waterbox(size: (f64, f64, f64)) -> PDB {
    let now = Instant::now();

    let (mut liquid, _errors) = ReadOptions::new()
        .set_level(StrictnessLevel::Loose)
        .set_format(Format::Pdb)
        .read("example-pdbs/liquid.pdb")
        .unwrap();

    let time = now.elapsed();

    liquid.remove_atoms_by(|a| a.name() != "O");
    liquid.atoms_mut().for_each(|a| {
        a.set_b_factor(50.0).unwrap();
        a.set_element(Element::O);
    });

    println!("Time to parse liquid.pdb {}ms", time.as_millis());
    println!("The PDB: {liquid}");

    let cell = liquid.unit_cell.as_ref().unwrap().size();
    let fa = (size.0 / cell.0).ceil() as usize;
    let fb = (size.1 / cell.1).ceil() as usize;
    let fc = (size.2 / cell.2).ceil() as usize;

    for a in 0..fa {
        for b in 0..fb {
            for c in 0..fc {
                let mut extra = liquid.model(0).unwrap().clone();
                extra.apply_transformation(&TransformationMatrix::translation(
                    a as f64 * cell.0,
                    b as f64 * cell.1,
                    c as f64 * cell.2,
                ));
                liquid.model_mut(0).unwrap().join(extra);
            }
        }
    }

    liquid.remove_atoms_by(|atom| {
        atom.x() < 0.0
            || atom.x() > size.0
            || atom.y() < 0.0
            || atom.y() > size.1
            || atom.z() < 0.0
            || atom.z() > size.2
    });

    liquid.remove_empty();
    liquid.renumber();

    liquid
}
