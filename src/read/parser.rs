use super::lexitem::*;
use crate::structs::*;
use crate::validate::*;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

/// Parse the given filename into a PDB struct
/// Returns an error message if it fails to parse it properly
pub fn parse(filename: &str) -> Result<PDB, String> {
    // Open a file a use a buffered reader to minimise memory use while immediately lexing the line followed by adding it to the current PDB
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut pdb = PDB::new();
    let mut current_model = Model::new(0, Some(&mut pdb));

    for (linenumber, read_line) in reader.lines().enumerate() {
        // Lex the line
        let line = read_line.expect("Line not read").into_bytes(); // TODO: make the iterator directly produce bytes
        let len = line.len();
        let lineresult = if len > 6 {
            match &line[..6] {
                b"REMARK" => lex_remark(linenumber, line),
                b"ATOM  " => lex_atom(linenumber, line, false),
                b"ANISOU" => lex_anisou(linenumber, line),
                b"HETATM" => lex_atom(linenumber, line, true),
                b"CRYST1" => lex_cryst(linenumber, line),
                b"SCALE1" => lex_scale(linenumber, line, 0),
                b"SCALE2" => lex_scale(linenumber, line, 1),
                b"SCALE3" => lex_scale(linenumber, line, 2),
                b"ORIGX1" => lex_origx(linenumber, line, 0),
                b"ORIGX2" => lex_origx(linenumber, line, 1),
                b"ORIGX3" => lex_origx(linenumber, line, 2),
                b"MTRIX1" => lex_mtrix(linenumber, line, 0),
                b"MTRIX2" => lex_mtrix(linenumber, line, 1),
                b"MTRIX3" => lex_mtrix(linenumber, line, 2),
                b"MODEL " => lex_model(linenumber, line),
                b"ENDMDL" => Ok(LexItem::EndModel()),
                _ => Err("Unknown option".to_string()),
            }
        } else if len > 2 {
            match &line[..3] {
                b"TER" => Ok(LexItem::TER()),
                b"END" => Ok(LexItem::End()),
                _ => Err(format!(
                    "Unknown short line: {}",
                    String::from_utf8(line).unwrap()
                )),
            }
        } else if len != 0 {
            Err(format!(
                "Short line: \"{}\" {}",
                String::from_utf8(line).unwrap(),
                len
            ))
        } else {
            Ok(LexItem::Empty())
        };

        // Then immediately add this lines information to the final PDB struct
        if let Ok(result) = lineresult {
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
                    let atom =
                        Atom::new(None, serial_number, name, x, y, z, occ, b, element, charge)
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
                            std::str::from_utf8(&n).unwrap().to_owned()
                        )
                    }
                }
                LexItem::Model(number) => {
                    if current_model.atom_count() > 0 {
                        pdb.add_model(current_model)
                    }

                    current_model = Model::new(number, Some(&mut pdb));
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
                _ => (),
            }
        }
    }
    pdb.add_model(current_model);
    if validate(&pdb) {
        Ok(pdb)
    } else {
        Err("Not a valid PDB resulting model".to_string())
    }
}

/// Lex a REMARK
/// ## Fails
/// It fails on incorrect numbers for the remark-type-number
fn lex_remark(linenumber: usize, line: Vec<u8>) -> Result<LexItem, String> {
    Ok(LexItem::Remark(
        parse_number(linenumber, &line[7..10])?,
        if line.len() > 11 {
            std::str::from_utf8(&line[11..]).unwrap().to_owned()
        } else {
            "".to_string()
        },
    ))
}

/// Lex a MODEL
/// ## Fails
/// It fails on incorrect numbers for the serial number
fn lex_model(linenumber: usize, line: Vec<u8>) -> Result<LexItem, String> {
    Ok(LexItem::Model(parse_number(linenumber, &line[6..])?))
}

/// Lex an ATOM
/// ## Fails
/// It fails on incorrect numbers in the line
fn lex_atom(linenumber: usize, line: Vec<u8>, hetero: bool) -> Result<LexItem, String> {
    let serial_number = parse_number(linenumber, &line[7..11])?;
    let atom_name = [line[12], line[13], line[14], line[15]];
    let alternate_location = line[16];
    let residue_name = [line[17], line[18], line[19]];
    let chain_id = line[21];
    let residue_serial_number = parse_number(linenumber, &line[22..26])?;
    let insertion = line[26];
    let x = parse_number(linenumber, &line[30..38])?;
    let y = parse_number(linenumber, &line[38..46])?;
    let z = parse_number(linenumber, &line[46..54])?;
    let mut occupancy = 1.0;
    if line.len() >= 60 {
        occupancy = parse_number(linenumber, &line[54..60])?;
    }
    let mut b_factor = 0.0;
    if line.len() >= 66 {
        b_factor = parse_number(linenumber, &line[60..66])?;
    }
    let mut segment_id = *b"    ";
    if line.len() >= 75 {
        segment_id = [line[72], line[73], line[74], line[75]];
    }
    let mut element = *b"  ";
    if line.len() >= 77 {
        element = [line[76], line[77]];
    }
    let mut charge = *b"  ";
    if line.len() >= 80 {
        charge = [line[78], line[79]];
    }

    Ok(LexItem::Atom(
        hetero,
        serial_number,
        atom_name,
        alternate_location as char,
        residue_name,
        chain_id as char,
        residue_serial_number,
        insertion as char,
        x,
        y,
        z,
        occupancy,
        b_factor,
        segment_id,
        element,
        charge,
    ))
}

/// Lex an ANISOU
/// ## Fails
/// It fails on incorrect numbers in the line
fn lex_anisou(linenumber: usize, line: Vec<u8>) -> Result<LexItem, String> {
    let serial_number = parse_number(linenumber, &line[7..11])?;
    let atom_name = [line[12], line[13], line[14], line[15]];
    let alternate_location = line[16];
    let residue_name = [line[17], line[18], line[19]];
    let chain_id = line[21];
    let residue_serial_number = parse_number(linenumber, &line[22..26])?;
    let insertion = line[26];
    let ai: isize = parse_number(linenumber, &line[28..35])?;
    let bi: isize = parse_number(linenumber, &line[35..42])?;
    let ci: isize = parse_number(linenumber, &line[42..49])?;
    let di: isize = parse_number(linenumber, &line[49..56])?;
    let ei: isize = parse_number(linenumber, &line[56..63])?;
    let fi: isize = parse_number(linenumber, &line[63..70])?;
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
    let segment_id = [line[72], line[73], line[74], line[75]];
    let element = [line[76], line[77]];
    let mut charge = *b"  ";
    if line.len() == 80 {
        charge = [line[79], line[80]];
    }

    Ok(LexItem::Anisou(
        serial_number,
        atom_name,
        alternate_location as char,
        residue_name,
        chain_id as char,
        residue_serial_number,
        insertion as char,
        factors,
        segment_id,
        element,
        charge,
    ))
}

/// Lex a CRYST1
/// ## Fails
/// It fails on incorrect numbers in the line
fn lex_cryst(linenumber: usize, line: Vec<u8>) -> Result<LexItem, String> {
    let a = parse_number(linenumber, &line[6..15])?;
    let b = parse_number(linenumber, &line[15..24])?;
    let c = parse_number(linenumber, &line[24..33])?;
    let alpha = parse_number(linenumber, &line[33..40])?;
    let beta = parse_number(linenumber, &line[40..47])?;
    let gamma = parse_number(linenumber, &line[47..54])?;
    // TODO: make a fancy error message if a part of the space group is not numeric
    let spacegroup = std::str::from_utf8(&line[55..std::cmp::min(66, line.len())])
        .unwrap()
        .to_owned();
    let mut z = 1;
    if line.len() > 66 {
        z = parse_number(linenumber, &line[66..])?;
    }

    Ok(LexItem::Crystal(a, b, c, alpha, beta, gamma, spacegroup, z))
}

/// Lex an SCALEn (where `n` is given)
/// ## Fails
/// It fails on incorrect numbers in the line
fn lex_scale(linenumber: usize, line: Vec<u8>, row: usize) -> Result<LexItem, String> {
    let a = parse_number(linenumber, &line[10..20])?;
    let b = parse_number(linenumber, &line[20..30])?;
    let c = parse_number(linenumber, &line[30..40])?;
    let d = parse_number(linenumber, &line[45..55])?;

    Ok(LexItem::Scale(row, [a, b, c, d]))
}

/// Lex an ORIGXn (where `n` is given)
/// ## Fails
/// It fails on incorrect numbers in the line
fn lex_origx(linenumber: usize, line: Vec<u8>, row: usize) -> Result<LexItem, String> {
    let a = parse_number(linenumber, &line[10..20])?;
    let b = parse_number(linenumber, &line[20..30])?;
    let c = parse_number(linenumber, &line[30..40])?;
    let d = parse_number(linenumber, &line[45..55])?;

    Ok(LexItem::OrigX(row, [a, b, c, d]))
}

/// Lex an MTRIXn (where `n` is given)
/// ## Fails
/// It fails on incorrect numbers in the line
fn lex_mtrix(linenumber: usize, line: Vec<u8>, row: usize) -> Result<LexItem, String> {
    let ser = parse_number(linenumber, &line[7..10])?;
    let a = parse_number(linenumber, &line[10..20])?;
    let b = parse_number(linenumber, &line[20..30])?;
    let c = parse_number(linenumber, &line[30..40])?;
    let d = parse_number(linenumber, &line[45..55])?;
    let mut given = false;
    if line.len() >= 60 {
        given = line[59] == b'1';
    }

    Ok(LexItem::MtriX(row, ser, [a, b, c, d], given))
}

/// Parse a number, generic for anything that can be parsed using FromStr
fn parse_number<T: FromStr>(linenumber: usize, input: &[u8]) -> Result<T, String> {
    // TODO: parse directly from bytes
    let string = String::from_utf8(
        input
            .iter()
            .filter_map(|c| {
                if !c.is_ascii_whitespace() {
                    Some(*c)
                } else {
                    None
                }
            })
            .collect::<Vec<u8>>(),
    )
    .unwrap();
    match string.parse::<T>() {
        Ok(v) => Ok(v),
        Err(_) => Err(format!(
            "\"{}\" is not a valid number (line: {})",
            string, linenumber
        )),
    }
}
