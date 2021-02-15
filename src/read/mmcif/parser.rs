use super::lexitem::*;
use crate::error::*;
use crate::structs::*;
use crate::validate::validate;
use crate::StrictnessLevel;
use std::fs::File;
use std::io::prelude::*;

/// Parse the given mmCIF file into a PDB struct.
/// Returns an PDBError when it found a BreakingError. Otherwise it returns the PDB with all errors/warnings found while parsing it.
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

/// Parse a CIF intermediate structure into a PDB
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
                    if multiple.header.contains(&"atom_site.group_PDB".to_string()) {
                        parse_atoms(multiple, &mut pdb)
                    } else {
                        None
                    }
                }
                DataItem::Single(single) => {
                    let context = Context::show(&single.name);
                    match &single.name[..] {
                        "cell.length_a" => get_f64(&single.content, &context)
                            .map(|n| unit_cell.set_a(n.expect("UnitCell length a should be provided")))
                            .err(),
                        "cell.length_b" => get_f64(&single.content, &context)
                            .map(|n| unit_cell.set_b(n.expect("UnitCell length b should be provided")))
                            .err(),
                        "cell.length_c" => get_f64(&single.content, &context)
                            .map(|n| unit_cell.set_c(n.expect("UnitCell length c should be provided")))
                            .err(),
                        "cell.angle_alpha" => get_f64(&single.content, &context)
                            .map(|n| unit_cell.set_alpha(n.expect("UnitCell angle alpha should be provided")))
                            .err(),
                        "cell.angle_beta" => get_f64(&single.content, &context)
                            .map(|n| unit_cell.set_beta(n.expect("UnitCell angle beta should be provided")))
                            .err(),
                        "cell.angle_gamma" => get_f64(&single.content, &context)
                            .map(|n| unit_cell.set_gamma(n.expect("UnitCell angle gamma should be provided")))
                            .err(),
                        "Int_Tables_number" => {
                            if !pdb.has_symmetry() {
                                get_usize(&single.content, &context)
                                    .map(|n| Symmetry::from_index(n.expect("Symmetry international tables number should be provided")).map(|s| pdb.set_symmetry(s)))
                                    .err()
                            } else {
                                None
                            }
                        }
                        "space_group_name_H-M" => {
                            if !pdb.has_symmetry() {
                                get_text(&single.content, &context)
                                    .map(|t| Symmetry::new(t.expect("Symmetry space group name H-M should be provided")).map(|s| pdb.set_symmetry(s)))
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

/// Flatten a Result of a Result with the same error type (#70142 is still unstable)
fn flatten_result<T, E>(value: Result<Result<T, E>, E>) -> Result<T, E> {
    match value {
        Ok(Ok(t)) => Ok(t),
        Ok(Err(e)) => Err(e),
        Err(e) => Err(e),
    }
}

/// Parse a loop containing atomic data
fn parse_atoms(input: &Loop, pdb: &mut PDB) -> Option<Vec<PDBError>> {
    /// These are the columns needed to fill out the PDB correctly
    const COLUMNS: &[&str] = &[
        "atom_site.group_PDB",
        "atom_site.label_atom_id",
        "atom_site.id",
        "atom_site.type_symbol",
        "atom_site.label_comp_id",
        "atom_site.label_seq_id",
        "atom_site.label_asym_id",
        "atom_site.Cartn_x",
        "atom_site.Cartn_y",
        "atom_site.Cartn_z",
        "atom_site.occupancy",
        "atom_site.B_iso_or_equiv",
        "atom_site.pdbx_formal_charge",
        "atom_site.pdbx_PDB_model_num",
    ];

    let positions_: Vec<Result<usize, PDBError>> = COLUMNS
        .iter()
        .map(|tag| (input.header.iter().position(|t| t == tag), tag))
        .map(|(pos, tag)| match pos {
            Some(p) => Ok(p),
            None => Err(PDBError::new(
                ErrorLevel::InvalidatingError,
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

    if !errors.is_empty() {
        return Some(errors);
    }

    #[allow(clippy::unwrap_used)]
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

        let atom_type = parse_column!(get_text, 0).expect("Atom type should be defined");
        let name = parse_column!(get_text, 1).expect("Atom name should be provided");
        let serial_number =
            parse_column!(get_usize, 2).expect("Atom serial number should be provided");
        let element = parse_column!(get_text, 3).expect("Atom element should be provided");
        let residue_name = parse_column!(get_text, 4).expect("Residue name should be provided");
        let residue_number =
            parse_column!(get_usize, 5).unwrap_or_else(|| pdb.total_residue_count());
        let chain_name = parse_column!(get_text, 6).expect("Chain name should be provided");
        let pos_x = parse_column!(get_f64, 7).expect("Atom X position should be provided");
        let pos_y = parse_column!(get_f64, 8).expect("Atom Y position should be provided");
        let pos_z = parse_column!(get_f64, 9).expect("Atom Z position should be provided");
        let occupancy = parse_column!(get_f64, 10).unwrap_or(1.0);
        let b_factor = parse_column!(get_f64, 11).unwrap_or(1.0);
        let charge = parse_column!(get_isize, 12).unwrap_or(0);
        let model_number =
            parse_column!(get_usize, 13).expect("Model serial number should be provided");

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
                #[allow(clippy::unwrap_used)]
                (*pdb_pntr).models_mut().rev().next().unwrap()
            }
        };

        if let Some(atom) = Atom::new(
            serial_number,
            name.to_string(),
            pos_x,
            pos_y,
            pos_z,
            occupancy,
            b_factor,
            element.to_string(),
            charge,
        ) {
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
        } else {
            errors.push(PDBError::new(
                ErrorLevel::InvalidatingError,
                "Atom definition incorrect",
                "The atom name and element should only contain valid characters.",
                context.clone(),
            ))
        }
    }
    if !errors.is_empty() {
        Some(errors)
    } else {
        None
    }
}

/// Get the Textual content of the value, if available
fn get_text<'a, 'b>(value: &'a Value, context: &'b Context) -> Result<Option<&'a str>, PDBError> {
    match value {
        Value::Text(t) => Ok(Some(t)),
        Value::Inapplicable => Ok(None),
        Value::Unknown => Ok(None),
        _ => Err(PDBError::new(
            ErrorLevel::InvalidatingError,
            "Not text",
            "",
            context.clone(),
        )),
    }
}

/// Get the Numeric content of the value, if available, it also fails on NumericWithUncertainty
fn get_f64(value: &Value, context: &Context) -> Result<Option<f64>, PDBError> {
    match value {
        Value::Numeric(num) => Ok(Some(*num)),
        Value::Inapplicable => Ok(None),
        Value::Unknown => Ok(None),
        _ => Err(PDBError::new(
            ErrorLevel::InvalidatingError,
            "Not a number",
            "",
            context.clone(),
        )),
    }
}

/// Get the Numeric content of the value, if available, as a usize
fn get_usize(value: &Value, context: &Context) -> Result<Option<usize>, PDBError> {
    flatten_result(get_f64(value, context).map(|result| {
        if let Some(num) = result {
            #[allow(
                clippy::cast_precision_loss,
                clippy::cast_possible_truncation,
                clippy::cast_sign_loss,
                clippy::float_cmp
            )]
            if (0.0..std::usize::MAX as f64).contains(&num) && num.trunc() == num {
                Ok(Some(num as usize))
            } else {
                Err(PDBError::new(
                    ErrorLevel::InvalidatingError,
                    "Not an unsigned integer",
                    "",
                    context.clone(),
                ))
            }
        } else {
            Ok(None)
        }
    }))
}

/// Get the Numeric content of the value, if available, as an isize
fn get_isize(value: &Value, context: &Context) -> Result<Option<isize>, PDBError> {
    flatten_result(get_f64(value, context).map(|result| {
        if let Some(num) = result {
            #[allow(
                clippy::cast_precision_loss,
                clippy::cast_possible_truncation,
                clippy::float_cmp
            )]
            if (std::isize::MIN as f64..std::isize::MAX as f64).contains(&num) && num.trunc() == num
            {
                Ok(Some(num as isize))
            } else {
                Err(PDBError::new(
                    ErrorLevel::InvalidatingError,
                    "Not an integer",
                    "",
                    context.clone(),
                ))
            }
        } else {
            Ok(None)
        }
    }))
}
