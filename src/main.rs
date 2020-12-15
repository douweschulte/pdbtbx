mod parser;
mod lexitem;
mod structs;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1])
        .expect("Something went wrong reading the file");

    let lexed = parser::lex(&contents).expect("Something wrong wile lexing");
    let mut pdb = parser::parse(&lexed);
    let mut avg = 0.0;

    for atom in pdb.atoms() {
        avg += atom.b_factor();
    }

    avg = avg / (pdb.atoms().len() as f64);

    println!("Average B factor: {}", avg);
    println!("Scale: {:?}", pdb.scale().factors);
}
