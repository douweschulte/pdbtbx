//! Benchmark to test the speed of opening files

use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::time::{Duration, Instant};

use pdbtbx::*;

fn main() {
    // Setup the data needed
    let pdb_names = vec![
        ("small", "example-pdbs/1ubq.pdb"),
        ("medium", "example-pdbs/1yyf.pdb"),
        ("big", "example-pdbs/pTLS-6484.pdb"),
        ("huge", "example-pdbs/large.pdb"),
        // ("extreme", "example-pdbs/1htq.pdb"),
    ];
    let mmcif_names = vec![
        ("small", "example-pdbs/1ubq.cif"),
        ("medium", "example-pdbs/1yyf.cif"),
        ("big", "example-pdbs/pTLS-6484.cif"),
    ];
    let mut models = Vec::with_capacity(pdb_names.len());
    let parser = ReadOptions::default()
        .set_level(StrictnessLevel::Loose)
        .set_format(Format::Pdb)
        .clone();
    for (name, path) in &pdb_names {
        models.push((*name, parser.read(path).unwrap().0));
    }
    let mut results = Vec::new();

    // Do the benchmarking
    results.extend(measure_multiple(bench_open, &pdb_names, "Open PDB"));
    results.extend(measure_multiple(bench_open, &mmcif_names, "Open mmCIF"));
    results.extend(measure_multiple(
        bench_transformation,
        &models,
        "Transformation",
    ));
    results.extend(measure_multiple(bench_remove, &models, "Remove"));
    results.extend(measure_multiple(bench_iteration, &models, "Iteration"));
    results.extend(measure_multiple(bench_validate, &models, "Validate"));
    results.extend(measure_multiple(bench_renumber, &models, "Renumber"));
    results.extend(measure_multiple(bench_clone, &models, "Clone"));
    results.extend(measure_multiple(bench_save_pdb, &models, "Save PDB"));
    results.extend(measure_multiple(bench_save_mmcif, &models, "Save mmCIF"));

    // Save the results to a csv
    let file = File::create("dump/benchmark_results.csv").unwrap();
    let mut sink = BufWriter::new(file);
    sink.write_all("Name,Average(ns),StandardDeviation(ns),Runs\n".as_bytes())
        .unwrap();
    for item in results {
        sink.write_fmt(format_args!(
            "{},{},{},{}\n",
            item.0, item.1, item.2, item.3
        ))
        .unwrap();
    }
    sink.flush().unwrap();
}

fn bench_open(filename: &str) {
    let (_pdb, _errors) = ReadOptions::default()
        .set_level(StrictnessLevel::Loose)
        .read(filename)
        .unwrap();
}

fn bench_transformation(mut pdb: PDB) {
    let transformation = TransformationMatrix::rotation_x(90.0);
    pdb.apply_transformation(&transformation);
}

fn bench_remove(mut pdb: PDB) {
    pdb.remove_atoms_by(|atom| atom.serial_number() % 2 == 0);
}

fn bench_iteration(pdb: PDB) {
    let mut _average = 0.0;
    for atom in pdb.atoms() {
        _average += atom.b_factor();
    }
    _average /= pdb.atom_count() as f64;
}

fn bench_validate(pdb: PDB) {
    let _ = validate(&pdb);
}

fn bench_renumber(mut pdb: PDB) {
    pdb.renumber();
}

fn bench_clone(pdb: PDB) {
    let _copy = pdb;
}

fn bench_save_pdb(pdb: PDB) {
    save(&pdb, "dump/dump.pdb", StrictnessLevel::Loose).unwrap();
}

fn bench_save_mmcif(pdb: PDB) {
    save(&pdb, "dump/dump.cif", StrictnessLevel::Loose).unwrap();
}

fn measure_multiple<T: Clone>(
    function: fn(T),
    subjects: &[(&str, T)],
    description: &str,
) -> Vec<(String, u128, u128, usize)> {
    let mut output = Vec::with_capacity(subjects.len());
    for (name, item) in subjects {
        output.push(measure(function, item, &format!("{description} - {name}")));
    }
    output
}

fn measure<T: Clone>(
    function: fn(T),
    subject: &T,
    description: &str,
) -> (String, u128, u128, usize) {
    let mut times = Vec::new();
    function(subject.clone());
    let start = Instant::now();
    let mut now;

    for _ in 0..5 {
        let clone = subject.clone();
        now = Instant::now();
        function(clone);
        times.push(now.elapsed());
    }

    let average = start.elapsed().checked_div(5).unwrap();

    // Lets run for 3 more seconds, including cloning of the subject
    for _ in 0..3_000_000_000 / average.as_nanos() {
        let clone = subject.clone();
        now = Instant::now();
        function(clone);
        times.push(now.elapsed());
    }

    let average = times
        .iter()
        .fold(Duration::new(0, 0), |total, item| {
            total.checked_add(*item).unwrap()
        })
        .checked_div(times.len() as u32)
        .unwrap();

    let mut deviation = 0.0;
    for run in &times {
        let difference = run.as_nanos() as f64 - average.as_nanos() as f64;
        deviation += difference * difference;
    }
    deviation /= times.len() as f64;
    deviation = deviation.sqrt();

    (
        description.to_string(),
        average.as_nanos(),
        deviation as u128,
        times.len(),
    )
}
