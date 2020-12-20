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

    let mut avg = 0.0;

    for atom in pdb.atoms() {
        avg += atom.b_factor();
    }

    avg = avg / (pdb.atoms().len() as f64);

    println!("Average B factor: {}", avg);
    println!("Scale: {:?}", pdb.scale().factors);

    save::save(&pdb, &format!("{}_saved", args[1]));
}
