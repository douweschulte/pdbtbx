use super::structs::*;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

pub fn save(pdb: &PDB, filename: &str) -> Result<(), String> {
    let file = File::create(filename).expect("Could not open file");
    let mut writer = BufWriter::new(file);

    // Remarks
    for line in &pdb.remarks {
        writer.write_fmt(format_args!("REMARK{}\n", line)).unwrap();
    }

    // Cryst -- extend with symmetry
    if let Some(unit_cell) = &pdb.unit_cell {
        writer
            .write_fmt(format_args!(
                "CRYST1{:9.3}{:9.3}{:9.3}{:7.2}{:7.2}{:7.2}",
                unit_cell.a(),
                unit_cell.b(),
                unit_cell.c(),
                unit_cell.alpha(),
                unit_cell.beta(),
                unit_cell.gamma(),
            ))
            .unwrap();
    }

    // Scale

    // Models

    writer.flush().unwrap();
    Ok(())
}
