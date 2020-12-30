mod parser;
mod reference_tables;
mod save;
mod structs;
mod transformation;
mod validate;

use std::env;
use std::time::Instant;
use transformation::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let now = Instant::now();

    let mut pdb = parser::parse(&args[1]).unwrap();

    let time = now.elapsed();

    println!(
        "Found {} atoms, in {} residues, in {} chains, in {} models it all took {} ms",
        pdb.total_amount_atoms(),
        pdb.total_amount_residues(),
        pdb.total_amount_chains(),
        pdb.amount_models(),
        time.as_millis()
    );

    println!("PDB parsed");

    let mut avg = 0.0;
    let mut total_back = 0;
    let mut avg_back = 0.0;
    let mut total_side = 0;
    let mut avg_side = 0.0;

    println!("Set values");

    for atom in pdb.atoms() {
        avg += atom.b_factor();
        if let Some(true) = atom.backbone() {
            total_back += 1;
            avg_back += atom.b_factor();
        } else {
            total_side += 1;
            avg_side += atom.b_factor();
        }
    }

    println!("Counted for averages");

    avg = avg / ((total_back + total_side) as f64);
    avg_back = avg_back / (total_back as f64);
    avg_side = avg_side / (total_side as f64);

    println!("Found averages");

    println!(
        "Average B factor: Total: {:.3}, Backbone: {:.3}, Sidechains: {:.3}",
        avg, avg_back, avg_side
    );
    println!("Scale: {:?}", pdb.scale().transformation());

    let index = reference_tables::get_index_for_symbol("P 21 21 21").unwrap();

    println!(
        "{:?}, {:?}, {}",
        reference_tables::get_transformation(index),
        reference_tables::get_symbol_for_index(index),
        index
    );

    pdb.renumber();

    let water = create_waterbox(pdb.unit_cell().size());

    save::save(&water, "example-pdbs/waterbox.pdb").expect("Save not successful");
    save::save(&pdb, &format!("{}_saved", args[1])).expect("Save not successful");
}

fn create_waterbox(size: (f64, f64, f64)) -> structs::PDB {
    let now = Instant::now();

    let mut liquid = parser::parse("example-pdbs/liquid.pdb").unwrap();

    let time = now.elapsed();

    println!("Time to parse liquid.pdb {}ms", time.as_millis());

    let cell = liquid.unit_cell().size().clone();
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

    for atom in liquid.atoms_mut() {
        if atom.x() < 0.0
            || atom.x() > size.0
            || atom.y() < 0.0
            || atom.y() > size.1
            || atom.z() < 0.0
            || atom.z() > size.2
        {
            atom.remove();
        }
    }

    liquid.renumber();

    liquid
}
