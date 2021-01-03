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

    let mut linenumber = 0;

    let mut pdb = PDB::new();
    let mut current_model = Model::new(0, Some(&mut pdb));

    for read_line in reader.lines() {
        // Lex the line
        let line = &read_line.expect("Line not read");
        linenumber += 1;
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
                "ENDMDL" => Ok(LexItem::EndModel()),
                _ => Err("Unknown option".to_string()),
            }
        } else {
            if line.len() > 2 {
                match &line[..3] {
                    "TER" => Ok(LexItem::TER()),
                    "END" => Ok(LexItem::End()),
                    _ => Err(format!("Unknown short line: {}", line)),
                }
            } else if line != "" {
                Err(format!("Short line: \"{}\" {}", line, line.len()))
            } else {
                Ok(LexItem::Empty())
            }
        };

        // Then immediately add this lines information to the final PDB struct
        if let Ok(result) = lineresult {
            match result {
                LexItem::Remark(num, text) => pdb.add_remark(num, text.to_string()),
                LexItem::Atom(hetero, s, n, _, r, c, rs, _, x, y, z, o, b, _, e, ch) => {
                    let atom = Atom::new(None, s, n, x, y, z, o, b, e, ch)
                        .expect("Invalid characters in atom creation");

                    if hetero {
                        current_model.add_hetero_atom(atom, c, rs, r);
                    } else {
                        current_model.add_atom(atom, c, rs, r);
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
                            .expect(&format!("Invalid space group: \"{}\"", spacegroup)),
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
fn lex_remark(linenumber: usize, line: &str) -> Result<LexItem, String> {
    Ok(LexItem::Remark(
        parse_number(linenumber, &line.chars().collect::<Vec<char>>()[7..10])?,
        if line.len() > 11 {
            line[11..].to_string()
        } else {
            "".to_string()
        },
    ))
}

/// Lex a MODEL
/// ## Fails
/// It fails on incorrect numbers for the serial number
fn lex_model(linenumber: usize, line: &str) -> Result<LexItem, String> {
    Ok(LexItem::Model(parse_number(
        linenumber,
        &line[6..]
            .split_whitespace()
            .collect::<String>()
            .chars()
            .collect::<Vec<char>>()[..],
    )?))
}

/// Lex an ATOM
/// ## Fails
/// It fails on incorrect numbers in the line
fn lex_atom(linenumber: usize, line: &str, hetero: bool) -> Result<LexItem, String> {
    let chars: Vec<char> = line.chars().collect();
    let serial_number = parse_number(linenumber, &chars[7..11])?;
    let atom_name = [chars[12], chars[13], chars[14], chars[15]];
    let alternate_location = chars[16];
    let residue_name = [chars[17], chars[18], chars[19]];
    let chain_id = chars[21];
    let residue_serial_number = parse_number(linenumber, &chars[22..26])?;
    let insertion = chars[26];
    let x = parse_number(linenumber, &chars[30..38])?;
    let y = parse_number(linenumber, &chars[38..46])?;
    let z = parse_number(linenumber, &chars[46..54])?;
    let mut occupancy = 1.0;
    if chars.len() >= 60 {
        occupancy = parse_number(linenumber, &chars[54..60])?;
    }
    let mut b_factor = 0.0;
    if chars.len() >= 66 {
        b_factor = parse_number(linenumber, &chars[60..66])?;
    }
    let mut segment_id = [' ', ' ', ' ', ' '];
    if chars.len() >= 75 {
        segment_id = [chars[72], chars[73], chars[74], chars[75]];
    }
    let mut element = [' ', ' '];
    if chars.len() >= 77 {
        element = [chars[76], chars[77]];
    }
    let mut charge = [' ', ' '];
    if chars.len() >= 80 {
        charge = [chars[78], chars[79]];
    }

    Ok(LexItem::Atom(
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
    ))
}

/// Lex an ANISOU
/// ## Fails
/// It fails on incorrect numbers in the line
fn lex_anisou(linenumber: usize, line: &str) -> Result<LexItem, String> {
    let chars: Vec<char> = line.chars().collect();
    let serial_number = parse_number(linenumber, &chars[7..11])?;
    let atom_name = [chars[12], chars[13], chars[14], chars[15]];
    let alternate_location = chars[16];
    let residue_name = [chars[17], chars[18], chars[19]];
    let chain_id = chars[21];
    let residue_serial_number = parse_number(linenumber, &chars[22..26])?;
    let insertion = chars[26];
    let ai: isize = parse_number(linenumber, &chars[28..35])?;
    let bi: isize = parse_number(linenumber, &chars[35..42])?;
    let ci: isize = parse_number(linenumber, &chars[42..49])?;
    let di: isize = parse_number(linenumber, &chars[49..56])?;
    let ei: isize = parse_number(linenumber, &chars[56..63])?;
    let fi: isize = parse_number(linenumber, &chars[63..70])?;
    let a = (ai as f64) / 10000.0;
    let b = (bi as f64) / 10000.0;
    let c = (ci as f64) / 10000.0;
    let d = (di as f64) / 10000.0;
    let e = (ei as f64) / 10000.0;
    let f = (fi as f64) / 10000.0;
    let segment_id = [chars[72], chars[73], chars[74], chars[75]];
    let element = [chars[76], chars[77]];
    let mut charge = [' ', ' '];
    if chars.len() == 80 {
        charge = [chars[79], chars[80]];
    }

    Ok(LexItem::Anisou(
        serial_number,
        atom_name,
        alternate_location,
        residue_name,
        chain_id,
        residue_serial_number,
        insertion,
        [[a, b, c], [d, e, f]],
        segment_id,
        element,
        charge,
    ))
}

/// Lex a CRYST1
/// ## Fails
/// It fails on incorrect numbers in the line
fn lex_cryst(linenumber: usize, line: &str) -> Result<LexItem, String> {
    let chars: Vec<char> = line.chars().collect();
    let a = parse_number(linenumber, &chars[6..15])?;
    let b = parse_number(linenumber, &chars[15..24])?;
    let c = parse_number(linenumber, &chars[24..33])?;
    let alpha = parse_number(linenumber, &chars[33..40])?;
    let beta = parse_number(linenumber, &chars[40..47])?;
    let gamma = parse_number(linenumber, &chars[47..54])?;
    // TODO: make a fancy error message if a part of the space group is not numeric
    let spacegroup = chars[55..std::cmp::min(66, chars.len())]
        .iter()
        .collect::<String>();
    let mut z = 1;
    if chars.len() > 66 {
        z = parse_number(linenumber, &chars[66..])?;
    }

    Ok(LexItem::Crystal(a, b, c, alpha, beta, gamma, spacegroup, z))
}

/// Lex an SCALEn (where `n` is given)
/// ## Fails
/// It fails on incorrect numbers in the line
fn lex_scale(linenumber: usize, line: &str, n: usize) -> Result<LexItem, String> {
    let chars: Vec<char> = line.chars().collect();
    let a = parse_number(linenumber, &chars[10..20])?;
    let b = parse_number(linenumber, &chars[20..30])?;
    let c = parse_number(linenumber, &chars[30..40])?;
    let d = parse_number(linenumber, &chars[45..55])?;

    Ok(LexItem::Scale(n, [a, b, c, d]))
}

/// Lex an ORIGXn (where `n` is given)
/// ## Fails
/// It fails on incorrect numbers in the line
fn lex_origx(linenumber: usize, line: &str, n: usize) -> Result<LexItem, String> {
    let chars: Vec<char> = line.chars().collect();
    let a = parse_number(linenumber, &chars[10..20])?;
    let b = parse_number(linenumber, &chars[20..30])?;
    let c = parse_number(linenumber, &chars[30..40])?;
    let d = parse_number(linenumber, &chars[45..55])?;

    Ok(LexItem::OrigX(n, [a, b, c, d]))
}

/// Lex an MTRIXn (where `n` is given)
/// ## Fails
/// It fails on incorrect numbers in the line
fn lex_mtrix(linenumber: usize, line: &str, n: usize) -> Result<LexItem, String> {
    let chars: Vec<char> = line.chars().collect();
    let ser = parse_number(linenumber, &chars[7..10])?;
    let a = parse_number(linenumber, &chars[10..20])?;
    let b = parse_number(linenumber, &chars[20..30])?;
    let c = parse_number(linenumber, &chars[30..40])?;
    let d = parse_number(linenumber, &chars[45..55])?;
    let mut given = false;
    if chars.len() >= 60 {
        given = chars[59] == '1';
    }

    Ok(LexItem::MtriX(n, ser, [a, b, c, d], given))
}

/// Parse a number, generic for anything that can be parsed using FromStr
fn parse_number<T: FromStr>(linenumber: usize, input: &[char]) -> Result<T, String> {
    let string = input
        .iter()
        .collect::<String>()
        .split_whitespace()
        .collect::<String>();
    match string.parse::<T>() {
        Ok(v) => Ok(v),
        Err(_) => Err(format!(
            "\"{}\" is not a valid number (line: {})",
            string, linenumber
        )),
    }
}
