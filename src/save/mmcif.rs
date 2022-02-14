use crate::error::*;
use crate::structs::*;
use crate::validate;
use crate::StrictnessLevel;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

/// Save the given PDB struct to the given file as mmCIF or PDBx.
/// # Errors
/// It validates the PDB. It fails if the validation fails with the given `level`, or if the file could not be opened.
/// If validation gives rise to problems, use the `save_raw` function.
pub fn save_mmcif(
    pdb: &PDB,
    filename: impl AsRef<str>,
    level: StrictnessLevel,
) -> Result<(), Vec<PDBError>> {
    let filename = filename.as_ref();
    let mut errors = validate(pdb);
    for error in &errors {
        if error.fails(level) {
            return Err(errors);
        }
    }

    let file = match File::create(filename) {
        Ok(f) => f,
        Err(_e) => {
            errors.push(PDBError::new(
                ErrorLevel::BreakingError,
                "Could not open file",
                "Could not open the file for writing, make sure you have permission for this file and no other program is currently using it.",
                Context::show(filename)
            ));
            return Err(errors);
        }
    };

    save_mmcif_raw(pdb, BufWriter::new(file));
    Ok(())
}

/// Save the given PDB struct to the given BufWriter.
/// It does not validate or renumber the PDB, so if that is needed that needs to be done in preparation.
/// It does change the output format based on the StrictnessLevel given.
///
/// ## Warning
/// This function is unstable and unfinished!
#[allow(clippy::unwrap_used)]
pub fn save_mmcif_raw<T: Write>(pdb: &PDB, mut sink: BufWriter<T>) {
    /// Write a piece of text to the file, has the same structure as format!
    macro_rules! write {
        ($($arg:tt)*) => {
            sink.write_fmt(format_args!($($arg)*)).unwrap();
            sink.write_all(b"\n").unwrap();
        }
    }

    let empty = "?".to_string();
    let name = pdb.identifier.as_ref().unwrap_or(&empty);

    // Header
    write!(
        "data_{}
# 
_entry.id   {} 
# 
_audit_conform.dict_name       mmcif_pdbx.dic 
_audit_conform.dict_version    5.338 
_audit_conform.dict_location   http://mmcif.pdb.org/dictionaries/ascii/mmcif_pdbx.dic",
        name,
        name
    );

    // Cryst
    if let Some(unit_cell) = &pdb.unit_cell {
        write!(
            "# Unit cell definition
_cell.entry_id           {}
_cell.length_a           {} 
_cell.length_b           {} 
_cell.length_c           {} 
_cell.angle_alpha        {}
_cell.angle_beta         {}
_cell.angle_gamma        {}
_cell.Z_PDB              {}",
            name,
            unit_cell.a(),
            unit_cell.b(),
            unit_cell.c(),
            unit_cell.alpha(),
            unit_cell.beta(),
            unit_cell.gamma(),
            if let Some(symmetry) = &pdb.symmetry {
                symmetry.z().to_string()
            } else {
                "?".to_owned()
            }
        );
    }

    if let Some(symmetry) = &pdb.symmetry {
        write!(
            "# Space group definition
_symmetry.entry_id                         {} 
_symmetry.space_group_name_H-M             '{}' 
_symmetry.pdbx_full_space_group_name_H-M   '{}'
_symmetry.Int_Tables_number                {}",
            name,
            symmetry.herman_mauguin_symbol(),
            symmetry.herman_mauguin_symbol(),
            symmetry.index()
        );
    }

    let anisou = pdb
        .atoms()
        .any(|a| a.anisotropic_temperature_factors().is_some());
    write!(
        "loop_
_atom_site.group_PDB 
_atom_site.id 
_atom_site.type_symbol 
_atom_site.label_atom_id 
_atom_site.label_alt_id 
_atom_site.label_comp_id 
_atom_site.label_asym_id 
_atom_site.label_entity_id 
_atom_site.label_seq_id 
_atom_site.pdbx_PDB_ins_code 
_atom_site.Cartn_x 
_atom_site.Cartn_y 
_atom_site.Cartn_z 
_atom_site.occupancy 
_atom_site.B_iso_or_equiv 
_atom_site.pdbx_formal_charge 
_atom_site.pdbx_PDB_model_num{}",
        if anisou {
            "
_atom_site.aniso_U[1][1]
_atom_site.aniso_U[1][2]
_atom_site.aniso_U[1][3]
_atom_site.aniso_U[2][1]
_atom_site.aniso_U[2][2]
_atom_site.aniso_U[2][3]
_atom_site.aniso_U[3][1]
_atom_site.aniso_U[3][2]
_atom_site.aniso_U[3][3]"
        } else {
            ""
        }
    );

    let mut lines = Vec::new();

    for model in pdb.models() {
        let mut chain_index = 0;
        for chain in model.chains() {
            chain_index += 1;
            for residue in chain.residues() {
                for conformer in residue.conformers() {
                    for atom in conformer.atoms() {
                        let mut data = vec![
                            (if atom.hetero() { "HETATM" } else { "ATOM" }).to_string(), // ATOM or HETATM
                            atom.serial_number().to_string(), // Serial number
                            atom.element()
                                .map_or_else(|| "", Element::symbol)
                                .to_string(), // Element
                            atom.name().to_string(),          // Name
                            conformer.alternative_location().unwrap_or(".").to_string(), // Alternative location
                            conformer.name().to_string(), // Residue name
                            chain.id().to_string(),       // Chain name
                            chain_index.to_string(),      // Entity ID, using chain serial number
                            residue.serial_number().to_string(), // Residue serial number
                            residue.insertion_code().unwrap_or(".").to_string(), // Insertion code
                            print_float(atom.x()),        // X
                            print_float(atom.y()),        // Y
                            print_float(atom.z()),        // Z
                            print_float(atom.occupancy()), // OCC/Q
                            print_float(atom.b_factor()), // B
                            atom.charge().to_string(),    // Charge
                            model.serial_number().to_string(), // Model serial number
                        ];
                        if anisou {
                            if let Some(matrix) = atom.anisotropic_temperature_factors() {
                                data.extend(vec![
                                    print_float(matrix[0][0]),
                                    print_float(matrix[0][1]),
                                    print_float(matrix[0][2]),
                                    print_float(matrix[1][0]),
                                    print_float(matrix[1][1]),
                                    print_float(matrix[1][2]),
                                    print_float(matrix[2][0]),
                                    print_float(matrix[2][1]),
                                    print_float(matrix[2][2]),
                                ]);
                            } else {
                                data.extend(vec![
                                    ".".to_string(),
                                    ".".to_string(),
                                    ".".to_string(),
                                    ".".to_string(),
                                    ".".to_string(),
                                    ".".to_string(),
                                    ".".to_string(),
                                    ".".to_string(),
                                    ".".to_string(),
                                ]);
                            }
                        }

                        lines.push(data);
                    }
                }
            }
        }
    }
    if !lines.is_empty() {
        // Now align the table
        let mut sizes = vec![1; lines[0].len()];
        for line in &lines {
            for index in 0..line.len() {
                sizes[index] = std::cmp::max(sizes[index], line[index].len());
            }
        }

        // Now write the table
        for line in lines {
            let mut output = String::new();
            output.push_str(&line[0]);
            output.push_str(&" ".repeat(sizes[0] - line[0].len()));
            for index in 1..line.len() {
                output.push(' ');
                if line[index].trim() != "" {
                    output.push_str(&line[index]);
                    output.push_str(&" ".repeat(sizes[index] - line[index].len()));
                } else {
                    output.push('?');
                    output.push_str(&" ".repeat(sizes[index] - 1));
                }
            }
            output.push('\n');
            sink.write_all(output.as_bytes()).unwrap();
        }
    }

    write!("#");

    sink.flush().unwrap();
}

/// Print a floating point with at least 1 decimal place and at max 5 decimals
#[allow(clippy::cast_possible_truncation)]
fn print_float(num: f64) -> String {
    let rounded = (num * 100000.).round() / 100000.;
    if (rounded.round() - rounded).abs() < std::f64::EPSILON {
        format!("{}.0", rounded.trunc() as isize)
    } else {
        format!("{}", rounded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::excessive_precision, clippy::print_literal)]
    fn test_print_float() {
        assert_eq!(print_float(1.), "1.0".to_string());
        assert_eq!(print_float(128734.), "128734.0".to_string());
        assert_eq!(print_float(0.1), "0.1".to_string());
        assert_eq!(print_float(1.015), "1.015".to_string());
        assert_eq!(print_float(2.015), "2.015".to_string());
        assert_eq!(print_float(1.4235263), "1.42353".to_string());
        println!("{}", 235617341053.235611341053); // Already printed as 235617341053.23563
        assert_eq!(
            print_float(235617341053.235611341053),
            "235617341053.23563".to_string()
        );
        println!("{}", 23561753.235617341053); // Printed as 23561753.23561734
        assert_eq!(
            print_float(23561753.235617341053),
            "23561753.23562".to_string()
        );
    }
}
