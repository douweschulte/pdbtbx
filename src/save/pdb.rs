use std::{
    cmp,
    fs::File,
    io::{BufWriter, Write},
    iter,
};

use context_error::{BoxedError, Context, CreateError, ErrorKind, FullErrorContent};
#[cfg(feature = "compression")]
use flate2::{write::GzEncoder, Compression};

use crate::structs::*;
use crate::StrictnessLevel;
use crate::TransformationMatrix;
use crate::{validate, validate_pdb, ErrorLevel};

/// Save the given PDB struct to the given file, validating it beforehand.
///
/// # Errors
/// It fails if the validation fails with the given `level`.
/// If validation gives rise to problems, use the `save_raw` function.
///
/// # Known Problems
/// Saving SEQRES lines is experimental, as there are many nitpicky things to consider
/// when generating SEQRES records, which are not all implemented (yet).
pub fn save_pdb(
    pdb: &PDB,
    filename: impl AsRef<str>,
    level: StrictnessLevel,
) -> Result<(), Vec<BoxedError<'static, ErrorLevel>>> {
    save_pdb_(pdb, filename, level, BufWriter::new)
}

/// Save the given PDB struct to the given file, validating it beforehand, and use gzip compression.
///
/// # Errors
/// It fails if the validation fails with the given `level`.
/// If validation gives rise to problems, use the `save_raw` function.
///
/// # Known Problems
/// Saving SEQRES lines is experimental, as there are many nitpicky things to consider
/// when generating SEQRES records, which are not all implemented (yet).
///
#[cfg(feature = "compression")]
pub fn save_pdb_gz(
    pdb: &PDB,
    filename: impl AsRef<str>,
    level: StrictnessLevel,
    compression_level: Option<Compression>,
) -> Result<(), Vec<BoxedError<'static, ErrorLevel>>> {
    save_pdb_(pdb, filename, level, |file| {
        let encoder = match compression_level {
            Some(level) => GzEncoder::new(file, level),
            None => GzEncoder::new(file, Compression::default()),
        };
        BufWriter::new(encoder)
    })
}

/// Generic function to save the given PDB struct to the given file, validating it beforehand.
fn save_pdb_<T, W>(
    pdb: &PDB,
    filename: impl AsRef<str>,
    level: StrictnessLevel,
    writer: W,
) -> Result<(), Vec<BoxedError<'static, ErrorLevel>>>
where
    T: Write,
    W: FnOnce(File) -> BufWriter<T>,
{
    // Validates the PDB, and returns early if any errors are found
    let filename = filename.as_ref();

    let mut errors = validate(pdb);
    errors.extend(validate_pdb(pdb));
    for error in &errors {
        if error.get_kind().is_error(level) {
            return Err(errors);
        }
    }

    // Creates a writer for the file
    let file = match File::create(filename) {
        Ok(f) => f,
        Err(_e) => {
            errors.push(BoxedError::new(
                ErrorLevel::BreakingError,
                "Could not open file",
                "Could not open the file for writing, make sure you have permission for this file and no other program is currently using it.",
                Context::default().source(filename.to_string())
            ));
            return Err(errors);
        }
    };

    let writer = writer(file);

    // Now call the writer function
    save_pdb_raw(pdb, writer, level);

    Ok(())
}

/// Save the given PDB struct to the given `BufWriter`.
/// It does not validate or renumber the PDB, so if that is needed, that needs to be done in preparation.
/// It does change the output format based on the `StrictnessLevel` given.
///
/// ## Loose
/// * Does not pad all lines to 70 chars length
/// * Does not save the MASTER record
#[allow(clippy::unwrap_used)]
pub fn save_pdb_raw<T: Write>(pdb: &PDB, mut sink: BufWriter<T>, level: StrictnessLevel) {
    let get_line = |fields: Vec<(usize, &str)>| {
        let mut line = String::with_capacity(70);
        for (length, text) in fields {
            if length > 0 {
                let cell = &text[text.len() - cmp::min(length, text.len())..];
                let trimmed = cell.trim_start_matches('0');
                if !cell.is_empty() && trimmed.is_empty() {
                    std::fmt::write(&mut line, format_args!("{0:1$}", "0", length)).unwrap();
                } else {
                    std::fmt::write(&mut line, format_args!("{trimmed:length$}")).unwrap();
                }
            } else {
                line += text;
            }
        }
        line
    };
    let mut print_line = |fields: Vec<(usize, &str)>| {
        let mut line = get_line(fields);
        if level != StrictnessLevel::Loose && line.len() < 70 {
            let dif = 70 - line.len();
            line.reserve(dif);
            line.extend(iter::repeat(" ").take(dif));
        }
        sink.write_all(line.as_bytes()).unwrap();
        sink.write_all(b"\n").unwrap();
    };
    /// Get the inner str of an Option<&str> or "" if the option is None
    macro_rules! get_option {
        ($option:expr) => {
            $option.as_deref().unwrap_or("")
        };
    }

    if let Some(name) = &pdb.identifier {
        print_line(vec![
            (
                0,
                "HEADER                                                        ",
            ),
            (0, name),
        ]);
    }

    // Remarks
    for line in pdb.remarks() {
        print_line(vec![
            (6, "REMARK"),
            (0, " "),
            (3, &line.0.to_string()),
            (0, " "),
            (0, &line.1),
        ]);
    }

    if let Some(model) = pdb.models().next() {
        // DBREF
        let mut seqres = level == StrictnessLevel::Strict;
        for chain in model.chains() {
            if let Some(dbref) = chain.database_reference() {
                seqres = true;
                if dbref.database.acc.len() > 8
                    || dbref.database.id.len() > 12
                    || dbref.database_position.start > 1_000_000 - 1
                    || dbref.database_position.end > 1_000_000 - 1
                {
                    print_line(vec![
                        (6, "DBREF1"),
                        (0, " "),
                        (4, pdb.identifier.as_deref().unwrap_or("")),
                        (0, " "),
                        (1, chain.id()),
                        (0, " "),
                        (4, &dbref.pdb_position.start.to_string()),
                        (1, get_option!(dbref.pdb_position.start_insert)),
                        (0, " "),
                        (4, &dbref.pdb_position.end.to_string()),
                        (1, get_option!(dbref.pdb_position.end_insert)),
                        (0, " "),
                        (6, &dbref.database.name),
                        (0, "               "),
                        (20, &dbref.database.id),
                    ]);
                    print_line(vec![
                        (6, "DBREF2"),
                        (0, " "),
                        (4, pdb.identifier.as_deref().unwrap_or("")),
                        (0, " "),
                        (1, chain.id()),
                        (0, "     "),
                        (22, &dbref.database.acc),
                        (0, "     "),
                        (10, &dbref.database_position.start.to_string()),
                        (0, "  "),
                        (10, &dbref.database_position.end.to_string()),
                    ]);
                } else {
                    print_line(vec![
                        (6, "DBREF"),
                        (0, " "),
                        (4, pdb.identifier.as_deref().unwrap_or("")),
                        (0, " "),
                        (1, chain.id()),
                        (0, " "),
                        (4, &dbref.pdb_position.start.to_string()),
                        (1, get_option!(dbref.pdb_position.start_insert)),
                        (0, " "),
                        (4, &dbref.pdb_position.end.to_string()),
                        (1, get_option!(dbref.pdb_position.end_insert)),
                        (0, " "),
                        (6, &dbref.database.name),
                        (0, " "),
                        (8, &dbref.database.acc),
                        (0, " "),
                        (12, &dbref.database.id),
                        (0, " "),
                        (5, &dbref.database_position.start.to_string()),
                        (1, get_option!(dbref.database_position.start_insert)),
                        (0, " "),
                        (5, &dbref.database_position.end.to_string()),
                        (1, get_option!(dbref.pdb_position.end_insert)),
                    ]);
                }
            }
        }

        // SEQADV
        for chain in model.chains() {
            if let Some(dbref) = chain.database_reference() {
                for dif in &dbref.differences {
                    print_line(vec![
                        (6, "SEQADV"),
                        (0, " "),
                        (4, pdb.identifier.as_deref().unwrap_or("")),
                        (0, " "),
                        (3, &dif.residue.0),
                        (0, " "),
                        (1, chain.id()),
                        (0, " "),
                        (4, &dif.residue.1.to_string()),
                        (0, "  "), // includes always empty field
                        (4, &dbref.database.name),
                        (0, " "),
                        (9, &dbref.database.acc),
                        (0, " "),
                        (
                            3,
                            dif.database_residue
                                .as_ref()
                                .map_or("", |(a, _)| a.as_str()),
                        ),
                        (0, " "),
                        (
                            5,
                            dif.database_residue
                                .as_ref()
                                .map_or(&0, |(_, a)| a)
                                .to_string()
                                .as_str(),
                        ),
                        (0, " "),
                        (0, &dif.comment),
                    ]);
                }
            }
        }

        // SEQRES
        if seqres {
            for chain in model.chains() {
                if let Some(dbref) = chain.database_reference() {
                    for (index, chunk) in chain
                        .residues()
                        .skip_while(|r| {
                            r.id()
                                != (
                                    dbref.pdb_position.start,
                                    dbref.pdb_position.start_insert.as_deref(),
                                )
                        })
                        .filter(|r| r.name() != Some("HOH"))
                        .map(|r| {
                            format!(
                                "{:3}",
                                r.name()
                                    .expect("Residue has multiple conformers in SEQRES generation")
                            )
                        })
                        .collect::<Vec<String>>()
                        .chunks(13)
                        .enumerate()
                    {
                        print_line(vec![
                            (6, "SEQRES"),
                            (0, " "),
                            (3, (index + 1).to_string().as_str()),
                            (0, " "),
                            (1, chain.id()),
                            (0, " "),
                            (
                                4,
                                chain
                                    .residues()
                                    .filter(|r| r.name() != Some("HOH"))
                                    .count()
                                    .to_string()
                                    .as_str(),
                            ),
                            (0, "  "),
                            (0, &chunk.join(" ")),
                        ]);
                    }
                } else {
                    for (index, chunk) in chain
                        .residues()
                        .filter(|r| r.name() != Some("HOH"))
                        .map(|r| {
                            format!(
                                "{:3}",
                                r.name()
                                    .expect("Residue has multiple conformers in SEQRES generation")
                            )
                        })
                        .collect::<Vec<String>>()
                        .chunks(13)
                        .enumerate()
                    {
                        print_line(vec![
                            (6, "SEQRES"),
                            (0, " "),
                            (3, (index + 1).to_string().as_str()),
                            (0, " "),
                            (1, chain.id()),
                            (0, " "),
                            (
                                4,
                                chain
                                    .residues()
                                    .filter(|r| r.name() != Some("HOH"))
                                    .count()
                                    .to_string()
                                    .as_str(),
                            ),
                            (0, "  "),
                            (0, &chunk.join(" ")),
                        ]);
                    }
                }
            }
        }

        // MODRES
        for chain in model.chains() {
            for residue in chain.residues() {
                for conformer in residue.conformers() {
                    if let Some((std_name, comment)) = conformer.modification() {
                        print_line(vec![
                            (6, "MODRES"),
                            (0, "      "), // includes empty field
                            (3, conformer.name()),
                            (0, " "),
                            (1, chain.id()),
                            (0, " "),
                            (4, residue.serial_number().to_string().as_str()),
                            (1, residue.insertion_code().unwrap_or(" ")),
                            (0, " "),
                            (3, std_name),
                            (0, "  "),
                            (0, comment),
                        ]);
                    }
                }
            }
        }
    }
    // Cryst
    if let Some(unit_cell) = &pdb.unit_cell {
        let sym = pdb.symmetry.as_ref().map_or_else(
            || "P 1         1".to_string(),
            |symmetry| format!("{:10}{:3}", symmetry.herman_mauguin_symbol(), symmetry.z(),),
        );
        print_line(vec![
            (6, "CRYST1"),
            (9, &format!("{:9.3}", unit_cell.a())),
            (9, &format!("{:9.3}", unit_cell.b())),
            (9, &format!("{:9.3}", unit_cell.c())),
            (7, &format!("{:7.2}", unit_cell.alpha())),
            (7, &format!("{:7.2}", unit_cell.beta())),
            (7, &format!("{:7.2}", unit_cell.gamma())),
            (0, "  "),
            (0, &sym),
        ]);
    }

    let mut write_matrix = |name, matrix: [[f64; 4]; 3]| {
        print_line(vec![
            (5, name),
            (0, "1"),
            (0, "    "),
            (10, &format!("{:10.6}", matrix[0][0])),
            (10, &format!("{:10.6}", matrix[0][1])),
            (10, &format!("{:10.6}", matrix[0][2])),
            (0, "     "),
            (10, &format!("{:10.5}", matrix[0][3])),
        ]);
        print_line(vec![
            (5, name),
            (0, "2"),
            (0, "    "),
            (10, &format!("{:10.6}", matrix[1][0])),
            (10, &format!("{:10.6}", matrix[1][1])),
            (10, &format!("{:10.6}", matrix[1][2])),
            (0, "     "),
            (10, &format!("{:10.5}", matrix[1][3])),
        ]);
        print_line(vec![
            (5, name),
            (0, "3"),
            (0, "    "),
            (10, &format!("{:10.6}", matrix[2][0])),
            (10, &format!("{:10.6}", matrix[2][1])),
            (10, &format!("{:10.6}", matrix[2][2])),
            (0, "     "),
            (10, &format!("{:10.5}", matrix[2][3])),
        ]);
    };

    // OrigX
    if let Some(origx) = &pdb.origx {
        write_matrix("ORIGX", origx.matrix());
    } else if level == StrictnessLevel::Strict {
        write_matrix("ORIGX", TransformationMatrix::identity().matrix());
    }

    // Scale
    if let Some(scale) = &pdb.scale {
        write_matrix("SCALE", scale.matrix());
    } else if level == StrictnessLevel::Strict {
        if let Some(unit_cell) = &pdb.unit_cell {
            write_matrix(
                "SCALE",
                TransformationMatrix::scale(
                    1.0 / unit_cell.a(),
                    1.0 / unit_cell.b(),
                    1.0 / unit_cell.c(),
                )
                .matrix(),
            );
        }
    }

    // MtriX
    for mtrix in pdb.mtrix() {
        let m = mtrix.transformation.matrix();
        print_line(vec![
            (0, "MTRIX1"),
            (0, " "),
            (3, mtrix.serial_number.to_string().as_str()),
            (10, &format!("{:10.6}", m[0][0])),
            (10, &format!("{:10.6}", m[0][1])),
            (10, &format!("{:10.6}", m[0][2])),
            (0, "     "),
            (10, &format!("{:10.5}", m[0][3])),
            (0, "    "),
            (0, if mtrix.contained { "1" } else { " " }),
        ]);
        print_line(vec![
            (0, "MTRIX2"),
            (0, " "),
            (3, mtrix.serial_number.to_string().as_str()),
            (10, &format!("{:10.6}", m[1][0])),
            (10, &format!("{:10.6}", m[1][1])),
            (10, &format!("{:10.6}", m[1][2])),
            (0, "     "),
            (10, &format!("{:10.5}", m[1][3])),
            (0, "    "),
            (0, if mtrix.contained { "1" } else { " " }),
        ]);
        print_line(vec![
            (0, "MTRIX3"),
            (0, " "),
            (3, mtrix.serial_number.to_string().as_str()),
            (10, &format!("{:10.6}", m[2][0])),
            (10, &format!("{:10.6}", m[2][1])),
            (10, &format!("{:10.6}", m[2][2])),
            (0, "     "),
            (10, &format!("{:10.5}", m[2][3])),
            (0, "    "),
            (0, if mtrix.contained { "1" } else { " " }),
        ]);
    }

    // Models
    let multiple_models = pdb.models().size_hint().0 > 1;
    for model in pdb.models() {
        if multiple_models {
            print_line(vec![
                (0, "MODEL        "),
                (0, model.serial_number().to_string().as_str()),
            ]);
        }

        let atom_line = |atom: &Atom, conformer: &Conformer, residue: &Residue, chain: &Chain| {
            get_line(vec![
                (5, atom.serial_number().to_string().as_str()),
                (0, " "),
                (4, atom.name()),
                (1, conformer.alternative_location().unwrap_or(" ")),
                (4, conformer.name()),
                (1, chain.id()),
                (4, residue.serial_number().to_string().as_str()),
                (1, residue.insertion_code().unwrap_or(" ")),
            ])
        };

        for chain in model.chains().filter(|c| c.atoms().next().is_some()) {
            for residue in chain.residues() {
                for conformer in residue.conformers() {
                    for atom in conformer.atoms() {
                        let element = atom.element().map_or_else(|| "", Element::symbol);
                        print_line(vec![
                            (6, if atom.hetero() { "HETATM" } else { "ATOM  " }),
                            (0, &atom_line(atom, conformer, residue, chain)),
                            (0, "   "),
                            (8, &format!("{:8.3}", atom.pos().0)),
                            (8, &format!("{:8.3}", atom.pos().1)),
                            (8, &format!("{:8.3}", atom.pos().2)),
                            (6, &format!("{:6.2}", atom.occupancy())),
                            (6, &format!("{:6.2}", atom.b_factor())),
                            (0, "          "),
                            (2, element),
                            (0, &atom.pdb_charge()),
                        ]);
                        #[allow(clippy::cast_possible_truncation)]
                        if atom.anisotropic_temperature_factors().is_some() {
                            let f = atom.anisotropic_temperature_factors().unwrap();
                            print_line(vec![
                                (6, "ANISOU"),
                                (0, &atom_line(atom, conformer, residue, chain)),
                                (0, " "),
                                (7, &format!("{:8.3}", (f[0][0] * 10000.0) as isize)),
                                (7, &format!("{:8.3}", (f[1][1] * 10000.0) as isize)),
                                (7, &format!("{:8.3}", (f[2][2] * 10000.0) as isize)),
                                (7, &format!("{:8.3}", (f[0][1] * 10000.0) as isize)),
                                (7, &format!("{:8.3}", (f[0][2] * 10000.0) as isize)),
                                (7, &format!("{:8.3}", (f[1][2] * 10000.0) as isize)),
                                (0, "      "),
                                (2, element),
                                (0, &atom.pdb_charge()),
                            ]);
                        }
                    }
                }
            }
            let last_atom = chain.atoms().nth_back(0).unwrap();
            let last_residue = chain.residues().nth_back(0).unwrap();
            let last_conformer = chain.conformers().nth_back(0).unwrap();
            print_line(vec![
                (0, "TER"),
                (5, last_atom.serial_number().to_string().as_str()),
                (0, "      "),
                (3, last_conformer.name()),
                (0, " "),
                (1, chain.id()),
                (4, last_residue.serial_number().to_string().as_str()),
            ]);
        }
        if multiple_models {
            print_line(vec![(0, "ENDMDL")]);
        }
    }
    if level != StrictnessLevel::Loose {
        let mut xform = 0;
        if pdb.origx.is_some() || level == StrictnessLevel::Strict {
            xform += 3;
        }
        if pdb.scale.is_some() || (level == StrictnessLevel::Strict && pdb.unit_cell.is_some()) {
            xform += 3;
        }
        for _ in pdb.mtrix() {
            xform += 3;
        }
        print_line(vec![
            (0, "MASTER    "),
            (5, pdb.remark_count().to_string().as_str()),
            (5, "0"), //defined to be empty
            (5, "0"), //numHet
            (5, "0"), //numHelix
            (5, "0"), //numSheet
            (5, "0"), //numTurn (deprecated)
            (5, "0"), //numSite
            (5, xform.to_string().as_str()),
            (5, pdb.total_atom_count().to_string().as_str()),
            (5, pdb.model_count().to_string().as_str()),
            (5, "0"), //numConnect
            (5, "0"), //numSeq
        ]);
    }
    print_line(vec![(0, "END")]);

    sink.flush().unwrap();
}
