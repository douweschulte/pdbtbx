mod parser;
mod save;
mod structs;
mod transformation;

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
        pdb.all_atoms().collect::<Vec<_>>().len(),
        pdb.all_residues().collect::<Vec<_>>().len(),
        pdb.all_chains().collect::<Vec<_>>().len(),
        pdb.models().collect::<Vec<_>>().len(),
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
    println!("Scale: {:?}", pdb.scale().factors);

    pdb.renumber();

    save::save(&pdb, &format!("{}_saved", args[1])).expect("Save not successful");
}
