use crate::structs::*;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

pub fn save(pdb: &PDB, filename: &str) -> Result<(), String> {
    let file = File::create(filename).expect("Could not open file");
    let mut writer = BufWriter::new(file);

    // Remarks
    for line in &pdb.remarks {
        writer
            .write_fmt(format_args!("REMARK {:3} {}\n", line.0, line.1))
            .unwrap();
    }

    // Cryst
    if let Some(unit_cell) = &pdb.unit_cell {
        let sym = if let Some(symmetry) = &pdb.symmetry {
            format!(
                "{} {}",
                symmetry.space_group(),
                symmetry
                    .symbols()
                    .iter()
                    .map(|a| format!("{}", a))
                    .collect::<Vec<String>>()
                    .join(" ")
            )
        } else {
            "P 1".to_string()
        };
        writer
            .write_fmt(format_args!(
                "CRYST1{:9.3}{:9.3}{:9.3}{:7.2}{:7.2}{:7.2} {}\n",
                unit_cell.a(),
                unit_cell.b(),
                unit_cell.c(),
                unit_cell.alpha(),
                unit_cell.beta(),
                unit_cell.gamma(),
                sym
            ))
            .unwrap();
    }

    // Scale
    if let Some(scale) = &pdb.scale {
        writer.write_fmt(format_args!(
            "SCALE1    {:10.6}{:10.6}{:10.6}     {:10.5}\nSCALE2    {:10.6}{:10.6}{:10.6}     {:10.5}\nSCALE3    {:10.6}{:10.6}{:10.6}     {:10.5}\n",
            scale.factors[0][0],
            scale.factors[0][1],
            scale.factors[0][2],
            scale.factors[0][3],
            scale.factors[1][0],
            scale.factors[1][1],
            scale.factors[1][2],
            scale.factors[1][3],
            scale.factors[2][0],
            scale.factors[2][1],
            scale.factors[2][2],
            scale.factors[2][3],
        )).unwrap();
    }

    // Models
    let multiple_models = pdb.models().size_hint().0 > 1;
    for model in pdb.models() {
        if multiple_models {
            writer
                .write_fmt(format_args!("MODEL        {}\n", model.serial_number()))
                .unwrap();
        }

        for atom in model.atoms() {
            writer
                .write_fmt(format_args!(
                    "ATOM  {:5} {:^4} {:4}{}{:4}    {:8.3}{:8.3}{:8.3}{:6.2}{:6.2}          {:>2}{:>2}\n",
                    atom.serial_number(),
                    atom.name(),
                    atom.residue().id(),
                    atom.residue().chain().id(),
                    atom.residue().serial_number(),
                    atom.pos().0,
                    atom.pos().1,
                    atom.pos().2,
                    atom.occupancy(),
                    atom.b_factor(),
                    atom.element(),
                    atom.charge(),
                ))
                .unwrap();
            if atom.anisotropic_temperature_factors().is_some() {
                writer
                    .write_fmt(format_args!(
                        "ANSIOU{:5} {:^4} {:4}{}{:4}  {:7}{:7}{:7}{:7}{:7}{:7}      {:>2}{:>2}\n",
                        atom.serial_number(),
                        atom.name(),
                        atom.residue().id(),
                        atom.residue().chain().id(),
                        atom.residue().serial_number(),
                        (atom.anisotropic_temperature_factors().unwrap()[0][0] * 10000.0) as isize,
                        (atom.anisotropic_temperature_factors().unwrap()[0][1] * 10000.0) as isize,
                        (atom.anisotropic_temperature_factors().unwrap()[0][2] * 10000.0) as isize,
                        (atom.anisotropic_temperature_factors().unwrap()[1][0] * 10000.0) as isize,
                        (atom.anisotropic_temperature_factors().unwrap()[1][1] * 10000.0) as isize,
                        (atom.anisotropic_temperature_factors().unwrap()[1][2] * 10000.0) as isize,
                        atom.element(),
                        atom.charge(),
                    ))
                    .unwrap();
            }
        }
        writer.write_fmt(format_args!("TER\n")).unwrap();
        for atom in model.hetero_atoms() {
            writer
                .write_fmt(format_args!(
                    "HETATM{:5} {:^4} {:4}{}{:4}    {:8.3}{:8.3}{:8.3}{:6.2}{:6.2}          {:>2}{:>2}\n",
                    atom.serial_number(),
                    atom.name(),
                    atom.residue().id(),
                    atom.residue().chain().id(),
                    atom.residue().serial_number(),
                    atom.pos().0,
                    atom.pos().1,
                    atom.pos().2,
                    atom.occupancy(),
                    atom.b_factor(),
                    atom.element(),
                    atom.charge(),
                ))
                .unwrap();
            if atom.anisotropic_temperature_factors().is_some() {
                writer
                    .write_fmt(format_args!(
                        "ANSIOU{:5} {:^4} {:4}{}{:4}  {:7}{:7}{:7}{:7}{:7}{:7}      {:>2}{:>2}\n",
                        atom.serial_number(),
                        atom.name(),
                        atom.residue().id(),
                        atom.residue().chain().id(),
                        atom.residue().serial_number(),
                        (atom.anisotropic_temperature_factors().unwrap()[0][0] * 10000.0) as isize,
                        (atom.anisotropic_temperature_factors().unwrap()[0][1] * 10000.0) as isize,
                        (atom.anisotropic_temperature_factors().unwrap()[0][2] * 10000.0) as isize,
                        (atom.anisotropic_temperature_factors().unwrap()[1][0] * 10000.0) as isize,
                        (atom.anisotropic_temperature_factors().unwrap()[1][1] * 10000.0) as isize,
                        (atom.anisotropic_temperature_factors().unwrap()[1][2] * 10000.0) as isize,
                        atom.element(),
                        atom.charge(),
                    ))
                    .unwrap();
            }
        }
    }

    writer.write_fmt(format_args!("END\n")).unwrap();

    writer.flush().unwrap();
    Ok(())
}
