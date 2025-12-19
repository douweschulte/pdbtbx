//! Benchmark to test the speed of many files

use pdbtbx::*;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::time::{Duration, Instant};

fn main() {
    println!("Starting E. coli PDB loading benchmark...");

    // Find all .pdb files in the directory
    let pdb_files = match fs::read_dir("UP000000625_83333_ECOLI_v4/") {
        Ok(entries) => entries
            .filter_map(Result::ok)
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map_or(false, |ext| ext.to_lowercase() == "pdb")
            })
            .map(|entry| entry.path())
            .collect::<Vec<_>>(),
        Err(e) => {
            eprintln!("Error reading directory: {e}");
            return;
        }
    };

    println!("Found {} PDB files", pdb_files.len());

    if pdb_files.is_empty() {
        println!("No PDB files found in UP000000625_83333_ECOLI_v4/");
        return;
    }

    // Setup parser
    let parser = ReadOptions::default()
        .set_level(StrictnessLevel::Loose)
        .set_format(Format::Pdb)
        .clone();

    let mut load_times = Vec::new();
    let mut successful_loads = 0;
    let mut failed_loads = 0;
    let mut failed_files = Vec::new();

    println!("Loading files...");

    for (i, pdb_file) in pdb_files.iter().enumerate() {
        let start = Instant::now();

        match parser.read(pdb_file.to_string_lossy()) {
            Ok((_pdb, _errors)) => {
                let duration = start.elapsed();
                load_times.push(duration);
                successful_loads += 1;

                if i % 100 == 0 {
                    println!("Loaded {} files so far...", i + 1);
                }
            }
            Err(e) => {
                failed_loads += 1;
                failed_files.push((pdb_file.clone(), format!("{e:?}")));
                if failed_loads <= 10 {
                    // Only show first 10 errors to avoid spam
                    eprintln!("Failed to load {}: {e:?}", pdb_file.display());
                }
            }
        }
    }

    // Calculate statistics
    let total_time: Duration = load_times.iter().sum();
    let average_time = if load_times.is_empty() {
        Duration::new(0, 0)
    } else {
        total_time / load_times.len() as u32
    };

    let mut min_time = Duration::new(u64::MAX, 0);
    let mut max_time = Duration::new(0, 0);
    let mut variance = 0.0;

    if !load_times.is_empty() {
        for time in &load_times {
            if *time < min_time {
                min_time = *time;
            }
            if *time > max_time {
                max_time = *time;
            }
        }

        // Calculate standard deviation
        let avg_nanos = average_time.as_nanos() as f64;
        for time in &load_times {
            let diff = time.as_nanos() as f64 - avg_nanos;
            variance += diff * diff;
        }
        variance /= load_times.len() as f64;
    }

    let std_dev = variance.sqrt();

    println!("\n=== E. coli PDB Loading Benchmark Results ===");
    println!("Total files found: {}", pdb_files.len());
    println!("Successfully loaded: {successful_loads}");
    println!("Failed to load: {failed_loads}");
    println!("Total time: {total_time:?}");

    if successful_loads > 0 {
        println!("Average time per file: {average_time:?}");
        println!(
            "Average time per file (ms): {:.2}",
            average_time.as_secs_f64() * 1000.0
        );
        println!("Min time per file: {min_time:?}");
        println!("Max time per file: {max_time:?}");
        println!("Standard deviation (ms): {:.2}", std_dev / 1_000_000.0);
        println!("Files per second: {:.2}", 1.0 / average_time.as_secs_f64());
    }

    if failed_loads > 10 {
        println!(
            "... and {} more failed loads (only first 10 errors shown)",
            failed_loads - 10
        );
    }

    // Save detailed results to file
    if let Ok(file) = File::create("dump/e_coli_benchmark_results.csv") {
        let mut writer = BufWriter::new(file);
        writer.write_all(b"File,LoadTime(ms),Status\n").unwrap();

        let mut success_index = 0;

        for pdb_file in &pdb_files {
            let filename = pdb_file
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");

            // Check if this file was in the failed list
            let is_failed = failed_files.iter().any(|(path, _)| path == pdb_file);

            if is_failed {
                writer
                    .write_fmt(format_args!("{filename},N/A,Failed\n"))
                    .unwrap();
            } else if success_index < load_times.len() {
                let time_ms = load_times[success_index].as_secs_f64() * 1000.0;
                writer
                    .write_fmt(format_args!("{filename},{time_ms:.2},Success\n"))
                    .unwrap();
                success_index += 1;
            }
        }
        writer.flush().unwrap();
        println!("Detailed results saved to dump/e_coli_benchmark_results.csv");
    }

    println!("Benchmark complete!");
}
