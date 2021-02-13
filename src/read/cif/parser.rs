#![allow(clippy::missing_docs_in_private_items, clippy::unwrap_used)]
use super::lexitem::*;
use crate::error::*;
use crate::structs::*;
use std::fs::File;
use std::io::prelude::*;

/// !!UNSTABLE!!
/// Parse the given mmCIF file.
pub fn open_mmcif(filename: &str) -> Result<PDB, PDBError> {
    let mut file = if let Ok(f) = File::open(filename) {
        f
    } else {
        return Err(PDBError::new(ErrorLevel::BreakingError, "Could not open file", "Could not open the specified file, make sure the path is correct, you have permission, and that it is not open in another program.", Context::show(filename)));
    };
    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        return Err(PDBError::new(
            ErrorLevel::BreakingError,
            "Error while reading file",
            &format!("Error: {}", e),
            Context::show(filename),
        ));
    }
    match super::lexer::lex_cif(contents) {
        Ok(data_block) => parse_mmcif(&data_block),
        Err(e) => Err(e),
    }
}

/// !!UNSTABLE!!
/// Parse a CIF file into CIF intermediate structure
fn parse_mmcif(input: &DataBlock) -> Result<PDB, PDBError> {
    let mut pdb = PDB::default();
    if let Some(sym) = parse_symmetry(input) {
        pdb.set_symmetry(sym);
    }
    if let Some(unit_cell) = parse_unit_cell(input) {
        pdb.set_unit_cell(unit_cell);
    }
    parse_atoms(input, &mut pdb);
    unimplemented!();
}

macro_rules! find_numeric {
    ($input:ident, $tag:expr, $body:expr) => {
        if let Some(item) = $input.find($tag) {
            if let Value::Numeric(num) = item.content {
                $body(num);
            }
        }
    };
}

fn parse_unit_cell(input: &DataBlock) -> Option<UnitCell> {
    let mut output = UnitCell::default();
    find_numeric!(input, "cell.length_a", |num| output.set_a(num));
    find_numeric!(input, "cell.length_b", |num| output.set_b(num));
    find_numeric!(input, "cell.length_c", |num| output.set_c(num));
    find_numeric!(input, "cell.angle_alpha", |num| output.set_alpha(num));
    find_numeric!(input, "cell.angle_beta", |num| output.set_beta(num));
    find_numeric!(input, "cell.angle_gamma", |num| output.set_gamma(num));
    if output == UnitCell::default() {
        None
    } else {
        Some(output)
    }
}

fn parse_symmetry(input: &DataBlock) -> Option<Symmetry> {
    if let Some(item) = input.find("Int_Tables_number") {
        if let Value::Numeric(num) = item.content {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            Symmetry::from_index(num as usize)
        } else {
            None
        }
    } else if let Some(item) = input.find("space_group_name_H-M") {
        if let Value::Text(ident) = &item.content {
            Symmetry::new(ident)
        } else {
            None
        }
    } else {
        None
    }
}

fn parse_atoms(_input: &DataBlock, _pdb: &mut PDB) -> Vec<Atom> {
    /*
    Find this specific loop, then align all columns (can be shuffled in any order)
    loop_
    _atom_site.group_PDB
    _atom_site.id
    _atom_site.type_symbol
    _atom_site.label_atom_id
    _atom_site.label_alt_id
    _atom_site.label_comp_id
    _atom_site.label_asym_id
    _atom_site.label_entity_id
    _atom_site.label_seq_id
    _atom_site.pdbx_PDB_ins_code
    _atom_site.Cartn_x
    _atom_site.Cartn_y
    _atom_site.Cartn_z
    _atom_site.occupancy
    _atom_site.B_iso_or_equiv
    _atom_site.pdbx_formal_charge
    _atom_site.auth_seq_id
    _atom_site.auth_comp_id
    _atom_site.auth_asym_id
    _atom_site.auth_atom_id
    _atom_site.pdbx_PDB_model_num */
    unimplemented!();
}
