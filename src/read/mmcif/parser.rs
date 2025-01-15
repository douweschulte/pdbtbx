use super::lexitem::*;
use crate::error::*;
use crate::structs::*;
use crate::validate::*;
use crate::ReadOptions;
use crate::StrictnessLevel;
use crate::TransformationMatrix;
use std::fs::File;
use std::io::prelude::*;

/// Parse the given mmCIF file into a PDB struct.
/// Returns a PDBError if a BreakingError is found. Otherwise it returns the PDB with all errors/warnings found while parsing it.
///
/// # Related
/// If you want to open a file from memory, see [`ReadOptions::read_raw`].
#[deprecated(
    since = "0.12.0",
    note = "Please use `ReadOptions::default().set_format(Format::Mmcif).read(filename)` instead"
)]
pub fn open_mmcif(
    filename: impl AsRef<str>,
    level: StrictnessLevel,
) -> Result<(PDB, Vec<PDBError>), Vec<PDBError>> {
    open_mmcif_with_options(filename, ReadOptions::default().set_level(level))
}

/// Parse the given mmCIF file into a PDB struct with [`ReadOptions`].
pub(crate) fn open_mmcif_with_options(
    filename: impl AsRef<str>,
    options: &ReadOptions,
) -> Result<(PDB, Vec<PDBError>), Vec<PDBError>> {
    let filename = filename.as_ref();
    let file = if let Ok(f) = File::open(filename) {
        f
    } else {
        return Err(vec![PDBError::new(ErrorLevel::BreakingError, "Could not open file", "Could not open the specified file, make sure the path is correct, you have permission, and that it is not open in another program.", Context::show(filename))]);
    };
    let reader = std::io::BufReader::new(file);
    open_mmcif_raw_with_options(reader, options)
}

/// Open's mmCIF file from a BufRead. This allows opening mmCIF files directly from memory.
///
/// This is particularly useful if you want to open a compressed file, as you can use the BufReader
#[deprecated(
    since = "0.12.0",
    note = "Please use `ReadOptions::default().set_format(Format::Mmcif).read_raw(input)` instead"
)]
pub fn open_mmcif_bufread<T>(
    bufreader: std::io::BufReader<T>,
) -> Result<(PDB, Vec<PDBError>), Vec<PDBError>>
where
    T: std::io::Read,
{
    ReadOptions::default()
        .set_format(crate::Format::Mmcif)
        .read_raw(bufreader)
}

/// Parse the given mmCIF `&str` into a PDB struct. This allows opening mmCIF files directly from memory.
/// Returns a PDBError if a BreakingError is found. Otherwise it returns the PDB with all errors/warnings found while parsing it.
///
/// # Related
/// If you want to open a file see [`open_mmcif`]. There is also a function to open a PDB file directly
/// see [`crate::open_pdb`] and [`crate::open_pdb_raw`]. If you want to open a general file
/// with no knowledge about the file type see [`crate::open`] and [`crate::open_raw`].
#[deprecated(
    since = "0.12.0",
    note = "Please use `ReadOptions::default().set_format(Format::Mmcif).read_raw(input)` instead"
)]
pub fn open_mmcif_raw(
    input: &str,
    level: StrictnessLevel,
) -> Result<(PDB, Vec<PDBError>), Vec<PDBError>> {
    match super::lexer::lex_cif(input) {
        Ok(data_block) => parse_mmcif(&data_block, level),
        Err(e) => Err(vec![e]),
    }
}

/// Parse the given stream into a [`PDB`] struct.
pub(crate) fn open_mmcif_raw_with_options<T>(
    mut input: std::io::BufReader<T>,
    options: &ReadOptions,
) -> Result<(PDB, Vec<PDBError>), Vec<PDBError>>
where
    T: std::io::Read,
{
    let mut contents = String::new();
    if input.read_to_string(&mut contents).is_ok() {
        match super::lexer::lex_cif(contents.as_str()) {
            Ok(data_block) => parse_mmcif_with_options(&data_block, options),
            Err(e) => Err(vec![e]),
        }
    } else {
        Err(vec![PDBError::new(
            crate::ErrorLevel::BreakingError,
            "Buffer could not be read",
            "The buffer provided to `open_raw` could not be read to end.",
            Context::None,
        )])
    }
}

/// Parse a CIF intermediate structure into a PDB
fn parse_mmcif(
    input: &DataBlock,
    level: StrictnessLevel,
) -> Result<(PDB, Vec<PDBError>), Vec<PDBError>> {
    parse_mmcif_with_options(input, ReadOptions::default().set_level(level))
}

/// Parse a CIF intermediate structure into a PDB with [`ReadOptions`].
///
/// # Related
/// See [`parse_mmcif`] for a version of this function with sane defaults.
fn parse_mmcif_with_options(
    input: &DataBlock,
    options: &ReadOptions,
) -> Result<(PDB, Vec<PDBError>), Vec<PDBError>> {
    let mut pdb = PDB::default();
    let mut errors: Vec<PDBError> = Vec::new();
    let mut unit_cell = UnitCell::default();
    let mut mtrix_id = None;

    pdb.identifier = Some(input.name.clone());

    for item in &input.items {
        let result = match item {
            Item::DataItem(di) => match di {
                DataItem::Loop(multiple) => {
                    if multiple.header.contains(&"atom_site.group_PDB".to_string()) {
                        parse_atoms(multiple, &mut pdb, options)
                    } else {
                        None
                    }
                }
                DataItem::Single(single) => {
                    let context = Context::show(&single.name);
                    match &single.name[..] {
                        "cell.length_a" => get_f64(&single.content, &context, None)
                            .map(|n| unit_cell.set_a(n.expect("UnitCell length a should be provided")))
                            .err(),
                        "cell.length_b" => get_f64(&single.content, &context, None)
                            .map(|n| unit_cell.set_b(n.expect("UnitCell length b should be provided")))
                            .err(),
                        "cell.length_c" => get_f64(&single.content, &context, None)
                            .map(|n| unit_cell.set_c(n.expect("UnitCell length c should be provided")))
                            .err(),
                        "cell.angle_alpha" => get_f64(&single.content, &context, None)
                            .map(|n| unit_cell.set_alpha(n.expect("UnitCell angle alpha should be provided")))
                            .err(),
                        "cell.angle_beta" => get_f64(&single.content, &context, None)
                            .map(|n| unit_cell.set_beta(n.expect("UnitCell angle beta should be provided")))
                            .err(),
                        "cell.angle_gamma" => get_f64(&single.content, &context, None)
                            .map(|n| unit_cell.set_gamma(n.expect("UnitCell angle gamma should be provided")))
                            .err(),
                        "symmetry.Int_Tables_number" | "space_group.IT_number" => {
                            if pdb.symmetry.is_none() {
                                get_usize(&single.content, &context, None)
                                    .map(|n| pdb.symmetry = Symmetry::from_index(n.expect("Symmetry international tables number should be provided")))
                                    .err()
                            } else if let Ok(Some(value)) = get_usize(&single.content, &context, None) {
                                if pdb.symmetry != Symmetry::from_index(value) {
                                    Some(PDBError::new(ErrorLevel::InvalidatingError, "Space group does not match", "The given space group does not match the space group earlier defined in this file.", context.clone()))
                                }
                                else {
                                    None
                                }
                            } else {
                                None
                            }
                        }
                        "symmetry.space_group_name_H-M" | "symmetry.space_group_name_Hall" | "space_group.name_H-M_alt" | "space_group.name_Hall" => {
                            if pdb.symmetry.is_none() {
                                get_text(&single.content, &context, None)
                                    .map(|t| pdb.symmetry = Symmetry::new(t.expect("Symmetry space group name should be provided")))
                                    .err()
                            } else if let Ok(Some(value)) = get_text(&single.content, &context, None) {
                                if pdb.symmetry != Symmetry::new(value) {
                                    Some(PDBError::new(ErrorLevel::InvalidatingError, "Space group does not match", "The given space group does not match the space group earlier defined in this file.", context.clone()))
                                }
                                else {
                                    None
                                }
                            } else {
                                None
                            }
                        }
                        s if s.starts_with("atom_sites.Cartn_transf") => {
                            if pdb.scale.is_none() {
                                pdb.scale = Some(TransformationMatrix::identity());
                            }
                            #[allow(clippy::unwrap_used)]
                            parse_matrix(s, get_f64(&single.content, &context, None),pdb.scale.as_mut().unwrap(), &context)
                        }
                        s if s.starts_with("database_PDB_matrix.origx") => {
                            if pdb.origx.is_none() {
                                pdb.origx = Some(TransformationMatrix::identity());
                            }
                            #[allow(clippy::unwrap_used)]
                            parse_matrix(s, get_f64(&single.content, &context, None),pdb.origx.as_mut().unwrap(), &context)
                        }
                        s if s.starts_with("structs_ncs_oper.") => {
                            if s.ends_with("id") {
                                match get_usize(&single.content, &context, None) {
                                    Err(e) => Some(e),
                                    Ok(Some(id)) => {
                                        mtrix_id = Some(id);
                                        pdb.add_mtrix(MtriX::new(id, TransformationMatrix::identity(), true));
                                        None
                                    }
                                    Ok(None) => Some(PDBError::new(
                                        ErrorLevel::InvalidatingError,
                                        "MtriX with missing ID",
                                        "If a MtriX id is given it should be a number not a missing value.",
                                        context.clone(),
                                    ))
                                }
                            } else {
                                match mtrix_id {
                                    Some(id) => {
                                        let mtrix = pdb.mtrix_mut().find(|m| m.serial_number == id).expect("Could not find the MtriX record with the previously found `_struct_ncs_oper.id`");
                                        if s.ends_with("code") {
                                            match get_text(&single.content, &context, None) {
                                                Ok(Some(t)) if t == "given" => {mtrix.contained = true; None},
                                                Ok(Some(t)) if t == "generate" => {mtrix.contained = false; None},
                                                Ok(Some(_)) => Some(PDBError::new(
                                                    ErrorLevel::InvalidatingError,
                                                    "MtriX code invalid",
                                                    "Only the values 'generate' and 'given' are valid for `_struct_ncs_oper.code`.",
                                                    context.clone(),
                                                )),
                                                _ => Some(PDBError::new(
                                                    ErrorLevel::InvalidatingError,
                                                    "MtriX code invalid",
                                                    "The value for `_struct_ncs_oper.code` should be a textual value.",
                                                    context.clone(),
                                                )),
                                            }
                                        } else if s.ends_with("details") {
                                            None // Ignore the details, it will not be saved somewhere
                                        } else {
                                            parse_matrix(s, get_f64(&single.content, &context, None),&mut mtrix.transformation, &context)
                                        }
                                    },
                                    None => Some(PDBError::new(
                                        ErrorLevel::InvalidatingError,
                                        "MtriX matrix given without ID",
                                        "The MtriX ID (`_struct_ncs_oper.id`) should be given before any matrix information is given.",
                                        context.clone(),
                                    ))
                                }
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
        pdb.unit_cell = Some(unit_cell);
    }

    reshuffle_conformers(&mut pdb);
    errors.extend(validate(&pdb));
    if errors.iter().any(|e| e.fails(options.level)) {
        Err(errors)
    } else {
        Ok((pdb, errors))
    }
}

/// Parse the name of this matrix defining line to find out the index it is pointing at and change that value in the given matrix.
fn parse_matrix(
    name: &str,
    value: Result<Option<f64>, PDBError>,
    matrix: &mut TransformationMatrix,
    context: &Context,
) -> Option<PDBError> {
    let get_index = |n| {
        if let Some(c) = name.chars().nth_back(n) {
            if let Some(n) = c.to_digit(10) {
                Ok((n - 1) as usize)
            } else {
                Err(PDBError::new(
                    ErrorLevel::InvalidatingError,
                    "Matrix item definition incorrect",
                    "There are no indices into the matrix. For example this is a valid name: `_database_PDB_matrix.origx[1][1]`",
                    context.clone(),
                ))
            }
        } else {
            Err(PDBError::new(
                ErrorLevel::InvalidatingError,
                "Matrix definition too short",
                "This matrix definition item name is too short to contain the matrix indices.",
                context.clone(),
            ))
        }
    };
    match value {
        Ok(o) => {
            if let Some(value) = o {
                if name.matches('[').count() == 2 {
                    // Two sets of braces so a matrix line
                    let r = match get_index(4) {
                        Ok(o) => o,
                        Err(e) => return Some(e),
                    };
                    let c = match get_index(1) {
                        Ok(o) => o,
                        Err(e) => return Some(e),
                    };
                    matrix.matrix_mut()[r][c] = value;
                } else {
                    // One set of braces so a vector line
                    let r = match get_index(1) {
                        Ok(o) => o,
                        Err(e) => return Some(e),
                    };
                    matrix.matrix_mut()[r][3] = value;
                }
                None // Everything went well
            } else {
                None // Ignore places where there is no value, assume it to be the default identity
            }
        }
        Err(e) => Some(e),
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
fn parse_atoms(input: &Loop, pdb: &mut PDB, options: &ReadOptions) -> Option<Vec<PDBError>> {
    #[derive(Eq, PartialEq)]
    /// The mode of a column
    enum Mode {
        /// A required column (has to be defined)
        Required,
        /// An optional column, if undefined it will have a default value
        Optional,
    }
    use Mode::{Optional, Required};

    /// Easily define all columns
    macro_rules! define_columns {
        ($($i:expr, $name:ident, $label:expr, $req:expr);+;) => {
            $(const $name: (usize, &str, Mode) = ($i, $label, $req);)+
            const COLUMNS: &[(Mode, &str)] = &[
                $(($req, $name.1)),+
            ];
        };
    }

    define_columns!(
        0,  ATOM_ALT_ID, "atom_site.label_alt_id", Optional;
        1,  ATOM_ANISOU_1_1, "_atom_site.aniso_U[1][1]", Optional;
        2,  ATOM_ANISOU_1_2, "_atom_site.aniso_U[1][2]", Optional;
        3,  ATOM_ANISOU_1_3, "_atom_site.aniso_U[1][3]", Optional;
        4,  ATOM_ANISOU_2_1, "_atom_site.aniso_U[2][1]", Optional;
        5,  ATOM_ANISOU_2_2, "_atom_site.aniso_U[2][2]", Optional;
        6,  ATOM_ANISOU_2_3, "_atom_site.aniso_U[2][3]", Optional;
        7,  ATOM_ANISOU_3_1, "_atom_site.aniso_U[3][1]", Optional;
        8,  ATOM_ANISOU_3_2, "_atom_site.aniso_U[3][2]", Optional;
        9,  ATOM_ANISOU_3_3, "_atom_site.aniso_U[3][3]", Optional;
        10, ATOM_ASYM_ID, "atom_site.label_asym_id", Required;
        11, ATOM_AUTH_ASYM_ID, "atom_site.auth_asym_id", Optional;
        12, ATOM_B, "atom_site.B_iso_or_equiv", Optional;
        13, ATOM_CHARGE, "atom_site.pdbx_formal_charge", Optional;
        14, ATOM_COMP_ID, "atom_site.label_comp_id", Required;
        15, ATOM_GROUP, "atom_site.group_PDB", Optional;
        16, ATOM_ID, "atom_site.id", Required;
        17, ATOM_INSERTION, "atom_site.pdbx_PDB_ins_code", Optional;
        18, ATOM_MODEL, "atom_site.pdbx_PDB_model_num", Optional;
        19, ATOM_NAME, "atom_site.label_atom_id", Required;
        20, ATOM_OCCUPANCY, "atom_site.occupancy", Optional;
        21, ATOM_SEQ_ID, "atom_site.label_seq_id", Required;
        22, ATOM_AUTH_SEQ_ID, "atom_site.auth_seq_id", Optional;
        23, ATOM_TYPE, "atom_site.type_symbol", Required;
        24, ATOM_X, "atom_site.Cartn_x", Required;
        25, ATOM_Y, "atom_site.Cartn_y", Required;
        26, ATOM_Z, "atom_site.Cartn_z", Required;
    );

    let positions_: Vec<Result<Option<usize>, PDBError>> = COLUMNS
        .iter()
        .map(|tag| (input.header.iter().position(|t| t == tag.1), tag))
        .map(|(pos, tag)| match pos {
            Some(p) => Ok(Some(p)),
            None if tag.0 == Required => Err(PDBError::new(
                ErrorLevel::InvalidatingError,
                "Missing column in coordinate atoms data loop",
                "The above column is missing",
                Context::show(tag.1),
            )),
            None => Ok(None),
        })
        .collect();

    let mut errors = positions_
        .iter()
        .filter_map(|i| i.clone().err())
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        return Some(errors);
    }

    // The previous lines make sure that there is no error in the vector.
    #[allow(clippy::unwrap_used)]
    let positions: Vec<Option<usize>> = positions_.iter().map(|i| *i.as_ref().unwrap()).collect();
    let mut first_model_number: usize = 0;
    for (index, row) in input.data.iter().enumerate() {
        let values: Vec<Option<&Value>> = positions.iter().map(|i| i.map(|x| &row[x])).collect();
        let context = Context::show(format!("Main atomic data loop row: {index}"));

        /// Parse a column given the function to use and the column index
        macro_rules! parse_column {
            ($type:tt, $index:tt) => {
                if let Some(value) = values[$index.0] {
                    match $type(value, &context, Some($index.1)) {
                        Ok(t) => t,
                        Err(e) => {
                            errors.push(e);
                            None
                        }
                    }
                } else {
                    None
                }
            };
        }

        // Early return cases
        let element = parse_column!(get_text, ATOM_TYPE).expect("Atom element should be provided");
        if options.discard_hydrogens & (element == "H") {
            continue;
        }
        let model_number = parse_column!(get_usize, ATOM_MODEL).unwrap_or(1);
        if options.only_first_model {
            if index == 0 {
                first_model_number = model_number;
            } else if model_number != first_model_number {
                break;
            }
        }

        // Parse remaining fields in the order they appear in the line
        let atom_type = parse_column!(get_text, ATOM_GROUP).unwrap_or_else(|| "ATOM".to_string());
        let name = parse_column!(get_text, ATOM_NAME).expect("Atom name should be provided");
        let serial_number =
            parse_column!(get_usize, ATOM_ID).expect("Atom serial number should be provided");
        let residue_name =
            parse_column!(get_text, ATOM_COMP_ID).expect("Residue name should be provided");
        #[allow(clippy::cast_possible_wrap)]
        let residue_number = parse_column!(get_isize, ATOM_AUTH_SEQ_ID).unwrap_or_else(|| {
            parse_column!(get_isize, ATOM_SEQ_ID)
                .unwrap_or_else(|| pdb.total_residue_count() as isize)
        });
        let chain_name = parse_column!(get_text, ATOM_AUTH_ASYM_ID).unwrap_or_else(|| {
            parse_column!(get_text, ATOM_ASYM_ID).expect("Chain name should be provided")
        });
        let pos_x = parse_column!(get_f64, ATOM_X).expect("Atom X position should be provided");
        let pos_y = parse_column!(get_f64, ATOM_Y).expect("Atom Y position should be provided");
        let pos_z = parse_column!(get_f64, ATOM_Z).expect("Atom Z position should be provided");
        let occupancy = parse_column!(get_f64, ATOM_OCCUPANCY).unwrap_or(1.0);
        let b_factor = parse_column!(get_f64, ATOM_B).unwrap_or(1.0);
        let charge = parse_column!(get_isize, ATOM_CHARGE).unwrap_or(0);
        let alt_loc = parse_column!(get_text, ATOM_ALT_ID);
        let insertion_code = parse_column!(get_text, ATOM_INSERTION);
        let aniso_temp = [
            [
                parse_column!(get_f64, ATOM_ANISOU_1_1),
                parse_column!(get_f64, ATOM_ANISOU_1_2),
                parse_column!(get_f64, ATOM_ANISOU_1_3),
            ],
            [
                parse_column!(get_f64, ATOM_ANISOU_2_1),
                parse_column!(get_f64, ATOM_ANISOU_2_2),
                parse_column!(get_f64, ATOM_ANISOU_2_3),
            ],
            [
                parse_column!(get_f64, ATOM_ANISOU_3_1),
                parse_column!(get_f64, ATOM_ANISOU_3_2),
                parse_column!(get_f64, ATOM_ANISOU_3_3),
            ],
        ];

        let aniso = if aniso_temp
            .iter()
            .flat_map(|l| l.iter())
            .all(Option::is_some)
        {
            #[allow(clippy::unwrap_used)]
            Some([
                [
                    aniso_temp[0][0].unwrap(),
                    aniso_temp[0][1].unwrap(),
                    aniso_temp[0][2].unwrap(),
                ],
                [
                    aniso_temp[1][0].unwrap(),
                    aniso_temp[1][1].unwrap(),
                    aniso_temp[1][2].unwrap(),
                ],
                [
                    aniso_temp[2][0].unwrap(),
                    aniso_temp[2][1].unwrap(),
                    aniso_temp[2][2].unwrap(),
                ],
            ])
        } else if aniso_temp
            .iter()
            .flat_map(|l| l.iter())
            .any(Option::is_some)
        {
            errors.push(PDBError::new(
                ErrorLevel::StrictWarning,
                "Atom aniso U definition incomplete",
                "For a valid anisotropic temperature factor definition all columns (1,1 up to and including 3,3) have to be defined.",
                context.clone(),
            ));
            None
        } else {
            None
        };

        let model = unsafe {
            // I could not find a way to make the borrow checker happy, but if no item
            // could be find the borrow should be ended and as such safe for mutating
            // in the second branch.
            let pdb_pointer: *mut PDB = pdb;
            if let Some(m) = (*pdb_pointer)
                .models_mut()
                .find(|m| m.serial_number() == model_number)
            {
                m
            } else {
                (*pdb_pointer).add_model(Model::new(model_number));
                #[allow(clippy::unwrap_used)]
                (*pdb_pointer).models_mut().next_back().unwrap()
            }
        };

        let mut hetero = false;
        if atom_type == "ATOM" {
            hetero = false;
        } else if atom_type == "HETATM" {
            hetero = true;
        } else {
            errors.push(PDBError::new(
                ErrorLevel::InvalidatingError,
                "Atom type not correct",
                "The atom type should be ATOM or HETATM",
                context.clone(),
            ))
        }
        // not used for .pdbqt
        todo!()
        // if let Some(mut atom) = Atom::new(
        //     hetero,
        //     serial_number,
        //     name,
        //     pos_x,
        //     pos_y,
        //     pos_z,
        //     occupancy,
        //     b_factor,
        //     element,
        //     charge,
        // ) {
        //     if let Some(matrix) = aniso {
        //         atom.set_anisotropic_temperature_factors(matrix);
        //     }

        //     model.add_atom(
        //         atom,
        //         chain_name,
        //         (residue_number, insertion_code.as_deref()),
        //         (residue_name, alt_loc.as_deref()),
        //     );
        // } else {
        //     errors.push(PDBError::new(
        //         ErrorLevel::InvalidatingError,
        //         "Atom definition incorrect",
        //         "The atom name and element should only contain valid characters.",
        //         context.clone(),
        //     ))
        // }
    }
    if !errors.is_empty() {
        Some(errors)
    } else {
        None
    }
}

/// Get the Textual content of the value, if available
fn get_text(
    value: &Value,
    _context: &Context,
    _column: Option<&str>,
) -> Result<Option<String>, PDBError> {
    match value {
        Value::Text(t) => Ok(Some(t.to_string())),
        Value::Inapplicable => Ok(None),
        Value::Unknown => Ok(None),
        Value::Numeric(n) => Ok(Some(format!("{n}"))),
        Value::NumericWithUncertainty(n, u) => Ok(Some(format!("{n}({u})"))),
    }
}

/// Get the Numeric content of the value, if available, it also fails on NumericWithUncertainty
fn get_f64(
    value: &Value,
    context: &Context,
    column: Option<&str>,
) -> Result<Option<f64>, PDBError> {
    match value {
        Value::Numeric(num) => Ok(Some(*num)),
        Value::Inapplicable => Ok(None),
        Value::Unknown => Ok(None),
        _ => Err(PDBError::new(
            ErrorLevel::InvalidatingError,
            "Not a number",
            column.map_or(String::new(), |v| {
                format!("The '{v}' column should contain a number.")
            }),
            context.clone(),
        )),
    }
}

/// Get the Numeric content of the value, if available, as a usize
fn get_usize(
    value: &Value,
    context: &Context,
    column: Option<&str>,
) -> Result<Option<usize>, PDBError> {
    flatten_result(get_f64(value, context, column).map(|result| {
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
                    column.map_or(String::new(), |v| {
                        format!("The '{v}' column should contain an unsigned integer.")
                    }),
                    context.clone(),
                ))
            }
        } else {
            Ok(None)
        }
    }))
}

/// Get the Numeric content of the value, if available, as an isize
fn get_isize(
    value: &Value,
    context: &Context,
    column: Option<&str>,
) -> Result<Option<isize>, PDBError> {
    flatten_result(get_f64(value, context, column).map(|result| {
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
                    column.map_or(String::new(), |v| {
                        format!("The '{v}' column should a singed integer.")
                    }),
                    context.clone(),
                ))
            }
        } else {
            Ok(None)
        }
    }))
}
