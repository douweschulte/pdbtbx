use crate::structs::*;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

/// Save the given PDB struct to the given file.
/// It does not validate or renumber the PDB, so if that is needed that needs to be done in preparation.
pub fn save(pdb: &PDB, filename: &str) -> Result<(), String> {
    let file = match File::create(filename) {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };
    let mut writer = BufWriter::new(file);

    // Remarks
    for line in pdb.remarks() {
        writer
            .write_fmt(format_args!("REMARK {:3} {}\n", line.0, line.1))
            .unwrap();
    }

    // Cryst
    if pdb.has_unit_cell() {
        let unit_cell = pdb.unit_cell();
        let sym = if pdb.has_symmetry() {
            format!("{:10}{:3}", pdb.symmetry().symbol(), pdb.symmetry().z(),)
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
    if pdb.has_scale() {
        let m = pdb.scale().transformation().matrix();
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
    if pdb.has_origx() {
        let m = pdb.origx().transformation().matrix();
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
    for mtrix in pdb.mtrix() {
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

        for chain in model.chains() {
            for residue in chain.residues() {
                for atom in residue.atoms() {
                    writer
                .write_fmt(format_args!(
                    "ATOM  {:5} {:^4} {:4}{}{:4}    {:8.3}{:8.3}{:8.3}{:6.2}{:6.2}          {:>2}{}\n",
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
                ))
                .unwrap();
                    if atom.anisotropic_temperature_factors().is_some() {
                        writer
                            .write_fmt(format_args!(
                        "ANSIOU{:5} {:^4} {:4}{}{:4}  {:7}{:7}{:7}{:7}{:7}{:7}      {:>2}{}\n",
                        atom.serial_number(),
                        atom.name(),
                        residue.id(),
                        chain.id(),
                        residue.serial_number(),
                        (atom.anisotropic_temperature_factors().unwrap()[0][0] * 10000.0) as isize,
                        (atom.anisotropic_temperature_factors().unwrap()[0][1] * 10000.0) as isize,
                        (atom.anisotropic_temperature_factors().unwrap()[0][2] * 10000.0) as isize,
                        (atom.anisotropic_temperature_factors().unwrap()[1][0] * 10000.0) as isize,
                        (atom.anisotropic_temperature_factors().unwrap()[1][1] * 10000.0) as isize,
                        (atom.anisotropic_temperature_factors().unwrap()[1][2] * 10000.0) as isize,
                        atom.element(),
                        atom.pdb_charge(),
                    ))
                            .unwrap();
                    }
                }
            }
            let last_atom = chain.atoms().nth_back(0).unwrap();
            let last_residue = chain.residues().nth_back(0).unwrap();
            writer
                .write_fmt(format_args!(
                    "TER{:5}      {:3} {}{:4} \n",
                    last_atom.serial_number(),
                    last_residue.id(),
                    chain.id(),
                    last_residue.serial_number()
                ))
                .unwrap();
        }
        for chain in model.hetero_chains() {
            for residue in chain.residues() {
                for atom in residue.atoms() {
                    writer
                .write_fmt(format_args!(
                    "HETATM{:5} {:^4} {:4}{}{:4}    {:8.3}{:8.3}{:8.3}{:6.2}{:6.2}          {:>2}{}\n",
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
                ))
                .unwrap();
                    if atom.anisotropic_temperature_factors().is_some() {
                        writer
                            .write_fmt(format_args!(
                        "ANSIOU{:5} {:^4} {:4}{}{:4}  {:7}{:7}{:7}{:7}{:7}{:7}      {:>2}{}\n",
                        atom.serial_number(),
                        atom.name(),
                        residue.id(),
                        chain.id(),
                        residue.serial_number(),
                        (atom.anisotropic_temperature_factors().unwrap()[0][0] * 10000.0) as isize,
                        (atom.anisotropic_temperature_factors().unwrap()[0][1] * 10000.0) as isize,
                        (atom.anisotropic_temperature_factors().unwrap()[0][2] * 10000.0) as isize,
                        (atom.anisotropic_temperature_factors().unwrap()[1][0] * 10000.0) as isize,
                        (atom.anisotropic_temperature_factors().unwrap()[1][1] * 10000.0) as isize,
                        (atom.anisotropic_temperature_factors().unwrap()[1][2] * 10000.0) as isize,
                        atom.element(),
                        atom.pdb_charge()
                    ))
                            .unwrap();
                    }
                }
            }
        }
    }

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
    writer
        .write_fmt(format_args!(
            "MASTER    {:5}{:5}{:5}{:5}{:5}{:5}{:5}{:5}{:5}{:5}{:5}{:5}\n",
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
        ))
        .unwrap();
    writer.write_fmt(format_args!("END\n")).unwrap();

    writer.flush().unwrap();
    Ok(())
}
