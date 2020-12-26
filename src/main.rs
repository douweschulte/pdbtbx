mod lexitem;
mod parser;
mod save;
mod structs;

use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    let now = Instant::now();

    let mut pdb = parser::parse(&args[1]).unwrap();

    let time = now.elapsed();

    println!(
        "Found {} atoms, in {} residues, in {} chains, in {} models it all took {} ms",
        pdb.atoms().len(),
        pdb.residues().len(),
        pdb.chains().len(),
        pdb.models.len(),
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
        if atom.anisotropic_temperature_factors().is_none() {
            println!("No ANISOU for atom {}", atom);
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

    println!(
        "{} {} {} {} {}",
        pdb.atoms()[62],
        pdb.atoms()[62].residue(),
        pdb.atoms()[62].residue().chain(),
        pdb.atoms()[62].residue().chain().model(),
        pdb.atoms()[62].residue().chain().model().pdb()
    );

    save::save(&pdb, &format!("{}_saved", args[1])).expect("Save not successful");
}
