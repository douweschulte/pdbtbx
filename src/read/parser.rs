use super::lexitem::*;
use crate::error::*;
use crate::reference_tables;
use crate::structs::*;
use crate::validate::*;
use crate::StrictnessLevel;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

/// Parse the given file into a PDB struct.
/// Returns an PDBError when it found a BreakingError. Otherwise it returns the PDB with all errors/warnings found while parsing it.
pub fn open(filename: &str, level: StrictnessLevel) -> Result<(PDB, Vec<PDBError>), Vec<PDBError>> {
    // Open a file a use a buffered reader to minimise memory use while immediately lexing the line followed by adding it to the current PDB
    let file = if let Ok(f) = File::open(filename) {
        f
    } else {
        return Err(vec![PDBError::new(ErrorLevel::BreakingError, "Could not open file", "Could not open the specified file, make sure the path is correct, you have permission, and that it is not open in another program.", Context::show(filename))]);
    };
    let reader = BufReader::new(file);
    parse(reader, Context::show(filename), level)
}

/// Parse the input stream into a PDB struct. To allow for direct streaming from sources, like from RCSB.org.
/// Returns an PDBError when it found a BreakingError. Otherwise it returns the PDB with all errors/warnings found while parsing it.
///
/// ## Arguments
/// * `input` - the input stream
/// * `context` - the context of the full stream, to place error messages correctly, for files this is `Context::show(filename)`.
/// * `level` - the strictness level to operate in. If errors are generated which are breaking in the given level the parsing will fail.
pub fn parse<T>(
    input: std::io::BufReader<T>,
    context: Context,
    level: StrictnessLevel,
) -> Result<(PDB, Vec<PDBError>), Vec<PDBError>>
where
    T: std::io::Read,
{
    let mut errors = Vec::new();
    let mut pdb = PDB::new();
    let mut current_model = Model::new(0);

    for (mut linenumber, read_line) in input.lines().enumerate() {
        linenumber += 1; // 1 based indexing in files

        let line = if let Ok(l) = read_line {
            l
        } else {
            return Err(vec![PDBError::new(
                ErrorLevel::BreakingError,
                "Could read line",
                &format!(
                    "Could not read line {} while parsing the input file.",
                    linenumber
                ),
                context,
            )]);
        };
        let lineresult = if line.len() > 6 {
            match &line[..6] {
                "REMARK" => lex_remark(linenumber, line),
                "ATOM  " => lex_atom(linenumber, line, false),
                "ANISOU" => lex_anisou(linenumber, line),
                "HETATM" => lex_atom(linenumber, line, true),
                "CRYST1" => lex_cryst(linenumber, line),
                "SCALE1" => lex_scale(linenumber, line, 0),
                "SCALE2" => lex_scale(linenumber, line, 1),
                "SCALE3" => lex_scale(linenumber, line, 2),
                "ORIGX1" => lex_origx(linenumber, line, 0),
                "ORIGX2" => lex_origx(linenumber, line, 1),
                "ORIGX3" => lex_origx(linenumber, line, 2),
                "MTRIX1" => lex_mtrix(linenumber, line, 0),
                "MTRIX2" => lex_mtrix(linenumber, line, 1),
                "MTRIX3" => lex_mtrix(linenumber, line, 2),
                "MODEL " => lex_model(linenumber, line),
                "MASTER" => lex_master(linenumber, line),
                "ENDMDL" => Ok((LexItem::EndModel(), Vec::new())),
                "TER   " => Ok((LexItem::TER(), Vec::new())),
                "END   " => Ok((LexItem::End(), Vec::new())),
                _ => Err(PDBError::new(ErrorLevel::GeneralWarning, "Could not recognise tag.", "Could not parse the tag above, it is possible that it is valid PDB but just not supported right now.",Context::full_line(linenumber, &line))),
            }
        } else if line.len() > 2 {
            match &line[..3] {
                "TER" => Ok((LexItem::TER(), Vec::new())),
                "END" => Ok((LexItem::End(), Vec::new())),
                _ => Err(PDBError::new(ErrorLevel::GeneralWarning, "Could not recognise tag.", "Could not parse the tag above, it is possible that it is valid PDB but just not supported right now.",Context::full_line(linenumber, &line))),
            }
        } else if !line.is_empty() {
            Err(PDBError::new(ErrorLevel::GeneralWarning, "Could not recognise tag.", "Could not parse the tag above, it is possible that it is valid PDB but just not supported right now.",Context::full_line(linenumber, &line)))
        } else {
            Ok((LexItem::Empty(), Vec::new()))
        };

        // Then immediately add this lines information to the final PDB struct
        if let Ok((result, line_errors)) = lineresult {
            errors.extend(line_errors);
            match result {
                LexItem::Remark(num, text) => pdb.add_remark(num, text.to_string()),
                LexItem::Atom(
                    hetero,
                    serial_number,
                    name,
                    _,
                    residue_name,
                    chain_id,
                    residue_serial_number,
                    _,
                    x,
                    y,
                    z,
                    occ,
                    b,
                    _,
                    element,
                    charge,
                ) => {
                    let atom = Atom::new(serial_number, name, x, y, z, occ, b, element, charge)
                        .expect("Invalid characters in atom creation");

                    if hetero {
                        current_model.add_hetero_atom(
                            atom,
                            chain_id,
                            residue_serial_number,
                            residue_name,
                        );
                    } else {
                        current_model.add_atom(atom, chain_id, residue_serial_number, residue_name);
                    }
                }
                LexItem::Anisou(s, n, _, _r, _c, _rs, _, factors, _, _e, _ch) => {
                    let mut found = false;
                    for atom in current_model.all_atoms_mut().rev() {
                        if atom.serial_number() == s {
                            atom.set_anisotropic_temperature_factors(factors);
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        println!(
                            "Could not find atom for temperature factors, coupled to atom {} {}",
                            s,
                            n.iter().collect::<String>()
                        )
                    }
                }
                LexItem::Model(number) => {
                    if current_model.atom_count() > 0 {
                        pdb.add_model(current_model)
                    }

                    current_model = Model::new(number);
                }
                LexItem::Scale(n, row) => {
                    if !pdb.has_scale() {
                        pdb.set_scale(Scale::new());
                    }
                    pdb.scale_mut().set_row(n, row);
                }
                LexItem::OrigX(n, row) => {
                    if !pdb.has_origx() {
                        pdb.set_origx(OrigX::new());
                    }
                    pdb.origx_mut().set_row(n, row);
                }
                LexItem::MtriX(n, ser, row, given) => {
                    let mut found = false;
                    for mtrix in pdb.mtrix_mut() {
                        if mtrix.serial_number == ser {
                            mtrix.set_row(n, row);
                            mtrix.contained = given;
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        let mut mtrix = MtriX::new();
                        mtrix.serial_number = ser;
                        mtrix.set_row(n, row);
                        mtrix.contained = given;
                        pdb.add_mtrix(mtrix);
                    }
                }
                LexItem::Crystal(a, b, c, alpha, beta, gamma, spacegroup, _z) => {
                    pdb.set_unit_cell(UnitCell::new(a, b, c, alpha, beta, gamma));
                    pdb.set_symmetry(
                        Symmetry::new(&spacegroup)
                            .unwrap_or_else(|| panic!("Invalid space group: \"{}\"", spacegroup)),
                    );
                }
                LexItem::Master(
                    num_remark,
                    num_empty,
                    _num_het,
                    _num_helix,
                    _num_sheet,
                    _num_turn,
                    _num_site,
                    num_xform,
                    num_coord,
                    _num_ter,
                    _num_connect,
                    _num_seq,
                ) => {
                    // This has to be one of the last lines so push the current model
                    if current_model.total_atom_count() > 0 {
                        pdb.add_model(current_model);
                        current_model = Model::new(0);
                    }
                    // The for now forgotten numbers will have to be added when the appropriate records are added to the parser
                    if num_remark != pdb.remark_count() {
                        errors.push(
                            PDBError::new(
                                ErrorLevel::StrictWarning,
                                "MASTER checksum failed",
                                &format!("The number of REMARKS ({}) is different then posed in the MASTER Record ({})", pdb.remark_count(), num_remark),
                                context.clone()
                            )
                        );
                    }
                    if num_empty != 0 {
                        errors.push(
                            PDBError::new(
                                ErrorLevel::LooseWarning,
                                "MASTER checksum failed",
                                &format!("The empty checksum number is not empty (value: {}) while it is defined to be empty.", num_empty),
                                context.clone()
                            )
                        );
                    }
                    let mut xform = 0;
                    if pdb.has_origx() && pdb.origx().valid() {
                        xform += 3;
                    }
                    if pdb.has_scale() && pdb.scale().valid() {
                        xform += 3;
                    }
                    for mtrix in pdb.mtrix() {
                        if mtrix.valid() {
                            xform += 3;
                        }
                    }
                    if num_xform != xform {
                        errors.push(
                            PDBError::new(
                                ErrorLevel::StrictWarning,
                                "MASTER checksum failed",
                                &format!("The number of coordinate transformation records ({}) is different then posed in the MASTER Record ({})", xform, num_xform),
                                context.clone()
                            )
                        );
                    }
                    if num_coord != pdb.total_atom_count() {
                        errors.push(
                            PDBError::new(
                                ErrorLevel::StrictWarning,
                                "MASTER checksum failed",
                                &format!("The number of Atoms (Normal + Hetero) ({}) is different then posed in the MASTER Record ({})", pdb.total_atom_count(), num_coord),
                                context.clone()
                            )
                        );
                    }
                }
                _ => (),
            }
        } else {
            errors.push(lineresult.unwrap_err())
        }
    }
    if current_model.total_atom_count() > 0 {
        pdb.add_model(current_model);
    }
    errors.extend(validate(&pdb));

    for error in &errors {
        if error.fails(level) {
            return Err(errors);
        }
    }

    Ok((pdb, errors))
}

/// Lex a REMARK
/// ## Fails
/// It fails on incorrect numbers for the remark-type-number
fn lex_remark(linenumber: usize, line: String) -> Result<(LexItem, Vec<PDBError>), PDBError> {
    let mut errors = Vec::new();
    let number = match parse_number(
        Context::line(linenumber, &line, 7, 3),
        &line.chars().collect::<Vec<char>>()[7..10],
    ) {
        Ok(n) => n,
        Err(e) => {
            errors.push(e);
            0
        }
    };
    if !reference_tables::valid_remark_type_number(number) {
        errors.push(PDBError::new(
            ErrorLevel::StrictWarning,
            "Remark type number invalid",
            "The remark-type-number is not valid, see wwPDB v3.30 for all valid numbers.",
            Context::line(linenumber, &line, 7, 3),
        ));
    }
    Ok((
        LexItem::Remark(
            number,
            if line.len() > 11 {
                if line.len() - 11 > 70 {
                    return Err(PDBError::new(
                        ErrorLevel::LooseWarning,
                        "Remark too long",
                        "The REMARK is too long, the max is 70 characters.",
                        Context::line(linenumber, &line, 11, line.len() - 11),
                    ));
                }
                line[11..].to_string()
            } else {
                "".to_string()
            },
        ),
        errors,
    ))
}

/// Lex a MODEL
/// ## Fails
/// It fails on incorrect numbers for the serial number
fn lex_model(linenumber: usize, line: String) -> Result<(LexItem, Vec<PDBError>), PDBError> {
    let mut errors = Vec::new();
    let number = match parse_number(
        Context::line(linenumber, &line, 6, line.len() - 6),
        &line.chars().collect::<Vec<char>>()[6..]
            .iter()
            .collect::<String>()
            .trim()
            .chars()
            .collect::<Vec<char>>()[..],
    ) {
        Ok(n) => n,
        Err(e) => {
            errors.push(e);
            0
        }
    };
    Ok((LexItem::Model(number), errors))
}

/// Lex an ATOM
/// ## Fails
/// It fails on incorrect numbers in the line
fn lex_atom(
    linenumber: usize,
    line: String,
    hetero: bool,
) -> Result<(LexItem, Vec<PDBError>), PDBError> {
    let mut errors = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    if chars.len() < 54 {
        return Err(PDBError::new(
            ErrorLevel::BreakingError,
            "Atom line too short",
            "This line is too short to contain all necessary elements (up to `z` at least).",
            Context::full_line(linenumber, &line),
        ));
    };
    let mut check = |item| match item {
        Ok(t) => t,
        Err(e) => {
            errors.push(e);
            0.0
        }
    };
    let x = check(parse_number(
        Context::line(linenumber, &line, 30, 8),
        &chars[30..38],
    ));
    let y = check(parse_number(
        Context::line(linenumber, &line, 38, 8),
        &chars[38..46],
    ));
    let z = check(parse_number(
        Context::line(linenumber, &line, 46, 8),
        &chars[46..54],
    ));
    let mut occupancy = 1.0;
    if chars.len() >= 60 {
        occupancy = check(parse_number(
            Context::line(linenumber, &line, 54, 6),
            &chars[54..60],
        ));
    }
    let mut b_factor = 0.0;
    if chars.len() >= 66 {
        b_factor = check(parse_number(
            Context::line(linenumber, &line, 60, 6),
            &chars[60..66],
        ));
    }

    let (
        (
            serial_number,
            atom_name,
            alternate_location,
            residue_name,
            chain_id,
            residue_serial_number,
            insertion,
            segment_id,
            element,
            charge,
        ),
        basic_errors,
    ) = lex_atom_basics(linenumber, line);
    errors.extend(basic_errors);

    Ok((
        LexItem::Atom(
            hetero,
            serial_number,
            atom_name,
            alternate_location,
            residue_name,
            chain_id,
            residue_serial_number,
            insertion,
            x,
            y,
            z,
            occupancy,
            b_factor,
            segment_id,
            element,
            charge,
        ),
        errors,
    ))
}

/// Lex an ANISOU
/// ## Fails
/// It fails on incorrect numbers in the line
fn lex_anisou(linenumber: usize, line: String) -> Result<(LexItem, Vec<PDBError>), PDBError> {
    let mut errors = Vec::new();
    let mut check = |item| match item {
        Ok(t) => t,
        Err(e) => {
            errors.push(e);
            0
        }
    };
    let chars: Vec<char> = line.chars().collect();
    let ai: isize = check(parse_number(
        Context::line(linenumber, &line, 28, 7),
        &chars[28..35],
    ));
    let bi: isize = check(parse_number(
        Context::line(linenumber, &line, 35, 7),
        &chars[35..42],
    ));
    let ci: isize = check(parse_number(
        Context::line(linenumber, &line, 42, 7),
        &chars[42..49],
    ));
    let di: isize = check(parse_number(
        Context::line(linenumber, &line, 49, 7),
        &chars[49..56],
    ));
    let ei: isize = check(parse_number(
        Context::line(linenumber, &line, 56, 7),
        &chars[56..63],
    ));
    let fi: isize = check(parse_number(
        Context::line(linenumber, &line, 63, 7),
        &chars[63..70],
    ));
    let factors = [
        [
            (ai as f64) / 10000.0,
            (bi as f64) / 10000.0,
            (ci as f64) / 10000.0,
        ],
        [
            (di as f64) / 10000.0,
            (ei as f64) / 10000.0,
            (fi as f64) / 10000.0,
        ],
    ];

    let (
        (
            serial_number,
            atom_name,
            alternate_location,
            residue_name,
            chain_id,
            residue_serial_number,
            insertion,
            segment_id,
            element,
            charge,
        ),
        basic_errors,
    ) = lex_atom_basics(linenumber, line);
    errors.extend(basic_errors);

    Ok((
        LexItem::Anisou(
            serial_number,
            atom_name,
            alternate_location,
            residue_name,
            chain_id,
            residue_serial_number,
            insertion,
            factors,
            segment_id,
            element,
            charge,
        ),
        errors,
    ))
}

/// Lex the basic structure of the ATOM/HETATM/ANISOU Records, to minimise code duplication
#[allow(clippy::type_complexity)]
fn lex_atom_basics(
    linenumber: usize,
    line: String,
) -> (
    (
        usize,
        [char; 4],
        char,
        [char; 3],
        char,
        usize,
        char,
        [char; 4],
        [char; 2],
        isize,
    ),
    Vec<PDBError>,
) {
    let mut errors = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut check_usize = |item| match item {
        Ok(t) => t,
        Err(e) => {
            errors.push(e);
            0
        }
    };
    let serial_number = check_usize(parse_number(
        Context::line(linenumber, &line, 7, 4),
        &chars[7..11],
    ));
    let atom_name = [chars[12], chars[13], chars[14], chars[15]];
    let alternate_location = chars[16];
    let residue_name = [chars[17], chars[18], chars[19]];
    let chain_id = chars[21];
    let residue_serial_number = check_usize(parse_number(
        Context::line(linenumber, &line, 22, 4),
        &chars[22..26],
    ));
    let insertion = chars[26];
    let mut segment_id = [' ', ' ', ' ', ' '];
    if chars.len() >= 75 {
        segment_id = [chars[72], chars[73], chars[74], chars[75]];
    }
    let mut element = [' ', ' '];
    if chars.len() >= 77 {
        element = [chars[76], chars[77]];
    }
    let mut charge = 0;
    if chars.len() >= 79 && !(chars[78] == ' ' && chars[79] == ' ') {
        if !chars[78].is_ascii_digit() {
            errors.push(PDBError::new(
                ErrorLevel::InvalidatingError,
                "Atom charge is not correct",
                "The charge is not numeric, it is defined to be [0-9][+-], so two characters in total.",
                Context::line(linenumber, &line, 78, 1),
            ));
        } else if chars[79] != '-' && chars[79] != '+' {
            errors.push(PDBError::new(
                ErrorLevel::InvalidatingError,
                "Atom charge is not correct",
                "The charge is not properly signed, it is defined to be [0-9][+-], so two characters in total.",
                Context::line(linenumber, &line, 79, 1),
            ));
        } else {
            charge = chars[78].to_digit(10).unwrap() as isize;
            if chars[79] == '-' {
                charge *= -1;
            }
        }
    }

    (
        (
            serial_number,
            atom_name,
            alternate_location,
            residue_name,
            chain_id,
            residue_serial_number,
            insertion,
            segment_id,
            element,
            charge,
        ),
        errors,
    )
}

/// Lex a CRYST1
/// ## Fails
/// It fails on incorrect numbers in the line
fn lex_cryst(linenumber: usize, line: String) -> Result<(LexItem, Vec<PDBError>), PDBError> {
    let mut errors = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut check = |item| match item {
        Ok(t) => t,
        Err(e) => {
            errors.push(e);
            0.0
        }
    };
    let a = check(parse_number(
        Context::line(linenumber, &line, 6, 9),
        &chars[6..15],
    ));
    let b = check(parse_number(
        Context::line(linenumber, &line, 15, 9),
        &chars[15..24],
    ));
    let c = check(parse_number(
        Context::line(linenumber, &line, 24, 9),
        &chars[24..33],
    ));
    let alpha = check(parse_number(
        Context::line(linenumber, &line, 33, 7),
        &chars[33..40],
    ));
    let beta = check(parse_number(
        Context::line(linenumber, &line, 40, 7),
        &chars[40..47],
    ));
    let gamma = check(parse_number(
        Context::line(linenumber, &line, 47, 7),
        &chars[47..54],
    ));
    let spacegroup = chars[55..std::cmp::min(66, chars.len())]
        .iter()
        .collect::<String>();
    let mut z = 1;
    if chars.len() > 66 {
        z = match parse_number(
            Context::line(linenumber, &line, 66, line.len() - 66),
            &chars[66..],
        ) {
            Ok(value) => value,
            Err(error) => {
                errors.push(error);
                0
            }
        };
    }

    Ok((
        LexItem::Crystal(a, b, c, alpha, beta, gamma, spacegroup, z),
        errors,
    ))
}

/// Lex an SCALEn (where `n` is given)
/// ## Fails
/// It fails on incorrect numbers in the line
fn lex_scale(
    linenumber: usize,
    line: String,
    row: usize,
) -> Result<(LexItem, Vec<PDBError>), PDBError> {
    let (data, errors) = lex_transformation(linenumber, line);

    Ok((LexItem::Scale(row, data), errors))
}

/// Lex an ORIGXn (where `n` is given)
/// ## Fails
/// It fails on incorrect numbers in the line
fn lex_origx(
    linenumber: usize,
    line: String,
    row: usize,
) -> Result<(LexItem, Vec<PDBError>), PDBError> {
    let (data, errors) = lex_transformation(linenumber, line);

    Ok((LexItem::OrigX(row, data), errors))
}

/// Lex an MTRIXn (where `n` is given)
/// ## Fails
/// It fails on incorrect numbers in the line
fn lex_mtrix(
    linenumber: usize,
    line: String,
    row: usize,
) -> Result<(LexItem, Vec<PDBError>), PDBError> {
    let mut errors = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut check = |item| match item {
        Ok(t) => t,
        Err(e) => {
            errors.push(e);
            0
        }
    };
    let ser = check(parse_number(
        Context::line(linenumber, &line, 7, 4),
        &chars[7..10],
    ));
    let (data, transformation_errors) = lex_transformation(linenumber, line);
    errors.extend(transformation_errors);

    let mut given = false;
    if chars.len() >= 60 {
        given = chars[59] == '1';
    }

    Ok((LexItem::MtriX(row, ser, data, given), errors))
}

fn lex_transformation(linenumber: usize, line: String) -> ([f64; 4], Vec<PDBError>) {
    let mut errors = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut check = |item| match item {
        Ok(t) => t,
        Err(e) => {
            errors.push(e);
            0.0
        }
    };
    let a = check(parse_number(
        Context::line(linenumber, &line, 10, 10),
        &chars[10..20],
    ));
    let b = check(parse_number(
        Context::line(linenumber, &line, 20, 10),
        &chars[20..30],
    ));
    let c = check(parse_number(
        Context::line(linenumber, &line, 30, 10),
        &chars[30..40],
    ));
    let d = check(parse_number(
        Context::line(linenumber, &line, 45, 10),
        &chars[45..55],
    ));

    ([a, b, c, d], errors)
}

/// Lex a MASTER
/// ## Fails
/// It fails on incorrect numbers in the line
fn lex_master(linenumber: usize, line: String) -> Result<(LexItem, Vec<PDBError>), PDBError> {
    let mut errors = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut check = |item| match item {
        Ok(t) => t,
        Err(e) => {
            errors.push(e);
            0
        }
    };
    let num_remark = check(parse_number(
        Context::line(linenumber, &line, 10, 5),
        &chars[10..15],
    ));
    let num_empty = check(parse_number(
        Context::line(linenumber, &line, 15, 5),
        &chars[15..20],
    ));
    let num_het = check(parse_number(
        Context::line(linenumber, &line, 20, 5),
        &chars[20..25],
    ));
    let num_helix = check(parse_number(
        Context::line(linenumber, &line, 25, 5),
        &chars[25..30],
    ));
    let num_sheet = check(parse_number(
        Context::line(linenumber, &line, 30, 5),
        &chars[30..35],
    ));
    let num_turn = check(parse_number(
        Context::line(linenumber, &line, 35, 5),
        &chars[35..40],
    ));
    let num_site = check(parse_number(
        Context::line(linenumber, &line, 40, 5),
        &chars[40..45],
    ));
    let num_xform = check(parse_number(
        Context::line(linenumber, &line, 45, 5),
        &chars[45..50],
    ));
    let num_coord = check(parse_number(
        Context::line(linenumber, &line, 50, 5),
        &chars[50..55],
    ));
    let num_ter = check(parse_number(
        Context::line(linenumber, &line, 55, 5),
        &chars[55..60],
    ));
    let num_connect = check(parse_number(
        Context::line(linenumber, &line, 60, 5),
        &chars[60..65],
    ));
    let num_seq = check(parse_number(
        Context::line(linenumber, &line, 65, 5),
        &chars[65..70],
    ));

    Ok((
        LexItem::Master(
            num_remark,
            num_empty,
            num_het,
            num_helix,
            num_sheet,
            num_turn,
            num_site,
            num_xform,
            num_coord,
            num_ter,
            num_connect,
            num_seq,
        ),
        errors,
    ))
}

/// Parse a number, generic for anything that can be parsed using FromStr
fn parse_number<T: FromStr>(context: Context, input: &[char]) -> Result<T, PDBError> {
    let string = input
        .iter()
        .collect::<String>()
        .split_whitespace()
        .collect::<String>();
    match string.parse::<T>() {
        Ok(v) => Ok(v),
        Err(_) => Err(PDBError::new(
            ErrorLevel::InvalidatingError,
            "Not a number",
            "The text presented is not a number of the right kind.",
            context,
        )),
    }
}
