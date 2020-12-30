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
    if pdb.has_unit_cell() {
        let unit_cell = pdb.unit_cell();
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
        let m = scale.transformation().matrix();
        writer.write_fmt(format_args!(
            "SCALE1    {:10.6}{:10.6}{:10.6}     {:10.5}\nSCALE2    {:10.6}{:10.6}{:10.6}     {:10.5}\nSCALE3    {:10.6}{:10.6}{:10.6}     {:10.5}\n",
            m[0][0],
            m[0][1],
            m[0][2],
            m[0][3],
            m[1][0],
            m[1][1],
            m[1][2],
            m[1][3],
            m[2][0],
            m[2][1],
            m[2][2],
            m[2][3],
        )).unwrap();
    }

    // OrigX
    if let Some(origx) = &pdb.origx {
        let m = origx.transformation().matrix();
        writer.write_fmt(format_args!(
            "ORIGX1    {:10.6}{:10.6}{:10.6}     {:10.5}\nORIGX2    {:10.6}{:10.6}{:10.6}     {:10.5}\nORIGX3    {:10.6}{:10.6}{:10.6}     {:10.5}\n",
            m[0][0],
            m[0][1],
            m[0][2],
            m[0][3],
            m[1][0],
            m[1][1],
            m[1][2],
            m[1][3],
            m[2][0],
            m[2][1],
            m[2][2],
            m[2][3],
        )).unwrap();
    }

    // MtriX
    for mtrix in &pdb.mtrix {
        let m = mtrix.transformation().matrix();
        writer.write_fmt(format_args!(
            "MTRIX1 {:3}{:10.6}{:10.6}{:10.6}     {:10.5}    {}\nMTRIX2 {:3}{:10.6}{:10.6}{:10.6}     {:10.5}    {}\nMTRIX3 {:3}{:10.6}{:10.6}{:10.6}     {:10.5}    {}\n",
            mtrix.serial_number,
            m[0][0],
            m[0][1],
            m[0][2],
            m[0][3],
            if mtrix.contained {'1'} else {' '},
            mtrix.serial_number,
            m[1][0],
            m[1][1],
            m[1][2],
            m[1][3],
            if mtrix.contained {'1'} else {' '},
            mtrix.serial_number,
            m[2][0],
            m[2][1],
            m[2][2],
            m[2][3],
            if mtrix.contained {'1'} else {' '},
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
