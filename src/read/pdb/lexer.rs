use super::lexitem::*;
use crate::error::*;
use crate::reference_tables;

use std::cmp;
use std::convert::TryFrom;
use std::str::FromStr;

/// Lex a full line. It returns a lexed item with errors if it can lex something, otherwise it will only return an error.
pub fn lex_line(line: &str, linenumber: usize) -> Result<(LexItem, Vec<PDBError>), PDBError> {
    if line.len() > 6 {
        match &line[..6] {
            "HEADER" => lex_header(linenumber, line),
            "REMARK" => lex_remark(linenumber, line),
            "ATOM  " => lex_atom(linenumber, line, false),
            "ANISOU" => Ok(lex_anisou(linenumber, line)),
            "HETATM" => lex_atom(linenumber, line, true),
            "CRYST1" => Ok(lex_cryst(linenumber, line)),
            "SCALE1" => Ok(lex_scale(linenumber, line, 0)),
            "SCALE2" => Ok(lex_scale(linenumber, line, 1)),
            "SCALE3" => Ok(lex_scale(linenumber, line, 2)),
            "ORIGX1" => Ok(lex_origx(linenumber, line, 0)),
            "ORIGX2" => Ok(lex_origx(linenumber, line, 1)),
            "ORIGX3" => Ok(lex_origx(linenumber, line, 2)),
            "MTRIX1" => Ok(lex_mtrix(linenumber, line, 0)),
            "MTRIX2" => Ok(lex_mtrix(linenumber, line, 1)),
            "MTRIX3" => Ok(lex_mtrix(linenumber, line, 2)),
            "MODEL " => Ok(lex_model(linenumber, line)),
            "MASTER" => Ok(lex_master(linenumber, line)),
            "DBREF " => Ok(lex_dbref(linenumber, line)),
            "DBREF1" => Ok(lex_dbref1(linenumber, line)),
            "DBREF2" => Ok(lex_dbref2(linenumber, line)),
            "SEQRES" => Ok(lex_seqres(linenumber, line)),
            "SEQADV" => Ok(lex_seqadv(linenumber, line)),
            "MODRES" => Ok(lex_modres(linenumber, line)),
            "SSBOND" => Ok(lex_ssbond(linenumber, line)),
            "ENDMDL" => Ok((LexItem::EndModel(), Vec::new())),
            "TER   " => Ok((LexItem::TER(), Vec::new())),
            "END   " => Ok((LexItem::End(), Vec::new())),
            _ => Ok((LexItem::Empty(), Vec::new())),
        }
    } else if line.len() > 2 {
        match &line[..3] {
            "TER" => Ok((LexItem::TER(), Vec::new())),
            "END" => Ok((LexItem::End(), Vec::new())),
            _ => Ok((LexItem::Empty(), Vec::new())),
        }
    } else {
        Ok((LexItem::Empty(), Vec::new()))
    }
}

/// Lex a REMARK
/// ## Fails
/// It fails on incorrect numbers for the remark-type-number
fn lex_remark(linenumber: usize, line: &str) -> Result<(LexItem, Vec<PDBError>), PDBError> {
    let mut errors = Vec::new();
    let number = match parse_number(
        Context::line(linenumber, line, 7, 3),
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
            Context::line(linenumber, line, 7, 3),
        ));
    }
    Ok((
        LexItem::Remark(
            number,
            if line.len() > 11 {
                if line.len() - 11 > 70 {
                    return Err(PDBError::new(
                        ErrorLevel::GeneralWarning,
                        "Remark too long",
                        "The REMARK is too long, the max is 70 characters.",
                        Context::line(linenumber, line, 11, line.len() - 11),
                    ));
                }
                line[11..].trim_end().to_string()
            } else {
                "".to_string()
            },
        ),
        errors,
    ))
}

/// Lex a HEADER
/// ## Fails
/// Fails if the header is too short (below 66 lines)
fn lex_header(linenumber: usize, line: &str) -> Result<(LexItem, Vec<PDBError>), PDBError> {
    if line.len() < 66 {
        Err(PDBError::new(
            ErrorLevel::LooseWarning,
            "Header too short",
            "The HEADER is too short, the min is 66 characters.",
            Context::line(linenumber, line, 11, line.len() - 11),
        ))
    } else {
        Ok((
            LexItem::Header(
                line.chars().collect::<Vec<char>>()[10..50]
                    .iter()
                    .collect::<String>(),
                line.chars().collect::<Vec<char>>()[50..59]
                    .iter()
                    .collect::<String>(),
                line.chars().collect::<Vec<char>>()[62..66]
                    .iter()
                    .collect::<String>(),
            ),
            Vec::new(),
        ))
    }
}

/// Lex a MODEL
/// ## Fails
/// It fails on incorrect numbers for the serial number
fn lex_model(linenumber: usize, line: &str) -> (LexItem, Vec<PDBError>) {
    let mut errors = Vec::new();
    let number = match parse_number(
        Context::line(linenumber, line, 6, line.len() - 6),
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
    (LexItem::Model(number), errors)
}

/// Lex an ATOM
/// ## Fails
/// It fails on incorrect numbers in the line
fn lex_atom(
    linenumber: usize,
    line: &str,
    hetero: bool,
) -> Result<(LexItem, Vec<PDBError>), PDBError> {
    let mut errors = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    if chars.len() < 54 {
        return Err(PDBError::new(
            ErrorLevel::BreakingError,
            "Atom line too short",
            "This line is too short to contain all necessary elements (up to `z` at least).",
            Context::full_line(linenumber, line),
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
        Context::line(linenumber, line, 30, 8),
        &chars[30..38],
    ));
    let y = check(parse_number(
        Context::line(linenumber, line, 38, 8),
        &chars[38..46],
    ));
    let z = check(parse_number(
        Context::line(linenumber, line, 46, 8),
        &chars[46..54],
    ));
    let mut occupancy = 1.0;
    if chars.len() >= 60 {
        occupancy = check(parse_number(
            Context::line(linenumber, line, 54, 6),
            &chars[54..60],
        ));
    }
    let mut b_factor = 0.0;
    if chars.len() >= 66 {
        b_factor = check(parse_number(
            Context::line(linenumber, line, 60, 6),
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
fn lex_anisou(linenumber: usize, line: &str) -> (LexItem, Vec<PDBError>) {
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
        Context::line(linenumber, line, 28, 7),
        &chars[28..35],
    ));
    let bi: isize = check(parse_number(
        Context::line(linenumber, line, 35, 7),
        &chars[35..42],
    ));
    let ci: isize = check(parse_number(
        Context::line(linenumber, line, 42, 7),
        &chars[42..49],
    ));
    let di: isize = check(parse_number(
        Context::line(linenumber, line, 49, 7),
        &chars[49..56],
    ));
    let ei: isize = check(parse_number(
        Context::line(linenumber, line, 56, 7),
        &chars[56..63],
    ));
    let fi: isize = check(parse_number(
        Context::line(linenumber, line, 63, 7),
        &chars[63..70],
    ));
    #[allow(clippy::cast_precision_loss)]
    let factors = [
        [
            (ai as f64) / 10000.0,
            (di as f64) / 10000.0,
            (ei as f64) / 10000.0,
        ],
        [
            (di as f64) / 10000.0,
            (bi as f64) / 10000.0,
            (fi as f64) / 10000.0,
        ],
        [
            (ei as f64) / 10000.0,
            (fi as f64) / 10000.0,
            (ci as f64) / 10000.0,
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

    (
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
    )
}

/// Lex the basic structure of the ATOM/HETATM/ANISOU Records, to minimise code duplication
#[allow(clippy::type_complexity)]
fn lex_atom_basics(
    linenumber: usize,
    line: &str,
) -> (
    (
        usize,
        String,
        Option<String>,
        String,
        String,
        isize,
        Option<String>,
        String,
        String,
        isize,
    ),
    Vec<PDBError>,
) {
    let mut errors = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut check = |item: Result<usize, PDBError>| match item {
        Ok(t) => t,
        Err(e) => {
            errors.push(e);
            0
        }
    };
    let serial_number = check(parse_number(
        Context::line(linenumber, line, 6, 5),
        &chars[6..11],
    ));
    let atom_name = chars[12..16].iter().collect::<String>();
    let alternate_location = chars[16];
    let residue_name = chars[17..20].iter().collect::<String>();
    let chain_id = String::from(chars[21]);
    let residue_serial_number =
        parse_number(Context::line(linenumber, line, 22, 4), &chars[22..26]).unwrap_or_else(|e| {
            errors.push(e);
            0
        });
    let insertion = chars[26];
    let mut segment_id = String::new();
    if chars.len() >= 75 {
        segment_id = chars[72..76].iter().collect::<String>();
    }
    let mut element = String::new();
    if chars.len() >= 77 {
        element = chars[76..78].iter().collect::<String>();
    }
    let mut charge = 0;
    #[allow(clippy::unwrap_used)]
    if chars.len() >= 80 && !(chars[78] == ' ' && chars[79] == ' ') {
        if !chars[78].is_ascii_digit() {
            errors.push(PDBError::new(
                ErrorLevel::InvalidatingError,
                "Atom charge is not correct",
                "The charge is not numeric, it is defined to be [0-9][+-], so two characters in total.",
                Context::line(linenumber, line, 78, 1),
            ));
        } else if chars[79] != '-' && chars[79] != '+' {
            errors.push(PDBError::new(
                ErrorLevel::InvalidatingError,
                "Atom charge is not correct",
                "The charge is not properly signed, it is defined to be [0-9][+-], so two characters in total.",
                Context::line(linenumber, line, 79, 1),
            ));
        } else {
            charge = isize::try_from(chars[78].to_digit(10).unwrap()).unwrap();
            if chars[79] == '-' {
                charge *= -1;
            }
        }
    }

    (
        (
            serial_number,
            atom_name,
            if alternate_location == ' ' {
                None
            } else {
                Some(String::from(alternate_location))
            },
            residue_name,
            chain_id,
            residue_serial_number,
            if insertion == ' ' {
                None
            } else {
                Some(String::from(insertion))
            },
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
fn lex_cryst(linenumber: usize, line: &str) -> (LexItem, Vec<PDBError>) {
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
        Context::line(linenumber, line, 6, 9),
        &chars[6..15],
    ));
    let b = check(parse_number(
        Context::line(linenumber, line, 15, 9),
        &chars[15..24],
    ));
    let c = check(parse_number(
        Context::line(linenumber, line, 24, 9),
        &chars[24..33],
    ));
    let alpha = check(parse_number(
        Context::line(linenumber, line, 33, 7),
        &chars[33..40],
    ));
    let beta = check(parse_number(
        Context::line(linenumber, line, 40, 7),
        &chars[40..47],
    ));
    let gamma = check(parse_number(
        Context::line(linenumber, line, 47, 7),
        &chars[47..54],
    ));
    let spacegroup = chars[55..std::cmp::min(66, chars.len())]
        .iter()
        .collect::<String>();
    let mut z = 1;
    if chars.len() > 66 {
        z = match parse_number(
            Context::line(linenumber, line, 66, line.len() - 66),
            &chars[66..],
        ) {
            Ok(value) => value,
            Err(error) => {
                errors.push(error);
                0
            }
        };
    }

    (
        LexItem::Crystal(a, b, c, alpha, beta, gamma, spacegroup, z),
        errors,
    )
}

/// Lex an SCALEn (where `n` is given)
/// ## Fails
/// It fails on incorrect numbers in the line
fn lex_scale(linenumber: usize, line: &str, row: usize) -> (LexItem, Vec<PDBError>) {
    let (data, errors) = lex_transformation(linenumber, line);

    (LexItem::Scale(row, data), errors)
}

/// Lex an ORIGXn (where `n` is given)
/// ## Fails
/// It fails on incorrect numbers in the line
fn lex_origx(linenumber: usize, line: &str, row: usize) -> (LexItem, Vec<PDBError>) {
    let (data, errors) = lex_transformation(linenumber, line);

    (LexItem::OrigX(row, data), errors)
}

/// Lex an MTRIXn (where `n` is given)
/// ## Fails
/// It fails on incorrect numbers in the line
fn lex_mtrix(linenumber: usize, line: &str, row: usize) -> (LexItem, Vec<PDBError>) {
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
        Context::line(linenumber, line, 7, 4),
        &chars[7..10],
    ));
    let (data, transformation_errors) = lex_transformation(linenumber, line);
    errors.extend(transformation_errors);

    let mut given = false;
    if chars.len() >= 60 {
        given = chars[59] == '1';
    }

    (LexItem::MtriX(row, ser, data, given), errors)
}

/// Lexes the general structure of a transformation record (ORIGXn, SCALEn, MTRIXn)
fn lex_transformation(linenumber: usize, line: &str) -> ([f64; 4], Vec<PDBError>) {
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
        Context::line(linenumber, line, 10, 10),
        &chars[10..20],
    ));
    let b = check(parse_number(
        Context::line(linenumber, line, 20, 10),
        &chars[20..30],
    ));
    let c = check(parse_number(
        Context::line(linenumber, line, 30, 10),
        &chars[30..40],
    ));
    let d = check(parse_number(
        Context::line(linenumber, line, 45, 10),
        &chars[45..55],
    ));

    ([a, b, c, d], errors)
}

/// Lex a MASTER
/// ## Fails
/// It fails on incorrect numbers in the line
fn lex_master(linenumber: usize, line: &str) -> (LexItem, Vec<PDBError>) {
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
        Context::line(linenumber, line, 10, 5),
        &chars[10..15],
    ));
    let num_empty = check(parse_number(
        Context::line(linenumber, line, 15, 5),
        &chars[15..20],
    ));
    let num_het = check(parse_number(
        Context::line(linenumber, line, 20, 5),
        &chars[20..25],
    ));
    let num_helix = check(parse_number(
        Context::line(linenumber, line, 25, 5),
        &chars[25..30],
    ));
    let num_sheet = check(parse_number(
        Context::line(linenumber, line, 30, 5),
        &chars[30..35],
    ));
    let num_turn = check(parse_number(
        Context::line(linenumber, line, 35, 5),
        &chars[35..40],
    ));
    let num_site = check(parse_number(
        Context::line(linenumber, line, 40, 5),
        &chars[40..45],
    ));
    let num_xform = check(parse_number(
        Context::line(linenumber, line, 45, 5),
        &chars[45..50],
    ));
    let num_coord = check(parse_number(
        Context::line(linenumber, line, 50, 5),
        &chars[50..55],
    ));
    let num_ter = check(parse_number(
        Context::line(linenumber, line, 55, 5),
        &chars[55..60],
    ));
    let num_connect = check(parse_number(
        Context::line(linenumber, line, 60, 5),
        &chars[60..65],
    ));
    let num_seq = check(parse_number(
        Context::line(linenumber, line, 65, 5),
        &chars[65..70],
    ));

    (
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
    )
}

/// Lexes a SEQRES record
fn lex_seqres(linenumber: usize, line: &str) -> (LexItem, Vec<PDBError>) {
    let mut errors = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut check = |item| match item {
        Ok(t) => t,
        Err(e) => {
            errors.push(e);
            0
        }
    };
    let ser_num = check(parse_number(
        Context::line(linenumber, line, 7, 3),
        &chars[7..10],
    ));
    let chain_id = chars[11];
    let num_res = check(parse_number(
        Context::line(linenumber, line, 13, 4),
        &chars[13..17],
    ));
    let mut values = Vec::new();
    let mut index = 19;
    let max = cmp::min(chars.len(), 71);
    while index + 3 <= max {
        let seq = chars[index..index + 3].iter().collect::<String>();
        if seq == "   " {
            break;
        }
        values.push(seq);
        index += 4;
    }
    (
        LexItem::Seqres(ser_num, String::from(chain_id), num_res, values),
        errors,
    )
}

/// Lexes a DBREF record
fn lex_dbref(linenumber: usize, line: &str) -> (LexItem, Vec<PDBError>) {
    let mut errors = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut check = |item| match item {
        Ok(t) => t,
        Err(e) => {
            errors.push(e);
            0
        }
    };
    let id_code = [chars[7], chars[8], chars[9], chars[10]];
    let chain_id = chars[12];
    let seq_begin = check(parse_number(
        Context::line(linenumber, line, 14, 4),
        &chars[14..18],
    ));
    let insert_begin = chars[18];
    let seq_end = check(parse_number(
        Context::line(linenumber, line, 21, 4),
        &chars[20..24],
    ));
    let insert_end = chars[24];
    let database = chars[26..32].iter().collect::<String>().trim().to_string();
    let database_accession = chars[33..41].iter().collect::<String>().trim().to_string();
    let database_id_code = chars[42..54].iter().collect::<String>().trim().to_string();
    let database_seq_begin = check(parse_number(
        Context::line(linenumber, line, 55, 5),
        &chars[55..60],
    ));
    let database_insert_begin = chars[60];
    let database_seq_end = check(parse_number(
        Context::line(linenumber, line, 62, 5),
        &chars[62..67],
    ));
    let database_insert_end = chars[67];

    (
        LexItem::Dbref(
            id_code,
            String::from(chain_id),
            (seq_begin, insert_begin, seq_end, insert_end),
            database,
            database_accession,
            database_id_code,
            (
                database_seq_begin,
                database_insert_begin,
                database_seq_end,
                database_insert_end,
            ),
        ),
        errors,
    )
}

/// Lexes a DBREF1 record
fn lex_dbref1(linenumber: usize, line: &str) -> (LexItem, Vec<PDBError>) {
    let mut errors = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut check = |item| match item {
        Ok(t) => t,
        Err(e) => {
            errors.push(e);
            0
        }
    };
    let id_code = [chars[7], chars[8], chars[9], chars[10]];
    let chain_id = chars[12];
    let seq_begin = check(parse_number(
        Context::line(linenumber, line, 14, 4),
        &chars[14..18],
    ));
    let insert_begin = chars[18];
    let seq_end = check(parse_number(
        Context::line(linenumber, line, 21, 4),
        &chars[21..24],
    ));
    let insert_end = chars[24];
    let database = chars[26..32].iter().collect::<String>().trim().to_string();
    let database_id_code = chars[47..67].iter().collect::<String>().trim().to_string();

    (
        LexItem::Dbref1(
            id_code,
            String::from(chain_id),
            (seq_begin, insert_begin, seq_end, insert_end),
            database,
            database_id_code,
        ),
        errors,
    )
}

/// Lexes a DBREF2 record
fn lex_dbref2(linenumber: usize, line: &str) -> (LexItem, Vec<PDBError>) {
    let mut errors = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut check = |item| match item {
        Ok(t) => t,
        Err(e) => {
            errors.push(e);
            0
        }
    };
    let id_code = [chars[7], chars[8], chars[9], chars[10]];
    let chain_id = chars[12];
    let database_accession = chars[18..40].iter().collect::<String>().trim().to_string();
    let database_seq_begin = check(parse_number(
        Context::line(linenumber, line, 55, 5),
        &chars[45..55],
    ));
    let database_seq_end = check(parse_number(
        Context::line(linenumber, line, 62, 5),
        &chars[57..67],
    ));

    (
        LexItem::Dbref2(
            id_code,
            String::from(chain_id),
            database_accession,
            database_seq_begin,
            database_seq_end,
        ),
        errors,
    )
}

/// Lexes a SEQADV record
fn lex_seqadv(linenumber: usize, line: &str) -> (LexItem, Vec<PDBError>) {
    let mut errors = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut check = |item| match item {
        Ok(t) => t,
        Err(e) => {
            errors.push(e);
            0
        }
    };
    let id_code = [chars[7], chars[8], chars[9], chars[10]];
    let res_name = chars[12..15].iter().collect::<String>().trim().to_string();
    let chain_id = chars[16];
    let seq_num = check(parse_number(
        Context::line(linenumber, line, 18, 4),
        &chars[18..22],
    ));
    let insert = chars[22];
    let database = chars[24..28].iter().collect::<String>().trim().to_string();
    let database_accession = chars[29..38].iter().collect::<String>().trim().to_string();

    let mut db_pos = None;
    if !chars[39..48].iter().all(|c| *c == ' ') {
        let db_res_name = chars[39..42].iter().collect::<String>().trim().to_string();
        let db_seq_num = check(parse_number(
            Context::line(linenumber, line, 43, 5),
            &chars[43..48],
        ));
        db_pos = Some((db_res_name, db_seq_num));
    }
    let comment = chars[49..].iter().collect::<String>().trim().to_string();

    (
        LexItem::Seqadv(
            id_code,
            String::from(chain_id),
            res_name,
            seq_num,
            if insert == ' ' {
                None
            } else {
                Some(String::from(insert))
            },
            database,
            database_accession,
            db_pos,
            comment,
        ),
        errors,
    )
}

/// Lexes a MODRES record
fn lex_modres(linenumber: usize, line: &str) -> (LexItem, Vec<PDBError>) {
    let mut errors = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut check = |item| match item {
        Ok(t) => t,
        Err(e) => {
            errors.push(e);
            0
        }
    };
    let id = [chars[7], chars[8], chars[9], chars[10]];
    let res_name = chars[12..15].iter().collect::<String>();
    let chain_id = chars[16];
    let seq_num = check(parse_number(
        Context::line(linenumber, line, 18, 4),
        &chars[18..22],
    ));
    let insert = chars[22];
    let std_res = chars[24..27].iter().collect::<String>();
    let comment = chars[29..].iter().collect::<String>().trim().to_string();

    (
        LexItem::Modres(
            id,
            res_name,
            String::from(chain_id),
            seq_num,
            if insert == ' ' {
                None
            } else {
                Some(String::from(insert))
            },
            std_res,
            comment,
        ),
        errors,
    )
}

/// Parse a SSBond line into the corresponding LexItem
fn lex_ssbond(linenumber: usize, line: &str) -> (LexItem, Vec<PDBError>) {
    let mut errors = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    // The Serial number field is ignored
    let res_1 = chars[11..14].iter().collect::<String>();
    let chain_1 = chars[15];
    let res_seq_1: isize = parse_number(Context::line(linenumber, line, 17, 4), &chars[17..21])
        .unwrap_or_else(|err| {
            errors.push(err);
            0
        });
    let icode_1 = if chars[21] == ' ' {
        None
    } else {
        Some(chars[21].to_string())
    };
    let res_2 = chars[25..28].iter().collect::<String>();
    let chain_2 = chars[29];
    let res_seq_2 = parse_number(Context::line(linenumber, line, 31, 4), &chars[31..35])
        .unwrap_or_else(|err| {
            errors.push(err);
            0
        });
    let icode_2 = if chars[35] == ' ' {
        None
    } else {
        Some(chars[35].to_string())
    };

    let mut extra = None;

    if chars.len() >= 78 {
        let sym1 = chars[59..65].iter().collect::<String>();
        let sym2 = chars[66..72].iter().collect::<String>();
        let distance: f64 = parse_number(Context::line(linenumber, line, 73, 5), &chars[73..78])
            .unwrap_or_else(|err| {
                errors.push(err);
                0.0
            });
        extra = Some((sym1, sym2, distance));
    }

    (
        LexItem::SSBond(
            (res_1, res_seq_1, icode_1, chain_1.to_string()),
            (res_2, res_seq_2, icode_2, chain_2.to_string()),
            extra,
        ),
        errors,
    )
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
