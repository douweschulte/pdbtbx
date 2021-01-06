use rust_pdb::*;
use std::path::Path;
use std::time::Instant;
use std::{env, fs};

#[test]
fn run_pdbs() {
    let current_dir = env::current_dir().unwrap();
    let pdb_dir = current_dir.as_path().join(Path::new("example-pdbs"));
    let dump_dir = current_dir.as_path().join(Path::new("dump"));
    let _ = fs::create_dir(dump_dir.clone());
    println!("{:?}", pdb_dir);

    for entry in fs::read_dir(pdb_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let metadata = fs::metadata(&path).unwrap();
        if metadata.is_file() {
            do_someting(
                &path.clone().into_os_string().into_string().unwrap(),
                &dump_dir
                    .join(Path::new(path.file_name().unwrap()))
                    .into_os_string()
                    .into_string()
                    .unwrap(),
            );
        }
    }
}

fn do_someting(file: &str, output: &str) {
    println!("Working on file: {}", file);
    let now = Instant::now();

    let (mut pdb, errors) = parse(file).unwrap();

    let time = now.elapsed();

    let mut stop = false;

    for error in errors {
        println!("{}", error);
        if error.level() == ErrorLevel::BreakingError {
            stop = true;
        }
    }

    if stop {
        panic!("Stopped execution because of previous error message(s).")
    }

    println!(
        "Found {} atoms, in {} residues, in {} chains, in {} models it all took {} ms",
        pdb.total_atom_count(),
        pdb.total_residue_count(),
        pdb.total_chain_count(),
        pdb.model_count(),
        time.as_millis()
    );

    println!("PDB parsed");

    let mut avg = 0.0;
    let mut total_back = 0;
    let mut avg_back = 0.0;
    let mut total_side = 0;
    let mut avg_side = 0.0;

    println!("Set values");

    for residue in pdb.residues() {
        for atom in residue.atoms() {
            avg += atom.b_factor();
            if residue.amino_acid() && atom.backbone() {
                total_back += 1;
                avg_back += atom.b_factor();
            } else {
                total_side += 1;
                avg_side += atom.b_factor();
            }
        }
    }

    println!("Counted for averages");

    avg = avg / ((total_back + total_side) as f64);
    avg_back = avg_back / (total_back as f64);
    avg_side = avg_side / (total_side as f64);

    println!("Found averages");

    println!(
        "Average B factor: Total: {:.3}, Backbone: {:.3}, Sidechains: {:.3}",
        avg, avg_back, avg_side
    );

    pdb.renumber();
    save(&pdb, output).expect("Save not successful");
}
