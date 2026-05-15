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
    let mut total_decode_time = Duration::ZERO;
    let mut total_frames = 0;

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
            // Warmup
            let mut decoder = ActiveDecoder::new();
            if decoder.init().is_ok() {
                for frame in &frames {
                    let _ = decoder.decode_frame(&frame.payload);
                    let _ = decoder.get_frame();
                }
            }

            let iterations = 10;
            let mut iter_times = Vec::with_capacity(iterations);
            let mut success = true;

            for _ in 0..iterations {
                let mut decoder = ActiveDecoder::new();
                if decoder.init().is_err() {
                    success = false;
                    break;
                }

                let start = Instant::now();
                for frame in &frames {
                    if decoder.decode_frame(&frame.payload).is_err() || decoder.get_frame().is_err() {
                        success = false;
                        break;
                    }
                }
                iter_times.push(start.elapsed());
                if !success { break; }
            }

            if success {
                successful_decodes += 1;
                let sum: Duration = iter_times.iter().sum();
                total_decode_time += sum / (iterations as u32);
                total_frames += frames.len() as u32;
                
                let min = iter_times.iter().min().unwrap();
                let max = iter_times.iter().max().unwrap();
                let avg = sum / (iterations as u32);
                pb.println(format!("{:?}: avg {:?}, min {:?}, max {:?}", file.file_name().unwrap(), avg, min, max));
            } else {
                pb.println(format!("Decoding failed for {:?}", file));
            }

        } else {
            let mut decoder = ActiveDecoder::new();
            if let Err(e) = decoder.init() {
                pb.println(format!("Failed to initialize decoder for {:?}: {}", file, e));
                pb.inc(1);
                continue;
            }

            let mut success = true;
            let start = Instant::now();
            for frame in &frames {
                if decoder.decode_frame(&frame.payload).is_err() || decoder.get_frame().is_err() {
                    success = false;
                    break;
                }
            }
            let elapsed = start.elapsed();

            if success {
                successful_decodes += 1;
                total_decode_time += elapsed;
                total_frames += frames.len() as u32;
            } else {
                pb.println(format!("Decoding failed for {:?}", file));
            }
        }

        pb.inc(1);
    }

    pb.finish_with_message("Done");

    let avg_time_per_frame = if total_frames > 0 {
        total_decode_time.as_secs_f64() * 1000.0 / (total_frames as f64)
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
