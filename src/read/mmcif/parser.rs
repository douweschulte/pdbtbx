#![allow(clippy::missing_docs_in_private_items, clippy::unwrap_used)]
use super::lexitem::*;
use crate::error::*;
use crate::structs::*;
use crate::validate::validate;
use crate::StrictnessLevel;
use std::fs::File;
use std::io::prelude::*;

/// !!UNSTABLE!!
/// Parse the given mmCIF file.
pub fn open_mmcif(
    filename: &str,
    level: StrictnessLevel,
) -> Result<(PDB, Vec<PDBError>), Vec<PDBError>> {
    let mut file = if let Ok(f) = File::open(filename) {
        f
    } else {
        return Err(vec![PDBError::new(ErrorLevel::BreakingError, "Could not open file", "Could not open the specified file, make sure the path is correct, you have permission, and that it is not open in another program.", Context::show(filename))]);
    };
    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        return Err(vec![PDBError::new(
            ErrorLevel::BreakingError,
            "Error while reading file",
            &format!("Error: {}", e),
            Context::show(filename),
        )]);
    }
    match super::lexer::lex_cif(contents) {
        Ok(data_block) => parse_mmcif(&data_block, level),
        Err(e) => Err(vec![e]),
    }
}

/// !!UNSTABLE!!
/// Parse a CIF file into CIF intermediate structure
fn parse_mmcif(
    input: &DataBlock,
    level: StrictnessLevel,
) -> Result<(PDB, Vec<PDBError>), Vec<PDBError>> {
    let mut pdb = PDB::default();
    let mut errors: Vec<PDBError> = Vec::new();
    let mut unit_cell = UnitCell::default();

    for item in &input.items {
        let result = match item {
            Item::DataItem(di) => match di {
                DataItem::Loop(multiple) => {
                    if multiple
                        .header
                        .contains(&"_atom_site.group_PDB".to_string())
                    {
                        parse_atoms(multiple, &mut pdb)
                    } else {
                        None
                    }
                }
                DataItem::Single(single) => {
                    let context = Context::show(&single.name);
                    match &single.name[..] {
                        "cell.length_a" => get_f64(&single.content, &context)
                            .map(|n| unit_cell.set_a(n))
                            .err(),
                        "cell.length_b" => get_f64(&single.content, &context)
                            .map(|n| unit_cell.set_b(n))
                            .err(),
                        "cell.length_c" => get_f64(&single.content, &context)
                            .map(|n| unit_cell.set_c(n))
                            .err(),
                        "cell.angle_alpha" => get_f64(&single.content, &context)
                            .map(|n| unit_cell.set_alpha(n))
                            .err(),
                        "cell.angle_beta" => get_f64(&single.content, &context)
                            .map(|n| unit_cell.set_beta(n))
                            .err(),
                        "cell.angle_gamma" => get_f64(&single.content, &context)
                            .map(|n| unit_cell.set_gamma(n))
                            .err(),
                        "Int_Tables_number" => {
                            if !pdb.has_symmetry() {
                                get_usize(&single.content, &context)
                                    .map(|n| Symmetry::from_index(n).map(|s| pdb.set_symmetry(s)))
                                    .err()
                            } else {
                                None
                            }
                        }
                        "space_group_name_H-M" => {
                            if !pdb.has_symmetry() {
                                get_text(&single.content, &context)
                                    .map(|t| Symmetry::new(t).map(|s| pdb.set_symmetry(s)))
                                    .err()
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                    .map(|e| vec![e])
                }
            },
            _ => None,
        };
        if let Some(e) = result {
            errors.extend(e);
        }
    }

    if unit_cell != UnitCell::default() {
        pdb.set_unit_cell(unit_cell);
    }
    errors.extend(validate(&pdb));
    if errors.iter().any(|e| e.fails(level)) {
        Err(errors)
    } else {
        Ok((pdb, errors))
    }
}

fn flatten_result<T, E>(value: Result<Result<T, E>, E>) -> Result<T, E> {
    match value {
        Ok(Ok(t)) => Ok(t),
        Ok(Err(e)) => Err(e),
        Err(e) => Err(e),
    }
}

fn parse_atoms(input: &Loop, pdb: &mut PDB) -> Option<Vec<PDBError>> {
    const COLUMNS: &[&str] = &[
        "_atom_site.group_PDB",
        "_atom_site.label_atom_id",
        "_atom_site.id",
        "_atom_site.type_symbol",
        "_atom_site.label_comp_id",
        "_atom_site.label_seq_id",
        "_atom_site.label_asym_id",
        "_atom_site.Cartn_x",
        "_atom_site.Cartn_y",
        "_atom_site.Cartn_z",
        "_atom_site.occupancy",
        "_atom_site.B_iso_or_equiv",
        "_atom_site.pdbx_formal_charge",
        "_atom_site.pdbx_PDB_model_num",
    ];

    let positions_: Vec<Result<usize, PDBError>> = COLUMNS
        .iter()
        .map(|tag| (input.header.iter().position(|t| t == tag), tag))
        .map(|(pos, tag)| match pos {
            Some(p) => Ok(p),
            None => Err(PDBError::new(
                ErrorLevel::StrictWarning,
                "Missing column in coordinate atoms data loop",
                "The above column is missing",
                Context::show(tag),
            )),
        })
        .collect();

    let mut errors = positions_
        .iter()
        .filter_map(|i| i.clone().err())
        .collect::<Vec<_>>();

    if errors.len() > 0 {
        return Some(errors);
    }

    let positions: Vec<usize> = positions_.iter().map(|i| *i.as_ref().unwrap()).collect();

    for (index, row) in input.data.iter().enumerate() {
        let values: Vec<&Value> = positions.iter().map(|i| &row[*i]).collect();
        let context = Context::show(&format!("Main atomic data loop row: {}", index));

        macro_rules! parse_column {
            ($type:tt, $index:tt) => {
                match $type(values[$index], &context) {
                    Ok(t) => t,
                    Err(e) => {
                        errors.push(e);
                        continue;
                    }
                }
            };
        }

        let atom_type = parse_column!(get_text, 0);
        let name = parse_column!(get_text, 1);
        let serial_number = parse_column!(get_usize, 2);
        let element = parse_column!(get_text, 3);
        let residue_name = parse_column!(get_text, 4);
        let residue_number = parse_column!(get_usize, 5);
        let chain_name = parse_column!(get_text, 6);
        let x = parse_column!(get_f64, 8);
        let y = parse_column!(get_f64, 9);
        let z = parse_column!(get_f64, 10);
        let q = parse_column!(get_f64, 11);
        let b = parse_column!(get_f64, 12);
        let charge = parse_column!(get_isize, 13);
        let model_number = parse_column!(get_usize, 14);

        let model = unsafe {
            // I could not find a way to make the borrow checker happy, but if no item
            // could be find the borrow should be ended and as such safe for mutating
            // in the second branch.
            let pdb_pntr: *mut PDB = pdb;
            if let Some(m) = (*pdb_pntr)
                .models_mut()
                .find(|m| m.serial_number() == model_number)
            {
                m
            } else {
                (*pdb_pntr).add_model(Model::new(model_number));
                (*pdb_pntr).models_mut().rev().next().unwrap()
            }
        };

        let atom = Atom::new(
            serial_number,
            name.to_string(),
            x,
            y,
            z,
            q,
            b,
            element.to_string(),
            charge,
        )
        .unwrap();

        if atom_type == "ATOM" {
            model.add_atom(
                atom,
                chain_name.to_string(),
                residue_number,
                residue_name.to_string(),
            );
        } else if atom_type == "HETATM" {
            model.add_hetero_atom(
                atom,
                chain_name.to_string(),
                residue_number,
                residue_name.to_string(),
            );
        } else {
            errors.push(PDBError::new(
                ErrorLevel::InvalidatingError,
                "Atom type not correct",
                "The atom type should be ATOM or HETATM",
                context.clone(),
            ))
        }
    }
    if errors.len() > 0 {
        Some(errors)
    } else {
        None
    }
}

fn get_text<'a, 'b>(value: &'a Value, context: &'b Context) -> Result<&'a str, PDBError> {
    match value {
        Value::Text(t) => Ok(t),
        _ => Err(PDBError::new(
            ErrorLevel::InvalidatingError,
            "Not a number",
            "",
            context.clone(),
        )),
    }
}

fn get_f64(value: &Value, context: &Context) -> Result<f64, PDBError> {
    match value {
        Value::Numeric(num) => Ok(*num),
        _ => Err(PDBError::new(
            ErrorLevel::InvalidatingError,
            "Not a number",
            "",
            context.clone(),
        )),
    }
}

fn get_usize(value: &Value, context: &Context) -> Result<usize, PDBError> {
    flatten_result(get_f64(value, context).map(|num| {
        #[allow(
            clippy::cast_precision_loss,
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss
        )]
        if num > 0.0 && num < std::usize::MAX as f64 && num.trunc() == num {
            Ok(num as usize)
        } else {
            Err(PDBError::new(
                ErrorLevel::InvalidatingError,
                "Not a unsigned integer",
                "",
                context.clone(),
            ))
        }
    }))
}

fn get_isize(value: &Value, context: &Context) -> Result<isize, PDBError> {
    flatten_result(get_f64(value, context).map(|num| {
        #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
        if num > std::isize::MIN as f64 && num < std::isize::MAX as f64 && num.trunc() == num {
            Ok(num as isize)
        } else {
            Err(PDBError::new(
                ErrorLevel::InvalidatingError,
                "Not an integer",
                "",
                context.clone(),
            ))
        }
    }))
}
