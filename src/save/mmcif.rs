use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

use context_error::FullErrorContent;
use context_error::{BoxedError, Context, CreateError, ErrorKind};
#[cfg(feature = "compression")]
use flate2::{write::GzEncoder, Compression};

use crate::structs::*;
use crate::validate;
use crate::ErrorLevel;
use crate::StrictnessLevel;

/// Save the given PDB struct to the given file as mmCIF or PDBx.
/// # Errors
/// It validates the PDB. It fails if the validation fails with the given `level`, or if the file could not be opened.
/// If validation gives rise to problems, use the `save_raw` function.
pub fn save_mmcif(
    pdb: &PDB,
    filename: impl AsRef<str>,
    level: StrictnessLevel,
) -> Result<(), Vec<BoxedError<'static, ErrorLevel>>> {
    save_mmcif_(pdb, filename, level, BufWriter::new)
}

/// Save the given PDB struct to the given file as mmCIF or PDBx and compresses to .gz
/// # Errors
/// It validates the PDB. It fails if the validation fails with the given `level`, or if the file could not be opened.
/// If validation gives rise to problems, use the `save_raw` function.
#[cfg(feature = "compression")]
pub fn save_mmcif_gz(
    pdb: &PDB,
    filename: impl AsRef<str>,
    level: StrictnessLevel,
    compression_level: Option<Compression>,
) -> Result<(), Vec<BoxedError<'static, ErrorLevel>>> {
    save_mmcif_(pdb, filename, level, |file| {
        BufWriter::new(GzEncoder::new(file, compression_level.unwrap_or_default()))
    })
}

/// Generic function to save the given PDB struct to the given file as mmCIF or PDBx,
/// to some writer function, e.g. a `GzEncoder` or `BufWriter`.
fn save_mmcif_<T, W>(
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
    save_mmcif_raw(pdb, writer);

    Ok(())
}

/// Save the given PDB struct to the given `BufWriter`.
/// It does not validate or renumber the PDB, so if that is needed that needs to be done in preparation.
/// It does change the output format based on the `StrictnessLevel` given.
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
            pdb.symmetry
                .as_ref()
                .map_or_else(|| "?".to_owned(), |symmetry| symmetry.z().to_string())
        );
    }

    // Scale
    if let Some(scale) = &pdb.scale {
        let ma = scale.matrix();
        write!(
            "# Scale definition
            _atom_sites.entry_id                   '{}'
_atom_sites.Cartn_transf_matrix[1][1]  {}
_atom_sites.Cartn_transf_matrix[1][2]  {}
_atom_sites.Cartn_transf_matrix[1][3]  {}
_atom_sites.Cartn_transf_matrix[2][1]  {}
_atom_sites.Cartn_transf_matrix[2][2]  {}
_atom_sites.Cartn_transf_matrix[2][3]  {}
_atom_sites.Cartn_transf_matrix[3][1]  {}
_atom_sites.Cartn_transf_matrix[3][2]  {}
_atom_sites.Cartn_transf_matrix[3][3]  {}
_atom_sites.Cartn_transf_vector[1]     {}
_atom_sites.Cartn_transf_vector[2]     {}
_atom_sites.Cartn_transf_vector[3]     {}",
            name,
            ma[0][0],
            ma[0][1],
            ma[0][2],
            ma[1][0],
            ma[1][1],
            ma[1][2],
            ma[2][0],
            ma[2][1],
            ma[2][2],
            ma[0][3],
            ma[1][3],
            ma[2][3],
        );
    }

    // OrigX
    if let Some(origx) = &pdb.origx {
        let ma = origx.matrix();
        write!(
            "# OrigX definition
_database_PDB_matrix.entry_id                   '{}'
_database_PDB_matrix.origx[1][1]  {}
_database_PDB_matrix.origx[1][2]  {}
_database_PDB_matrix.origx[1][3]  {}
_database_PDB_matrix.origx[2][1]  {}
_database_PDB_matrix.origx[2][2]  {}
_database_PDB_matrix.origx[2][3]  {}
_database_PDB_matrix.origx[3][1]  {}
_database_PDB_matrix.origx[3][2]  {}
_database_PDB_matrix.origx[3][3]  {}
_database_PDB_matrix.origx_vector[1]     {}
_database_PDB_matrix.origx_vector[2]     {}
_database_PDB_matrix.origx_vector[3]     {}",
            name,
            ma[0][0],
            ma[0][1],
            ma[0][2],
            ma[1][0],
            ma[1][1],
            ma[1][2],
            ma[2][0],
            ma[2][1],
            ma[2][2],
            ma[0][3],
            ma[1][3],
            ma[2][3],
        );
    }

    // MtriX
    for mtrix in pdb.mtrix() {
        let ma = mtrix.transformation.matrix();
        write!(
            r"# OrigX definition
_struct_ncs_oper.id            '{}'
_struct_ncs_oper.code          {}
_struct_ncs_oper.matrix[1][1]  {}
_struct_ncs_oper.matrix[1][2]  {}
_struct_ncs_oper.matrix[1][3]  {}
_struct_ncs_oper.matrix[2][1]  {}
_struct_ncs_oper.matrix[2][2]  {}
_struct_ncs_oper.matrix[2][3]  {}
_struct_ncs_oper.matrix[3][1]  {}
_struct_ncs_oper.matrix[3][2]  {}
_struct_ncs_oper.matrix[3][3]  {}
_struct_ncs_oper.vector[1]     {}
_struct_ncs_oper.vector[2]     {}
_struct_ncs_oper.vector[3]     {}",
            mtrix.serial_number,
            if mtrix.contained {
                "given"
            } else {
                "generated"
            },
            ma[0][0],
            ma[0][1],
            ma[0][2],
            ma[1][0],
            ma[1][1],
            ma[1][2],
            ma[2][0],
            ma[2][1],
            ma[2][2],
            ma[0][3],
            ma[1][3],
            ma[2][3],
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
_atom_site.auth_asym_id
_atom_site.label_entity_id
_atom_site.label_seq_id
_atom_site.auth_seq_id
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
            for (residue_index, residue) in chain.residues().enumerate() {
                for conformer in residue.conformers() {
                    for atom in conformer.atoms() {
                        let mut data = vec![
                            (if atom.hetero() { "HETATM" } else { "ATOM" }).to_string(), // ATOM or HETATM
                            atom.id().to_string(),                                       // ID
                            atom.element()
                                .map_or_else(|| "", Element::symbol)
                                .to_string(), // Element
                            atom.name().to_string(),                                     // Name
                            conformer.alternative_location().unwrap_or(".").to_string(), // Alternative location
                            conformer.name().to_string(), // Residue name
                            number_to_base26(chain_index), // Label Chain name, defined to be without gaps
                            chain.id().to_string(),        // Auth Chain name
                            chain_index.to_string(),       // Entity ID, using chain serial number
                            (residue_index + 1).to_string(), // `label_seq_id` defined to be [1-N] where N is the index
                            residue.serial_number().to_string(), // Residue serial number
                            residue.insertion_code().unwrap_or(".").to_string(), // Insertion code
                            print_float(atom.x()),           // X
                            print_float(atom.y()),           // Y
                            print_float(atom.z()),           // Z
                            print_float(atom.occupancy()),   // OCC/Q
                            print_float(atom.b_factor()),    // B
                            atom.charge().to_string(),       // Charge
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
                if line[index].trim() == "" {
                    output.push('?');
                    output.push_str(&" ".repeat(sizes[index] - 1));
                } else {
                    output.push_str(&line[index]);
                    output.push_str(&" ".repeat(sizes[index] - line[index].len()));
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
    let rounded = (num * 100_000.).round() / 100_000.;
    if (rounded.round() - rounded).abs() < f64::EPSILON {
        format!("{}.0", rounded.trunc() as isize)
    } else {
        format!("{rounded}")
    }
}

#[cfg(test)]
#[allow(clippy::print_stdout)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::excessive_precision, clippy::print_literal)]
    fn test_print_float() {
        assert_eq!(print_float(1.), "1.0".to_string());
        assert_eq!(print_float(128_734.), "128734.0".to_string());
        assert_eq!(print_float(0.1), "0.1".to_string());
        assert_eq!(print_float(1.015), "1.015".to_string());
        assert_eq!(print_float(2.015), "2.015".to_string());
        assert_eq!(print_float(1.423_526_3), "1.42353".to_string());
        println!("{}", 235_617_341_053.235_611_341_053); // Already printed as 235617341053.23563
        assert_eq!(
            print_float(235_617_341_053.235_611_341_053),
            "235617341053.23563".to_string()
        );
        println!("{}", 23_561_753.235_617_341_053); // Printed as 23561753.23561734
        assert_eq!(
            print_float(23_561_753.235_617_341_053),
            "23561753.23562".to_string()
        );
    }
}
