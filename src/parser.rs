use super::lexitem::*;
use super::structs::*;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn parse(filename: &str) -> Result<PDB, String> {
    // Open a file a use a buffered reader to minimise memory use while immediately lexing the line followed by adding it to the current PDB
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut linenumber = 0;

    let mut pdb = PDB::new();
    let mut current_model = Model::new(None);

    for read_line in reader.lines() {
        let line = &read_line.expect("Line not read");
        linenumber += 1;
        let lineresult = if line.len() > 6 {
            match &line[..6] {
                "REMARK" => Ok(lex_remark(line)),
                "ATOM  " => lex_atom(linenumber, line, false),
                "HETATM" => lex_atom(linenumber, line, true),
                "CRYST1" => lex_cryst(linenumber, line),
                "SCALE1" => lex_scale(linenumber, line, 0),
                "SCALE2" => lex_scale(linenumber, line, 1),
                "SCALE3" => lex_scale(linenumber, line, 2),
                "MODEL " => Ok(LexItem::Model(
                    line[6..].split_whitespace().collect::<String>(),
                )),
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

        if let Ok(result) = lineresult {
            match result {
                LexItem::Remark(text) => pdb.remarks.push(text.to_string()),
                LexItem::Atom(hetero, s, n, _, r, c, rs, _, x, y, z, o, b, _, e, ch) => {
                    let atom = Atom::new(r, s, n, x, y, z, o, b, e, ch);

                    if hetero {
                        current_model.hetero_atoms.push(atom);
                    } else {
                        let mut current_chain = None;
                        for chain in &mut current_model.chains {
                            if chain.id == c {
                                current_chain = Some(chain);
                                break;
                            }
                        }

                        if let Some(chain) = current_chain {
                            let mut current_residue = None;
                            for residue in &mut chain.residues {
                                if residue.serial_number == rs {
                                    current_residue = Some(residue);
                                    break;
                                }
                            }

                            if let Some(res) = current_residue {
                                res.atoms.push(atom);
                            } else {
                                chain.residues.push(Residue::new(rs, Some(r), Some(atom)));
                            }
                        } else {
                            let mut chain = Chain::new(Some(c));
                            chain.residues.push(Residue::new(rs, Some(r), Some(atom)));
                            current_model.chains.push(chain);
                        }
                    }
                }
                LexItem::Model(name) => {
                    if current_model.atoms().len() > 0 {
                        pdb.models.push(current_model)
                    }

                    current_model = Model::new(Some(&name));
                }
                LexItem::Scale(n, row) => {
                    if pdb.scale.is_none() {
                        pdb.scale = Some(Scale::new());
                    }
                    pdb.scale().factors[n] = row;
                }
                LexItem::Crystal(a, b, c, alpha, beta, gamma, symmetry) => {
                    pdb.unit_cell = Some(UnitCell::new(a, b, c, alpha, beta, gamma));
                    pdb.symmetry = Some(Symmetry::new(symmetry.to_vec()));
                }
                _ => (),
            }
        }
    }
    pdb.models.push(current_model);
    Ok(pdb)
}

fn lex_remark(line: &str) -> LexItem {
    LexItem::Remark(line[6..].to_string())
}

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
    let occupancy = parse_number(linenumber, &chars[54..60])?;
    let b_factor = parse_number(linenumber, &chars[60..66])?;
    let segment_id = [chars[72], chars[73], chars[74], chars[75]];
    let element = [chars[76], chars[77]];
    let mut charge = [' ', ' '];
    if chars.len() == 80 {
        charge = [chars[79], chars[80]];
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

fn lex_cryst(linenumber: usize, line: &str) -> Result<LexItem, String> {
    let chars: Vec<char> = line.chars().collect();
    let a = parse_number(linenumber, &chars[6..15])?;
    let b = parse_number(linenumber, &chars[15..24])?;
    let c = parse_number(linenumber, &chars[24..33])?;
    let alpha = parse_number(linenumber, &chars[33..40])?;
    let beta = parse_number(linenumber, &chars[40..47])?;
    let gamma = parse_number(linenumber, &chars[47..54])?;
    // TODO: make a fancy error message if a part of the space group is not numeric
    let space_group = &chars[56..]
        .iter()
        .collect::<String>()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    Ok(LexItem::Crystal(
        a,
        b,
        c,
        alpha,
        beta,
        gamma,
        space_group.to_vec(),
    ))
}

fn lex_scale(linenumber: usize, line: &str, n: usize) -> Result<LexItem, String> {
    let chars: Vec<char> = line.chars().collect();
    let a = parse_number(linenumber, &chars[10..20])?;
    let b = parse_number(linenumber, &chars[20..30])?;
    let c = parse_number(linenumber, &chars[30..40])?;
    let d = parse_number(linenumber, &chars[45..55])?;

    Ok(LexItem::Scale(n, [a, b, c, d]))
}

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
