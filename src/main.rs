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
}
