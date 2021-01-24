use crate::error::*;
use crate::structs::*;
use crate::validate;
use crate::StrictnessLevel;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::iter;

/// Save the given PDB struct to the given file.
/// It validates the PDB. It fails if the validation fails with the given `level`.
/// If validation gives rise to problems use the `save_raw` function.
pub fn save(pdb: PDB, filename: &str, level: StrictnessLevel) -> Result<(), Vec<PDBError>> {
    let mut errors = validate(&pdb);
    for error in &errors {
        if error.fails(level) {
            return Err(errors);
        }
    }

    let file = match File::create(filename) {
        Ok(f) => f,
        Err(e) => {
            errors.push(PDBError::new(
                ErrorLevel::BreakingError,
                "Could not open file",
                "Could not open the file for writing, make sure you have permission for this file and no other program is currently using it.",
                Context::show(&e.to_string())
            ));
            return Err(errors);
        }
    };

    save_raw(&pdb, BufWriter::new(file), level);
    Ok(())
}

/// Save the given PDB struct to the given BufWriter.
/// It does not validate or renumber the PDB, so if that is needed that needs to be done in preparation.
/// It does change the output format based on the StrictnessLevel given.
///
/// ## Loose
/// * Does not pad all lines to 70 chars length
/// * Does not save the MASTER record
#[allow(clippy::unwrap_used)]
pub fn save_raw<T: Write>(pdb: &PDB, mut sink: BufWriter<T>, level: StrictnessLevel) {
    let mut finish_line = |mut line: String| {
        if level != StrictnessLevel::Loose && line.len() < 70 {
            let dif = 70 - line.len();
            line.reserve(dif);
            line.extend(iter::repeat(" ").take(dif));
        }
        sink.write_all(line.as_bytes()).unwrap();
        sink.write_all("\n".as_bytes()).unwrap();
    };
    macro_rules! write {
        ($($arg:tt)*) => {
            finish_line(format!($($arg)*));
        }
    }

    // Remarks
    for line in pdb.remarks() {
        write!("REMARK {:3} {}", line.0, line.1);
    }

    // MODRES
    for chain in pdb.chains() {
        for residue in chain.residues() {
            if let Some((std_name, comment)) = residue.modification() {
                write!(
                    "MODRES      {:3} {} {:4}  {:3}  {}",
                    residue.id(),
                    chain.id(),
                    residue.serial_number(),
                    std_name.iter().collect::<String>(),
                    comment
                );
            }
        }
    }

    // Cryst
    if pdb.has_unit_cell() {
        let unit_cell = pdb.unit_cell();
        let sym = if pdb.has_symmetry() {
            format!("{:10}{:3}", pdb.symmetry().symbol(), pdb.symmetry().z(),)
        } else {
            "P 1         1".to_string()
        };
        write!(
            "CRYST1{:9.3}{:9.3}{:9.3}{:7.2}{:7.2}{:7.2} {}",
            unit_cell.a(),
            unit_cell.b(),
            unit_cell.c(),
            unit_cell.alpha(),
            unit_cell.beta(),
            unit_cell.gamma(),
            sym
        );
    }

    // Scale
    if pdb.has_scale() {
        let m = pdb.scale().transformation().matrix();
        write!(
            "SCALE1    {:10.6}{:10.6}{:10.6}     {:10.5}",
            m[0][0],
            m[0][1], m[0][2], m[0][3],
        );
        write!(
            "SCALE2    {:10.6}{:10.6}{:10.6}     {:10.5}",
            m[1][0],
            m[1][1], m[1][2], m[1][3],
        );
        write!(
            "SCALE3    {:10.6}{:10.6}{:10.6}     {:10.5}",
            m[2][0],
            m[2][1], m[2][2], m[2][3],
        );
    }

    // OrigX
    if pdb.has_origx() {
        let m = pdb.origx().transformation().matrix();
        write!(
            "ORIGX1    {:10.6}{:10.6}{:10.6}     {:10.5}",
            m[0][0],
            m[0][1], m[0][2], m[0][3],
        );
        write!(
            "ORIGX2    {:10.6}{:10.6}{:10.6}     {:10.5}",
            m[1][0],
            m[1][1], m[1][2], m[1][3],
        );
        write!(
            "ORIGX3    {:10.6}{:10.6}{:10.6}     {:10.5}",
            m[2][0],
            m[2][1], m[2][2], m[2][3],
        );
    }

    // MtriX
    for mtrix in pdb.mtrix() {
        let m = mtrix.transformation().matrix();
        write!(
            "MTRIX1 {:3}{:10.6}{:10.6}{:10.6}     {:10.5}    {}",
            mtrix.serial_number,
            m[0][0],
            m[0][1],
            m[0][2],
            m[0][3],
            if mtrix.contained { '1' } else { ' ' },
        );
        write!(
            "MTRIX2 {:3}{:10.6}{:10.6}{:10.6}     {:10.5}    {}",
            mtrix.serial_number,
            m[1][0],
            m[1][1],
            m[1][2],
            m[1][3],
            if mtrix.contained { '1' } else { ' ' },
        );
        write!(
            "MTRIX3 {:3}{:10.6}{:10.6}{:10.6}     {:10.5}    {}",
            mtrix.serial_number,
            m[2][0],
            m[2][1],
            m[2][2],
            m[2][3],
            if mtrix.contained { '1' } else { ' ' },
        );
    }

    // Models
    let multiple_models = pdb.models().size_hint().0 > 1;
    for model in pdb.models() {
        if multiple_models {
            write!("MODEL        {}", model.serial_number());
        }

        for chain in model.chains() {
            for residue in chain.residues() {
                for atom in residue.atoms() {
                    write!(
                        "ATOM  {:5} {:^4} {:4}{}{:4}    {:8.3}{:8.3}{:8.3}{:6.2}{:6.2}          {:>2}{}",
                        atom.serial_number(),
                        atom.name(),
                        residue.id(),
                        chain.id(),
                        residue.serial_number(),
                        atom.pos().0,
                        atom.pos().1,
                        atom.pos().2,
                        atom.occupancy(),
                        atom.b_factor(),
                        atom.element(),
                        atom.pdb_charge(),
                    );
                    #[allow(clippy::cast_possible_truncation)]
                    if atom.anisotropic_temperature_factors().is_some() {
                        write!(
                            "ANSIOU{:5} {:^4} {:4}{}{:4}  {:7}{:7}{:7}{:7}{:7}{:7}      {:>2}{}",
                            atom.serial_number(),
                            atom.name(),
                            residue.id(),
                            chain.id(),
                            residue.serial_number(),
                            (atom.anisotropic_temperature_factors().unwrap()[0][0] * 10000.0)
                                as isize,
                            (atom.anisotropic_temperature_factors().unwrap()[0][1] * 10000.0)
                                as isize,
                            (atom.anisotropic_temperature_factors().unwrap()[0][2] * 10000.0)
                                as isize,
                            (atom.anisotropic_temperature_factors().unwrap()[1][0] * 10000.0)
                                as isize,
                            (atom.anisotropic_temperature_factors().unwrap()[1][1] * 10000.0)
                                as isize,
                            (atom.anisotropic_temperature_factors().unwrap()[1][2] * 10000.0)
                                as isize,
                            atom.element(),
                            atom.pdb_charge(),
                        );
                    }
                }
            }
            let last_atom = chain.atoms().nth_back(0).unwrap();
            let last_residue = chain.residues().nth_back(0).unwrap();
            write!(
                "TER{:5}      {:3} {}{:4}",
                last_atom.serial_number(),
                last_residue.id(),
                chain.id(),
                last_residue.serial_number()
            );
        }
        for chain in model.hetero_chains() {
            for residue in chain.residues() {
                for atom in residue.atoms() {
                    write!(
                        "HETATM{:5} {:^4} {:4}{}{:4}    {:8.3}{:8.3}{:8.3}{:6.2}{:6.2}          {:>2}{}",
                        atom.serial_number(),
                        atom.name(),
                        residue.id(),
                        chain.id(),
                        residue.serial_number(),
                        atom.pos().0,
                        atom.pos().1,
                        atom.pos().2,
                        atom.occupancy(),
                        atom.b_factor(),
                        atom.element(),
                        atom.pdb_charge()
                    );
                    #[allow(clippy::cast_possible_truncation)]
                    if atom.anisotropic_temperature_factors().is_some() {
                        write!(
                            "ANSIOU{:5} {:^4} {:4}{}{:4}  {:7}{:7}{:7}{:7}{:7}{:7}      {:>2}{}",
                            atom.serial_number(),
                            atom.name(),
                            residue.id(),
                            chain.id(),
                            residue.serial_number(),
                            (atom.anisotropic_temperature_factors().unwrap()[0][0] * 10000.0)
                                as isize,
                            (atom.anisotropic_temperature_factors().unwrap()[0][1] * 10000.0)
                                as isize,
                            (atom.anisotropic_temperature_factors().unwrap()[0][2] * 10000.0)
                                as isize,
                            (atom.anisotropic_temperature_factors().unwrap()[1][0] * 10000.0)
                                as isize,
                            (atom.anisotropic_temperature_factors().unwrap()[1][1] * 10000.0)
                                as isize,
                            (atom.anisotropic_temperature_factors().unwrap()[1][2] * 10000.0)
                                as isize,
                            atom.element(),
                            atom.pdb_charge()
                        );
                    }
                }
            }
        }

        if multiple_models {
            write!("ENDMDL");
        }
    }
    if level != StrictnessLevel::Loose {
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
        write!(
            "MASTER    {:5}{:5}{:5}{:5}{:5}{:5}{:5}{:5}{:5}{:5}{:5}{:5}",
            pdb.remark_count(),
            0, //defined to be empty
            0, //numHet
            0, //numHelix
            0, //numSheet
            0, //numTurn (deprecated)
            0, //numSite
            xform,
            pdb.total_atom_count(),
            pdb.model_count(),
            0, //numConnect
            0, //numSeq
        );
    }
    write!("END");

    sink.flush().unwrap();
}
