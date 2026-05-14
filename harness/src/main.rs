mod decoder;
mod ivf;

use clap::Parser;
use decoder::{LibVpxOracleDecoder, VideoDecoder};
use indicatif::{ProgressBar, ProgressStyle};
use ivf::IvfParser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory containing .ivf files
    #[arg(short, long)]
    dir: PathBuf,
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

        let mut decoder = LibVpxOracleDecoder::new();
        if let Err(e) = decoder.init() {
            pb.println(format!("Failed to initialize decoder for {:?}: {}", file, e));
            pb.inc(1);
            continue;
        }

        let mut success = true;
        while let Ok(Some(frame)) = ivf.next_frame() {
            if let Err(_) = decoder.decode_frame(&frame.payload) {
                success = false;
                break;
            }
            if let Err(_) = decoder.get_frame() {
                success = false;
                break;
            }
        }

        if success {
            successful_decodes += 1;
        } else {
            pb.println(format!("Decoding failed for {:?}", file));
        }

        pb.inc(1);
    }

    pb.finish_with_message("Done");

    println!(
        "\n{} out of {} vectors decoded successfully by the C Oracle.",
        successful_decodes,
        ivf_files.len()
    );
}
