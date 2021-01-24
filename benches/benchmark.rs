use pdbtbx::*;
use std::cmp::min;
use std::time::{Duration, Instant};

fn main() {
    let (small, _) = open("example-pdbs/1ubq.pdb", StrictnessLevel::Loose).unwrap();
    let (big, _) = open("example-pdbs/pTLS-6484.pdb", StrictnessLevel::Loose).unwrap();

    measure(bench_open, &"example-pdbs/1ubq.pdb", "Open - small", None);
    measure(
        bench_open,
        &"example-pdbs/pTLS-6484.pdb",
        "Open - big",
        None,
    );
    measure(bench_transformation, &small, "Transformation - small", None);
    measure(bench_transformation, &big, "Transformation - big", None);
    measure(bench_remove, &small, "Remove - small", None);
    measure(bench_remove, &big, "Remove - big", None);
    measure(bench_iteration, &small, "Iteration - small", None);
    measure(bench_iteration, &big, "Iteration - big", None);
    measure(bench_validate, &small, "Validate - small", Some(1000));
    measure(bench_validate, &big, "Validate - big", Some(1000));
    measure(bench_renumber, &small, "Renumber - small", None);
    measure(bench_renumber, &big, "Renumber - big", None);
    measure(bench_clone, &small, "Clone - small", None);
    measure(bench_clone, &big, "Clone - big", None);
    measure(bench_save, &small, "Save - small", None);
    measure(bench_save, &big, "Save - big", None);
}

fn bench_open(filename: &str) -> Duration {
    let now = Instant::now();
    let (_pdb, _errors) = open(filename, StrictnessLevel::Loose).unwrap();
    now.elapsed()
}

fn bench_transformation(mut pdb: PDB) -> Duration {
    let now = Instant::now();
    let transformation = TransformationMatrix::rotation_x(90.0);
    pdb.apply_transformation(&transformation);
    now.elapsed()
}

fn bench_remove(mut pdb: PDB) -> Duration {
    let now = Instant::now();
    pdb.remove_atoms_by(|atom| atom.serial_number() % 2 == 0);
    now.elapsed()
}

fn bench_iteration(pdb: PDB) -> Duration {
    let now = Instant::now();
    let mut _average = 0.0;
    for atom in pdb.atoms() {
        _average += atom.b_factor();
    }
    _average /= pdb.atom_count() as f64;
    now.elapsed()
}

fn bench_validate(pdb: PDB) -> Duration {
    let now = Instant::now();
    validate(&pdb);
    now.elapsed()
}

fn bench_renumber(mut pdb: PDB) -> Duration {
    let now = Instant::now();
    pdb.renumber();
    now.elapsed()
}

fn bench_clone(pdb: PDB) -> Duration {
    let now = Instant::now();
    let _copy = pdb.clone();
    now.elapsed()
}

fn bench_save(pdb: PDB) -> Duration {
    let now = Instant::now();
    save(pdb, "dump/dump.pdb", StrictnessLevel::Loose).unwrap();
    now.elapsed()
}

fn measure<T: Clone>(
    function: fn(T) -> Duration,
    subject: &T,
    description: &str,
    max: Option<u128>,
) {
    let mut times = Vec::new();
    let _ = function(subject.clone());
    let now = Instant::now();

    for _ in 0..5 {
        times.push(function(subject.clone()));
    }

    let average = now.elapsed().checked_div(5).unwrap();

    // Lets run for 3 more seconds, including cloning of the subject
    let mut runs = 3_000_000_000 / average.as_nanos();
    if let Some(n) = max {
        runs = min(n, runs);
    }
    for _ in 0..runs {
        times.push(function(subject.clone()));
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
    let standard_deviation = Duration::from_secs_f64(deviation / 1_000_000_000.0);

    println!(
        "{}: average time over {} runs:\n    {} ± {} ={:6.2}%\n",
        description,
        times.len(),
        pretty_print(average),
        pretty_print(standard_deviation),
        deviation / (average.as_nanos() as f64) * 100.0
    );
}

fn pretty_print(duration: Duration) -> String {
    if duration.as_secs() > 0 {
        format!(
            "{:3}s {:3}ms",
            duration.as_secs(),
            duration.as_millis() - (duration.as_secs() * 1000) as u128
        )
    } else if duration.as_millis() > 0 {
        format!(
            "{:3}ms {:3}μs",
            duration.as_millis(),
            duration.as_micros() - (duration.as_millis() * 1000) as u128
        )
    } else if duration.as_micros() > 0 {
        format!(
            "{:3}μs {:3}ns",
            duration.as_micros(),
            duration.as_nanos() - (duration.as_micros() * 1000) as u128
        )
    } else {
        format!("     {:4}ns", duration.as_nanos())
    }
}
