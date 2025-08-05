use super::lexitem::*;
use super::utils::fast_parse_u64_from_string;
use super::utils::fast_trim;
use crate::error::*;
use crate::reference_tables;
use crate::ReadOptions;
use crate::StrictnessLevel;

use std::cmp;

use std::ops::Range;
use std::str::FromStr;

/// Lex a full line. It returns a lexed item with errors if it can lex something, otherwise it will only return an error.
pub(crate) fn lex_line(
    line: &str,
    linenumber: usize,
    options: &ReadOptions,
) -> Result<(LexItem, Vec<PDBError>), PDBError> {
    match line.len() {
        len if len > 6 => match (options.only_atomic_coords, &line[..6]) {
            (_, "HETATM") => Ok(lex_atom(linenumber, line, true)),
            (_, "ATOM  ") => Ok(lex_atom(linenumber, line, false)),
            (false, "HEADER") => lex_header(linenumber, line),
            (false, "REMARK") => lex_remark(linenumber, line, options.level),
            (false, "ANISOU") => Ok(lex_anisou(linenumber, line)),
            (false, "CRYST1") => Ok(lex_cryst(linenumber, line)),
            (false, "SCALE1") => Ok(lex_scale(linenumber, line, 0)),
            (false, "SCALE2") => Ok(lex_scale(linenumber, line, 1)),
            (false, "SCALE3") => Ok(lex_scale(linenumber, line, 2)),
            (false, "ORIGX1") => Ok(lex_origx(linenumber, line, 0)),
            (false, "ORIGX2") => Ok(lex_origx(linenumber, line, 1)),
            (false, "ORIGX3") => Ok(lex_origx(linenumber, line, 2)),
            (false, "MTRIX1") => Ok(lex_mtrix(linenumber, line, 0)),
            (false, "MTRIX2") => Ok(lex_mtrix(linenumber, line, 1)),
            (false, "MTRIX3") => Ok(lex_mtrix(linenumber, line, 2)),
            (_, "MODEL ") => Ok(lex_model(linenumber, line)),
            (false, "MASTER") => Ok(lex_master(linenumber, line)),
            (false, "DBREF ") => Ok(lex_dbref(linenumber, line)),
            (false, "DBREF1") => Ok(lex_dbref1(linenumber, line)),
            (false, "DBREF2") => Ok(lex_dbref2(linenumber, line)),
            (false, "SEQRES") => Ok(lex_seqres(linenumber, line)),
            (false, "SEQADV") => Ok(lex_seqadv(linenumber, line)),
            (false, "MODRES") => Ok(lex_modres(linenumber, line)),
            (false, "SSBOND") => Ok(lex_ssbond(linenumber, line)),
            (_, "ENDMDL") => Ok((LexItem::EndModel(), Vec::new())),
            (_, "TER   ") => Ok((LexItem::TER(), Vec::new())),
            (_, "END   ") => Ok((LexItem::End(), Vec::new())),
            (_, _) => Ok((LexItem::Empty(), Vec::new())),
        },
        len if len > 2 => match &line[..3] {
            "TER" => Ok((LexItem::TER(), Vec::new())),
            "END" => Ok((LexItem::End(), Vec::new())),
            _ => Ok((LexItem::Empty(), Vec::new())),
        },
        _ => Ok((LexItem::Empty(), Vec::new())),
    }
}

/// Lex a REMARK
/// ## Fails
/// It fails on incorrect numbers for the remark-type-number
fn lex_remark(
    linenumber: usize,
    line: &str,
    level: StrictnessLevel,
) -> Result<(LexItem, Vec<PDBError>), PDBError> {
    let mut errors = Vec::new();
    let number = parse(linenumber, line, 7..10, &mut errors);

    if !reference_tables::valid_remark_type_number(number) {
        errors.push(PDBError::new(
            ErrorLevel::LooseWarning,
            "Remark type number invalid",
            "The remark-type-number is not valid, see wwPDB v3.30 for all valid numbers.",
            Context::line(linenumber, line, 7, 3),
        ));
    }
    Ok((
        LexItem::Remark(
            number,
            if line.len() > 11 {
                if line.trim_end().len() >= 80 && level != StrictnessLevel::Loose {
                    return Err(PDBError::new(
                        ErrorLevel::GeneralWarning,
                        "Remark too long",
                        "The REMARK is too long, the max is 80 characters.",
                        Context::line(linenumber, line, 80, line.len() - 80),
                    ));
                }
                line[11..].trim_end().to_string()
            } else {
                String::new()
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
    let number = parse(linenumber, line, 6..line.len(), &mut errors);
    (LexItem::Model(number), errors)
}

/// Lex an ATOM
/// ## Fails
/// It fails on incorrect numbers in the line
fn lex_atom(linenumber: usize, line: &str, hetero: bool) -> (LexItem, Vec<PDBError>) {
    let mut errors = Vec::new();

    let x = parse(linenumber, line, 30..38, &mut errors);
    let y = parse(linenumber, line, 38..46, &mut errors);
    let z = parse(linenumber, line, 46..54, &mut errors);
    let occupancy = parse_default(linenumber, line, 54..60, &mut errors, 1.0);
    let b_factor = parse(linenumber, line, 60..66, &mut errors);

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
    )
}

/// Lex an ANISOU
/// ## Fails
/// It fails on incorrect numbers in the line
fn lex_anisou(linenumber: usize, line: &str) -> (LexItem, Vec<PDBError>) {
    let mut errors = Vec::new();

    let ai: isize = parse(linenumber, line, 28..35, &mut errors);
    let bi: isize = parse(linenumber, line, 35..42, &mut errors);
    let ci: isize = parse(linenumber, line, 42..49, &mut errors);
    let di: isize = parse(linenumber, line, 49..56, &mut errors);
    let ei: isize = parse(linenumber, line, 56..63, &mut errors);
    let fi: isize = parse(linenumber, line, 63..70, &mut errors);
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
    let chars: &[u8] = line.as_bytes();

    let serial_number = fast_parse_u64(linenumber, line, 6..11, &mut errors, 0);
    let atom_name = parse(linenumber, line, 12..16, &mut errors);
    let alternate_location = parse_char(linenumber, chars, 16, &mut errors);
    let residue_name = parse(linenumber, line, 17..20, &mut errors);
    let chain_id = String::from(parse_char(linenumber, chars, 21, &mut errors));
    //TODO: is isize?
    let residue_serial_number = parse(linenumber, line, 22..26, &mut errors);
    let insertion = parse_char(linenumber, chars, 26, &mut errors);
    let segment_id = parse(linenumber, line, 72..76, &mut errors);
    let element = parse(linenumber, line, 76..78, &mut errors);
    // 14% lex atom -> 12%
    let mut charge = 0;
    #[allow(clippy::unwrap_used)]
    if chars.len() >= 80 && !(chars[78] == b' ' && chars[79] == b' ') {
        if !chars[78].is_ascii_digit() {
            errors.push(PDBError::new(
                ErrorLevel::InvalidatingError,
                "Atom charge is not correct",
                "The charge is not numeric, it is defined to be [0-9][+-], so two characters in total.",
                Context::line(linenumber, line, 78, 1),
            ));
        } else if chars[79] != b'-' && chars[79] != b'+' {
            errors.push(PDBError::new(
                ErrorLevel::InvalidatingError,
                "Atom charge is not correct",
                "The charge is not properly signed, it is defined to be [0-9][+-], so two characters in total.",
                Context::line(linenumber, line, 79, 1),
            ));
        } else if let Some(digit) = chars[78].checked_sub(b'0').filter(|&n| n <= 9) {
            charge = digit as isize;
            if chars[79] == b'-' {
                charge *= -1;
            }
        } else {
            errors.push(PDBError::new(
                ErrorLevel::InvalidatingError,
                "Invalid charge digit",
                format!("Expected a digit but found '{}'", chars[78] as char),
                Context::line(linenumber, line, 78, 1),
            ));
        }
    }

    (
        (
            serial_number as usize,
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

    let a = parse(linenumber, line, 6..15, &mut errors);
    let b = parse(linenumber, line, 15..24, &mut errors);
    let c = parse(linenumber, line, 24..33, &mut errors);
    let alpha = parse(linenumber, line, 33..40, &mut errors);
    let beta = parse(linenumber, line, 40..47, &mut errors);
    let gamma = parse(linenumber, line, 47..54, &mut errors);
    let spacegroup = parse(linenumber, line, 55..cmp::min(66, chars.len()), &mut errors);
    let z = if chars.len() > 66 {
        parse(linenumber, line, 66..chars.len(), &mut errors)
    } else {
        1
    };

    (
        LexItem::Crystal(a, b, c, alpha, beta, gamma, spacegroup, z),
        errors,
    )
}

/// Lex a SCALEn (where `n` is given)
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

    let ser = parse(linenumber, line, 7..10, &mut errors);
    let (data, transformation_errors) = lex_transformation(linenumber, line);
    errors.extend(transformation_errors);

    let given = chars.len() >= 60 && chars[59] == '1';

    (LexItem::MtriX(row, ser, data, given), errors)
}

/// Lexes the general structure of a transformation record (ORIGXn, SCALEn, MTRIXn)
fn lex_transformation(linenumber: usize, line: &str) -> ([f64; 4], Vec<PDBError>) {
    let mut errors = Vec::new();

    let a = parse(linenumber, line, 10..20, &mut errors);
    let b = parse(linenumber, line, 20..30, &mut errors);
    let c = parse(linenumber, line, 30..40, &mut errors);
    let d = parse(linenumber, line, 45..55, &mut errors);

    ([a, b, c, d], errors)
}

/// Lex a MASTER
/// ## Fails
/// It fails on incorrect numbers in the line
fn lex_master(linenumber: usize, line: &str) -> (LexItem, Vec<PDBError>) {
    let mut errors = Vec::new();

    let num_remark = parse(linenumber, line, 10..15, &mut errors);
    let num_empty = parse(linenumber, line, 15..20, &mut errors);
    let num_het = parse(linenumber, line, 20..25, &mut errors);
    let num_helix = parse(linenumber, line, 25..30, &mut errors);
    let num_sheet = parse(linenumber, line, 30..35, &mut errors);
    let num_turn = parse(linenumber, line, 35..40, &mut errors);
    let num_site = parse(linenumber, line, 40..45, &mut errors);
    let num_xform = parse(linenumber, line, 45..50, &mut errors);
    let num_coord = parse(linenumber, line, 50..55, &mut errors);
    let num_ter = parse(linenumber, line, 55..60, &mut errors);
    let num_connect = parse(linenumber, line, 60..65, &mut errors);
    let num_seq = parse(linenumber, line, 65..70, &mut errors);

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
    let chars: &[u8] = line.as_bytes();

    let ser_num = parse(linenumber, line, 7..10, &mut errors);
    let chain_id = parse_char(linenumber, chars, 11, &mut errors);
    let num_res = parse(linenumber, line, 13..17, &mut errors);
    let mut values = Vec::new();
    let mut index = 19;
    let max = cmp::min(chars.len(), 71);
    while index + 3 <= max {
        let seq = line[index..index + 3].to_string();
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
    let chars: &[u8] = line.as_bytes();

    let id_code = parse(linenumber, line, 7..11, &mut errors);
    let chain_id = parse(linenumber, line, 12..13, &mut errors);
    let seq_begin = parse(linenumber, line, 14..18, &mut errors);
    let insert_begin = parse_char(linenumber, chars, 18, &mut errors);
    let seq_end = parse(linenumber, line, 20..24, &mut errors);
    let insert_end = parse_char(linenumber, chars, 24, &mut errors);
    let database = parse(linenumber, line, 26..32, &mut errors);
    let database_accession = parse(linenumber, line, 33..41, &mut errors);
    let database_id_code = parse(linenumber, line, 42..54, &mut errors);
    let database_seq_begin = parse(linenumber, line, 55..60, &mut errors);
    let database_insert_begin = parse_char(linenumber, chars, 60, &mut errors);
    let database_seq_end = parse(linenumber, line, 62..67, &mut errors);
    let database_insert_end = parse_char(linenumber, chars, 67, &mut errors);

    (
        LexItem::Dbref(
            id_code,
            chain_id,
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
    let chars: &[u8] = line.as_bytes();

    let id_code = parse(linenumber, line, 7..11, &mut errors);
    let chain_id = parse(linenumber, line, 12..13, &mut errors);
    let seq_begin = parse(linenumber, line, 14..18, &mut errors);
    let insert_begin = parse_char(linenumber, chars, 18, &mut errors);
    let seq_end = parse(linenumber, line, 21..24, &mut errors);
    let insert_end = parse_char(linenumber, chars, 24, &mut errors);
    let database = parse(linenumber, line, 26..32, &mut errors);
    let database_id_code = parse(linenumber, line, 47..67, &mut errors);

    (
        LexItem::Dbref1(
            id_code,
            chain_id,
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

    let id_code = parse(linenumber, line, 7..11, &mut errors);
    let chain_id = parse(linenumber, line, 12..13, &mut errors);
    let database_accession = parse(linenumber, line, 18..40, &mut errors);
    let database_seq_begin = parse(linenumber, line, 45..55, &mut errors);
    let database_seq_end = parse(linenumber, line, 57..67, &mut errors);

    (
        LexItem::Dbref2(
            id_code,
            chain_id,
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
    let chars: &[u8] = line.as_bytes();

    let id_code = parse(linenumber, line, 7..11, &mut errors);
    let res_name = parse(linenumber, line, 12..15, &mut errors);
    let chain_id = parse(linenumber, line, 16..17, &mut errors);
    let seq_num = parse(linenumber, line, 18..22, &mut errors);
    let insert = parse_char(linenumber, chars, 22, &mut errors);
    let database = parse(linenumber, line, 24..28, &mut errors);
    let database_accession = parse(linenumber, line, 29..38, &mut errors);

    let db_pos = if chars[39..48].iter().all(|c| *c == b' ') {
        None
    } else {
        let db_res_name = parse(linenumber, line, 39..42, &mut errors);
        let db_seq_num = parse(linenumber, line, 43..48, &mut errors);
        Some((db_res_name, db_seq_num))
    };
    let comment = parse(linenumber, line, 49..chars.len(), &mut errors);

    (
        LexItem::Seqadv(
            id_code,
            chain_id,
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
    let chars: &[u8] = line.as_bytes();

    let id = parse(linenumber, line, 7..11, &mut errors);
    let res_name = parse(linenumber, line, 12..15, &mut errors);
    let chain_id = parse_char(linenumber, chars, 16, &mut errors);
    let seq_num = parse(linenumber, line, 18..22, &mut errors);
    let insert = parse_char(linenumber, chars, 22, &mut errors);
    let std_res = parse(linenumber, line, 24..27, &mut errors);
    let comment = parse(linenumber, line, 29..chars.len(), &mut errors);

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

/// Parse a `SSBond` line into the corresponding `LexItem`
fn lex_ssbond(linenumber: usize, line: &str) -> (LexItem, Vec<PDBError>) {
    let mut errors = Vec::new();
    let chars: &[u8] = line.as_bytes();
    // The Serial number field is ignored
    let res_1 = parse(linenumber, line, 11..14, &mut errors);
    let chain_1 = parse_char(linenumber, chars, 15, &mut errors);
    let res_seq_1: isize = parse(linenumber, line, 17..21, &mut errors);
    let icode_1 = if chars[21] == b' ' {
        None
    } else {
        Some(String::from(parse_char(linenumber, chars, 21, &mut errors)))
    };
    let res_2 = parse(linenumber, line, 25..28, &mut errors);
    let chain_2 = parse_char(linenumber, chars, 29, &mut errors);
    let res_seq_2 = parse(linenumber, line, 31..35, &mut errors);
    let icode_2 = if chars[35] == b' ' {
        None
    } else {
        Some(String::from(parse_char(linenumber, chars, 35, &mut errors)))
    };

    let extra = if chars.len() >= 78 {
        let sym1 = parse(linenumber, line, 59..65, &mut errors);
        let sym2 = parse(linenumber, line, 66..72, &mut errors);
        let distance: f64 = parse(linenumber, line, 73..78, &mut errors);
        Some((sym1, sym2, distance))
    } else {
        None
    };

    (
        LexItem::SSBond(
            (res_1, res_seq_1, icode_1, chain_1.to_string()),
            (res_2, res_seq_2, icode_2, chain_2.to_string()),
            extra,
        ),
        errors,
    )
}

/// Parse a field from a line, with `T::default()` as fall back, leave errors in the given mutable vec.
fn parse<T: FromStr + Default>(
    linenumber: usize,
    line: &str,
    range: Range<usize>,
    errors: &mut Vec<PDBError>,
) -> T {
    parse_default(linenumber, line, range, errors, T::default())
}

/// Parse a field from a line, with the given default as fall back, leave errors in the given mutable vec
fn fast_parse_u64(
    linenumber: usize,
    line: &str,
    range: Range<usize>,
    errors: &mut Vec<PDBError>,
    default: u64,
) -> u64 {
    if line.len() < range.end {
        let context = Context::line(linenumber, line, range.start, range.len());
        errors.push(PDBError::new(
            ErrorLevel::InvalidatingError,
            "Line too short",
            format!(
                "This line was too short to parse the expected data field (at {} to {})",
                range.start, range.end
            ),
            context,
        ));
        return default;
    }

    let range_start = range.start;
    let range_len = range.len();
    match fast_parse_u64_from_string(&line[range]) {
        Ok(value) => value,
        Err(e) => {
            errors.push(PDBError::new(
                ErrorLevel::InvalidatingError,
                "Invalid number format",
                format!("Could not parse number: {e}"),
                Context::line(linenumber, line, range_start, range_len),
            ));
            default
        }
    }
}
/// Parse a field from a line, with the given default as fall back, leave errors in the given mutable vec
fn parse_default<T: FromStr>(
    linenumber: usize,
    line: &str,
    range: Range<usize>,
    errors: &mut Vec<PDBError>,
    default: T,
) -> T {
    if line.len() < range.end {
        let context = Context::line(linenumber, line, range.start, range.len());
        errors.push(PDBError::new(
            ErrorLevel::InvalidatingError,
            "Line too short",
            format!(
                "This line was too short to parse the expected data field (at {} to {})",
                range.start, range.end
            ),
            context,
        ));
        return default;
    }

    fast_trim(&line[range.clone()])
        .parse::<T>()
        .unwrap_or_else(|_| {
            errors.push(PDBError::new(
                ErrorLevel::InvalidatingError,
                "Invalid data in field",
                format!(
                    "The text presented is not of the right kind ({}).",
                    std::any::type_name::<T>()
                ),
                Context::line(linenumber, line, range.start, range.len()),
            ));
            default
        })
}

/// Parse a character, needed because the trim in the generic `parse` could leave us with an empty character leading to errors
fn parse_char(linenumber: usize, line: &[u8], position: usize, errors: &mut Vec<PDBError>) -> char {
    if position > line.len() {
        let context = Context::line(
            linenumber,
            String::from_utf8(line.to_vec()).expect("Failed to convert bytes to string"),
            position,
            1,
        );
        errors.push(PDBError::new(
            ErrorLevel::InvalidatingError,
            "Line too short",
            format!("This line was too short to parse the expected data field (at {position})"),
            context,
        ));
        return ' ';
    }
    line[position] as char
}
