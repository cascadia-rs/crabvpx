mod decoder;
mod ivf;

use clap::Parser;
use decoder::VideoDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use ivf::IvfParser;
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};

#[cfg(not(feature = "rust"))]
use decoder::LibVpxOracleDecoder as ActiveDecoder;
#[cfg(not(feature = "rust"))]
const DECODER_NAME: &str = "Oracle";

#[cfg(feature = "rust")]
use decoder::CrabVpxDecoder as ActiveDecoder;
#[cfg(feature = "rust")]
const DECODER_NAME: &str = "CrabVPX Rust";

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

    let mut successful_decodes = 0;
    let mut total_frames = 0;

    let benchmark_iterations = if args.benchmark { args.runs as usize } else { 1 };
    let mut suite_iter_times = vec![Duration::ZERO; benchmark_iterations];

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

        let md5_file = file.with_extension("ivf.md5");
        let expected_md5s = if !args.benchmark && md5_file.exists() {
            let content = fs::read_to_string(md5_file).unwrap_or_default();
            content.lines()
                .filter_map(|l| l.split_whitespace().next())
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };

        if args.benchmark {
            // Warmup
            let mut decoder = ActiveDecoder::new();
            if decoder.init().is_ok() {
                for frame in &frames {
                    let _ = decoder.decode_frame(&frame.payload);
                    let _ = decoder.get_frame();
                }
            }
        }

        let mut success = true;
        for iter in 0..benchmark_iterations {
            let mut decoder = ActiveDecoder::new();
            if let Err(e) = decoder.init() {
                pb.println(format!("Failed to initialize decoder: {}", e));
                success = false;
                break;
            }

            let start = Instant::now();
            let mut frame_idx = 0;
            for frame in &frames {
                if decoder.decode_frame(&frame.payload).is_err() {
                    success = false;
                    break;
                }

                match decoder.get_frame() {
                    Ok(Some(actual)) => {
                        // Check against expected if not benchmarking
                        if !args.benchmark && !expected_md5s.is_empty() && frame_idx < expected_md5s.len() {
                            if actual.md5 != expected_md5s[frame_idx] {
                                pb.println(format!("MD5 mismatch for {:?} frame {}: expected {}, got {}", 
                                    file.file_name().unwrap(), frame_idx, expected_md5s[frame_idx], actual.md5));
                                success = false;
                                break;
                            }
                        }
                        
                        // Output machine-readable info for side-by-side comparison
                        if !args.benchmark {
                            pb.suspend(|| {
                                println!("FRAME_DATA: {{\"file\": {:?}, \"idx\": {}, \"md5\": \"{}\", \"w\": {}, \"h\": {}, \"bits\": {}}}",
                                    file.file_name().unwrap(), frame_idx, actual.md5, actual.width, actual.height, actual.bit_depth);
                            });
                        }
                        
                        frame_idx += 1;
                    }
                    Ok(None) => {}
                    Err(_) => {
                        success = false;
                        break;
                    }
                }
            }
            suite_iter_times[iter] += start.elapsed();
            if !success { break; }
        }

        if success {
            successful_decodes += 1;
            total_frames += frames.len() as u32;
        } else {
            pb.println(format!("Decoding failed for {:?}", file));
        }

        pb.inc(1);
    }

    pb.finish_with_message("Done");

    if args.benchmark {
        let min = suite_iter_times.iter().min().unwrap();
        let max = suite_iter_times.iter().max().unwrap();
        let sum: Duration = suite_iter_times.iter().sum();
        let avg = sum / (benchmark_iterations as u32);
        
        let avg_time_per_frame = if total_frames > 0 {
            avg.as_secs_f64() * 1000.0 / (total_frames as f64)
        } else {
            0.0
        };

        println!(
            "\n{} out of {} vectors decoded successfully by the {} decoder.",
            successful_decodes,
            ivf_files.len(),
            DECODER_NAME
        );
        
        let mut raw_times = String::new();
        for (i, t) in suite_iter_times.iter().enumerate() {
            if i > 0 { raw_times.push(','); }
            raw_times.push_str(&format!("{:.4}", t.as_secs_f64() * 1000.0));
        }

        println!("OVERALL_SUITE_PERF: avg {:.2?}, min {:.2?}, max {:.2?}, {:.2} ms/frame", avg, min, max, avg_time_per_frame);
        println!("RAW_ITERATION_TIMES_MS: {}", raw_times);

    } else {
        let total_time: Duration = suite_iter_times.iter().sum();
        let avg_time_per_frame = if total_frames > 0 {
            total_time.as_secs_f64() * 1000.0 / (total_frames as f64)
        } else {
            0.0
        };

        println!(
            "\n{} out of {} vectors decoded successfully by the {} decoder in {:.2} ms/frame.",
            successful_decodes,
            ivf_files.len(),
            DECODER_NAME,
            avg_time_per_frame
        );
    }
}
