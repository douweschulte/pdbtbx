use super::lexitem::*;
use super::structs::*;
use std::str::FromStr;

pub fn lex(input: &String) -> Result<Vec<LexItem>, String> {
    let mut result = Vec::new();
    let mut linenumber = 0;

    for line in input.split("\n") {
        linenumber += 1;
        if line.len() > 6 {
            match &line[..6] {
                "REMARK" => result.push(lex_remark(line)),
                "ATOM  " => result.push(lex_atom
                (linenumber, line, false).expect("ATOM error")),
                "HETATM" => result.push(lex_atom
                (linenumber, line, true).expect("HETATM error")),
                "CRYST1" => result.push(lex_cryst(linenumber, line).expect("CRYST1 error")),
                "SCALE1" => result.push(lex_scale(linenumber, line, 1).expect("SCALE1 error")),
                "SCALE2" => result.push(lex_scale(linenumber, line, 2).expect("SCALE2 error")),
                "SCALE3" => result.push(lex_scale(linenumber, line, 3).expect("SCALE3 error")),
                "MODEL " => result.push(LexItem::Model(line[6..].split_whitespace().collect::<String>())),
                "ENDMDL" => result.push(LexItem::EndModel()),
                _ => println!("Unknown: {}", line)
            }
        } else {
            if line.len() > 2 {
                match &line[..3] {
                    "TER"    => result.push(LexItem::TER()),
                    "END"    => result.push(LexItem::End()),
                    _ => println!("Unknown short line: {}", line)
                }
            }
            else if line != "" {
                println!("Short line: \"{}\" {}", line, line.len())
            }
        }
    }
    
    Ok(result)
}

fn lex_remark(line: &str) -> LexItem {
    LexItem::Remark(line[5..].to_string())
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
    let charge = [' ', ' '];
    if chars.len() == 80 {
        let charge = [chars[79], chars[80]];
    }

    Ok(LexItem::Atom(hetero, serial_number, atom_name, alternate_location, residue_name, chain_id, residue_serial_number, insertion, x, y, z, occupancy, b_factor, segment_id, element, charge))
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
    let space_group = &chars[56..].iter().collect::<String>().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    Ok(LexItem::Crystal(a, b, c, alpha, beta, gamma, space_group.to_vec()))
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
    let string = input.iter().collect::<String>().split_whitespace().collect::<String>();
    match string.parse::<T>() {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("\"{}\" is not a valid number (line: {})", string, linenumber))
    }
}


fn parse(input: &Vec<LexItem>) -> PDB {
    let mut stack = input.clone();
    let mut pdb = PDB::new();
    let mut current_model = Model::new();
    pdb.models.push(current_model);

    for item in stack {
        match item {
            LexItem::Remark(text) => pdb.remarks.push(text),
            LexItem::Atom(hetero, s, n, _, r, c, rs, _, x, y, z, o, b, _, e, ch) => {
                let atom = Atom {
                    serial_number: *s,
                    atom_name: *n,
                    x: *x,
                    y: *y,
                    z: *z,
                    occupancy: *o,
                    b_factor: *b,
                    element: *e,
                    charge: *ch,
                };

                if *hetero {
                    current_model.hetero_atoms.push(atom);
                } else {
                    let mut current_chain = &mut Chain::new(Some(*c));
                    for chain in &mut current_model.chains {
                        if chain.id == *c {
                            current_chain = chain;
                            break;
                        }
                    }

                    let mut current_residue = None;
                    for residue in &mut current_chain.residues {
                        if residue.serial_number == *rs {
                            current_residue = Some(residue);
                            break;
                        }
                    }

                    if let Some(res) = current_residue {
                        res.atoms.push(atom);
                    } else {
                        current_chain.residues.push(Residue::new(*rs, Some(atom)));
                    }
                }
            }
            _ => ()
        }
    }

    pdb
}

