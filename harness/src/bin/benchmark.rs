use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use harness::decoder::{CrabVpxDecoder, LibVpxOracleDecoder, VideoDecoder};
use harness::ivf::IvfParser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory containing .ivf files
    #[arg(short, long)]
    dir: PathBuf,

    /// Run extensive performance benchmarking
    #[arg(short, long)]
    benchmark: bool,

    /// Number of benchmark iterations (default: 50)
    #[arg(short, long, default_value_t = 50)]
    runs: u32,
}

fn main() {
    let args = Args::parse();

    if !args.dir.exists() || !args.dir.is_dir() {
        eprintln!("Error: Directory does not exist or is not a directory.");
        std::process::exit(1);
    }

    let mut ivf_files = Vec::new();
    if let Ok(entries) = fs::read_dir(&args.dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "ivf") {
                ivf_files.push(path);
            }
        }
    }

    if ivf_files.is_empty() {
        println!("No .ivf files found in the specified directory.");
        return;
    }

    println!("Found {} IVF files to test.", ivf_files.len());

    let pb = ProgressBar::new(ivf_files.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("##-"),
    );

    let mut successful_decodes_oracle = 0;
    let mut successful_decodes_crab = 0;
    let mut total_decode_time_oracle = Duration::ZERO;
    let mut total_decode_time_crab = Duration::ZERO;
    let mut total_frames = 0;

    let benchmark_iterations = args.runs as usize;
    let mut suite_iter_times_oracle = vec![Duration::ZERO; benchmark_iterations];
    let mut suite_iter_times_crab = vec![Duration::ZERO; benchmark_iterations];

    for file in &ivf_files {
        pb.set_message(file.file_name().unwrap().to_string_lossy().into_owned());

        let mut ivf = match IvfParser::new(file) {
            Ok(parser) => parser,
            Err(_) => {
                pb.println(format!("Failed to parse header: {:?}", file));
                pb.inc(1);
                continue;
            }
        };

        let mut frames = Vec::new();
        while let Ok(Some(frame)) = ivf.next_frame() {
            frames.push(frame);
        }

        if args.benchmark {
            let frames_len = frames.len() as u32;

            // Warmup
            let mut oracle = LibVpxOracleDecoder::new();
            if oracle.init().is_ok() {
                for frame in &frames {
                    let _ = oracle.decode_frame(&frame.payload);
                    let _ = oracle.get_frame();
                }
            }

            let mut crab = CrabVpxDecoder::new();
            if crab.init().is_ok() {
                for frame in &frames {
                    let _ = crab.decode_frame(&frame.payload);
                    let _ = crab.get_frame();
                }
            }

            let mut success_oracle = true;
            for i in 0..benchmark_iterations {
                let mut decoder = LibVpxOracleDecoder::new();
                if decoder.init().is_err() {
                    success_oracle = false;
                    break;
                }
                let start = Instant::now();
                for frame in &frames {
                    if decoder.decode_frame(&frame.payload).is_err() || decoder.get_frame().is_err()
                    {
                        success_oracle = false;
                        break;
                    }
                }
                suite_iter_times_oracle[i] += start.elapsed();
                if !success_oracle {
                    break;
                }
            }

            let mut success_crab = true;
            for i in 0..benchmark_iterations {
                let mut decoder = CrabVpxDecoder::new();
                if decoder.init().is_err() {
                    success_crab = false;
                    break;
                }
                let start = Instant::now();
                for frame in &frames {
                    if decoder.decode_frame(&frame.payload).is_err() || decoder.get_frame().is_err()
                    {
                        success_crab = false;
                        break;
                    }
                }
                suite_iter_times_crab[i] += start.elapsed();
                if !success_crab {
                    break;
                }
            }

            if success_oracle && success_crab {
                successful_decodes_oracle += 1;
                successful_decodes_crab += 1;
                total_frames += frames_len;
            } else {
                pb.println(format!("Decoding failed for {:?}", file));
            }
        } else {
            let mut success_oracle = true;
            let mut oracle = LibVpxOracleDecoder::new();
            if oracle.init().is_err() {
                success_oracle = false;
            }
            let start = Instant::now();
            if success_oracle {
                for frame in &frames {
                    if oracle.decode_frame(&frame.payload).is_err() || oracle.get_frame().is_err() {
                        success_oracle = false;
                        break;
                    }
                }
            }
            if success_oracle {
                total_decode_time_oracle += start.elapsed();
                successful_decodes_oracle += 1;
            }

            let mut success_crab = true;
            let mut crab = CrabVpxDecoder::new();
            if crab.init().is_err() {
                success_crab = false;
            }
            let start = Instant::now();
            if success_crab {
                for frame in &frames {
                    if crab.decode_frame(&frame.payload).is_err() || crab.get_frame().is_err() {
                        success_crab = false;
                        break;
                    }
                }
            }
            if success_crab {
                total_decode_time_crab += start.elapsed();
                successful_decodes_crab += 1;
            }

            if success_oracle || success_crab {
                total_frames += frames.len() as u32;
            }
        }

        pb.inc(1);
    }

    pb.finish_with_message("Done");

    if args.benchmark {
        let calc_stats = |times: &[Duration]| -> (Duration, f64, Duration, Duration) {
            let sum: Duration = times.iter().sum();
            let avg = sum / (times.len() as u32);
            let avg_secs = avg.as_secs_f64();
            let variance = times
                .iter()
                .map(|t| (t.as_secs_f64() - avg_secs).powi(2))
                .sum::<f64>()
                / (times.len() as f64);
            let sigma = variance.sqrt() * 1000.0; // ms
            let min = *times.iter().min().unwrap();
            let max = *times.iter().max().unwrap();
            (avg, sigma, min, max)
        };

        let (avg_oracle, sigma_oracle, min_oracle, max_oracle) =
            calc_stats(&suite_iter_times_oracle);
        let (avg_crab, sigma_crab, min_crab, max_crab) = calc_stats(&suite_iter_times_crab);

        let ms_per_frame_oracle = avg_oracle.as_secs_f64() * 1000.0 / (total_frames as f64);
        let ms_per_frame_crab = avg_crab.as_secs_f64() * 1000.0 / (total_frames as f64);

        println!("\n CrabVPX Performance Analysis vs. C Oracle");
        println!("-------------------------------------------------------");
        println!(
            " {:<15} | {:<10} | {:<10} | {:<10}",
            "Metric", "C Oracle", "CrabVPX", "Diff (%)"
        );
        println!("-------------------------------------------------------");

        let print_row = |label: &str, oracle_val: f64, crab_val: f64, unit: &str| {
            let diff = ((crab_val - oracle_val) / oracle_val) * 100.0;
            let color = if diff <= 0.0 { "\x1b[32m" } else { "\x1b[31m" };
            let reset = "\x1b[0m";
            println!(
                " {:<15} | {:>7.2} {} | {:>7.2} {} | {}{:>+8.2} %{}",
                label, oracle_val, unit, crab_val, unit, color, diff, reset
            );
        };

        print_row(
            "Suite Average",
            avg_oracle.as_secs_f64() * 1000.0,
            avg_crab.as_secs_f64() * 1000.0,
            "ms",
        );
        print_row("Suite Sigma (σ)", sigma_oracle, sigma_crab, "ms");
        print_row(
            "Suite Min",
            min_oracle.as_secs_f64() * 1000.0,
            min_crab.as_secs_f64() * 1000.0,
            "ms",
        );
        print_row(
            "Suite Max",
            max_oracle.as_secs_f64() * 1000.0,
            max_crab.as_secs_f64() * 1000.0,
            "ms",
        );
        print_row("Per Frame", ms_per_frame_oracle, ms_per_frame_crab, "ms");
        println!("-------------------------------------------------------");
        println!(" Note: Positive % indicates Rust regression (slower).");

        // ASCII Graph
        let mut all_times: Vec<f64> = suite_iter_times_oracle
            .iter()
            .map(|t| t.as_secs_f64() * 1000.0)
            .collect();
        all_times.extend(
            suite_iter_times_crab
                .iter()
                .map(|t| t.as_secs_f64() * 1000.0),
        );
        let min_t = all_times.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_t = all_times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let buckets = 40;
        let width = if max_t > min_t {
            (max_t - min_t) / (buckets as f64)
        } else {
            1.0
        };

        let get_hist = |times: &[Duration]| -> Vec<usize> {
            let mut hist = vec![0; buckets];
            for t in times {
                let t_ms = t.as_secs_f64() * 1000.0;
                let idx = std::cmp::min(((t_ms - min_t) / width) as usize, buckets - 1);
                hist[idx] += 1;
            }
            hist
        };

        let o_hist = get_hist(&suite_iter_times_oracle);
        let r_hist = get_hist(&suite_iter_times_crab);
        let max_h = std::cmp::max(*o_hist.iter().max().unwrap(), *r_hist.iter().max().unwrap());

        let mut graph =
            String::from("Suite Performance Distribution (X: time, O: Oracle, R: Rust)\n");
        for i in 0..buckets {
            let t_label = min_t + (i as f64) * width;
            let o_bar = if max_h > 0 {
                "O".repeat((o_hist[i] as f64 / max_h as f64 * 20.0) as usize)
            } else {
                String::new()
            };
            let r_bar = if max_h > 0 {
                "R".repeat((r_hist[i] as f64 / max_h as f64 * 20.0) as usize)
            } else {
                String::new()
            };
            graph.push_str(&format!(" {:>6.1} ms | {:<20} {}\n", t_label, o_bar, r_bar));
        }

        let out_dir = args.dir.parent().unwrap().join("out");
        fs::create_dir_all(&out_dir).unwrap();
        let report_file = out_dir.join("benchmark_results.md");

        let report = format!(
            "# Performance Benchmark Report\n\n\
| Metric | C Oracle | CrabVPX | Diff (%) |\n\
|---|---|---|---|\n\
| Suite Average | {:.2} ms | {:.2} ms | {:+.2}% |\n\
| Suite Sigma (σ) | {:.2} ms | {:.2} ms | {:+.2}% |\n\
| Suite Min | {:.2} ms | {:.2} ms | {:+.2}% |\n\
| Suite Max | {:.2} ms | {:.2} ms | {:+.2}% |\n\
| Per Frame | {:.2} ms | {:.2} ms | {:+.2}% |\n\n\
```text\n\
{}\n\
```",
            avg_oracle.as_secs_f64() * 1000.0,
            avg_crab.as_secs_f64() * 1000.0,
            ((avg_crab.as_secs_f64() - avg_oracle.as_secs_f64()) / avg_oracle.as_secs_f64())
                * 100.0,
            sigma_oracle,
            sigma_crab,
            ((sigma_crab - sigma_oracle) / sigma_oracle) * 100.0,
            min_oracle.as_secs_f64() * 1000.0,
            min_crab.as_secs_f64() * 1000.0,
            ((min_crab.as_secs_f64() - min_oracle.as_secs_f64()) / min_oracle.as_secs_f64())
                * 100.0,
            max_oracle.as_secs_f64() * 1000.0,
            max_crab.as_secs_f64() * 1000.0,
            ((max_crab.as_secs_f64() - max_oracle.as_secs_f64()) / max_oracle.as_secs_f64())
                * 100.0,
            ms_per_frame_oracle,
            ms_per_frame_crab,
            ((ms_per_frame_crab - ms_per_frame_oracle) / ms_per_frame_oracle) * 100.0,
            graph
        );
        fs::write(&report_file, report).unwrap();
        println!("\nDetailed report written to: {:?}", report_file);
    } else {
        println!(
            "\nOracle: {}/{} decoded in {:.2?}",
            successful_decodes_oracle,
            ivf_files.len(),
            total_decode_time_oracle
        );
        println!(
            "CrabVPX: {}/{} decoded in {:.2?}",
            successful_decodes_crab,
            ivf_files.len(),
            total_decode_time_crab
        );
    }
}
