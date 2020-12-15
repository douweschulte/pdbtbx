mod parser;
mod lexitem;
mod structs;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut pdb = parser::open(&args[1]).unwrap();

    println!("Found {} atoms, in {} residues, in {} chains, in {} models", pdb.atoms().len(), pdb.residues().len(), pdb.chains().len(), pdb.models.len());

    let mut avg = 0.0;

    for atom in pdb.atoms() {
        avg += atom.b_factor();
    }

    avg = avg / (pdb.atoms().len() as f64);

    println!("Average B factor: {}", avg);
    println!("Scale: {:?}", pdb.scale().factors);
}
