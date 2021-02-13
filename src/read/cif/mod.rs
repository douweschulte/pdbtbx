/// Parse CIF files into intermediate structure
mod lexer;
/// Save the CIF intermediate structure
mod lexitem;
/// Parse intermediate structure to PDB structure
mod parser;

pub use parser::*;
