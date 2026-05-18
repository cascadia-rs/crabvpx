use clap::Parser;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about = "Analyze c2rust transpilation complexity.")]
struct Args {
    /// Directory to analyze (e.g. 'src/')
    #[arg(long)]
    src_dir: Option<PathBuf>,

    /// Run full pipeline
    #[arg(long)]
    full_pipeline: bool,

    /// Output directory
    #[arg(long, default_value = "out")]
    out_dir: PathBuf,
}

#[derive(Clone, Debug)]
struct FileStats {
    path: String,
    total: usize,
    blocks: usize,
    fns: usize,
    statics: usize,
}

fn run_cmd(cmd: &[&str], cwd: &Path) {
    println!("Running command: {}", cmd.join(" "));
    let status = Command::new(cmd[0])
        .args(&cmd[1..])
        .current_dir(cwd)
        .status()
        .expect("Failed to execute process");

    if !status.success() {
        eprintln!("Command failed: {}", cmd.join(" "));
        std::process::exit(1);
    }
}

fn clone_libvpx(out_dir: &Path) -> PathBuf {
    let libvpx_path = out_dir.join("libvpx_full");
    if libvpx_path.exists() {
        println!(
            "Directory {:?} already exists, skipping clone.",
            libvpx_path
        );
        return libvpx_path;
    }

    println!("Cloning libvpx into {:?}...", libvpx_path);
    run_cmd(
        &[
            "git",
            "clone",
            "https://chromium.googlesource.com/webm/libvpx",
            libvpx_path.to_str().unwrap(),
        ],
        out_dir,
    );
    libvpx_path
}

fn configure_libvpx(libvpx_path: &Path) {
    println!("Configuring libvpx with multiple features enabled...");
    run_cmd(
        &[
            "./configure",
            "--enable-vp8",
            "--enable-vp9",
            "--enable-vp9-highbitdepth",
            "--enable-postproc",
            "--enable-multithread",
            "--enable-experimental",
        ],
        libvpx_path,
    );
}

fn generate_compile_commands(libvpx_path: &Path) {
    println!("Building libvpx and generating compile_commands.json using bear...");
    let num_cores = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
        .to_string();
    run_cmd(&["bear", "--", "make", "-j", &num_cores], libvpx_path);

    if !libvpx_path.join("compile_commands.json").exists() {
        eprintln!("Failed to generate compile_commands.json");
        std::process::exit(1);
    }
}

fn run_c2rust(libvpx_path: &Path) {
    println!("Transpiling C code to Rust using c2rust...");
    run_cmd(
        &[
            "c2rust",
            "transpile",
            "compile_commands.json",
            "--emit-build-files",
        ],
        libvpx_path,
    );
}

fn count_unsafe(file_path: &Path) -> (usize, usize, usize, usize) {
    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(_) => return (0, 0, 0, 0),
    };

    let re_total = Regex::new(r"\bunsafe\b").unwrap();
    let re_blocks = Regex::new(r"unsafe\s*\{").unwrap();
    let re_fns = Regex::new(r"unsafe\s+(?:extern\s+.[^.].\s+)?fn\s+").unwrap();
    let re_statics = Regex::new(r"static\s+mut\s+").unwrap();

    (
        re_total.find_iter(&content).count(),
        re_blocks.find_iter(&content).count(),
        re_fns.find_iter(&content).count(),
        re_statics.find_iter(&content).count(),
    )
}

fn gather_file_stats(src_dir: &Path) -> Vec<FileStats> {
    let mut file_stats = Vec::new();
    for entry in WalkDir::new(src_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "rs") {
            let path_str = path.to_string_lossy().to_string();
            if path_str.contains("target/")
                || path.file_name().unwrap() == "build.rs"
                || path.file_name().unwrap() == "lib.rs"
            {
                continue;
            }

            let (total, blocks, fns, statics) = count_unsafe(path);
            if total > 0 {
                let rel_path = path
                    .strip_prefix(src_dir)
                    .unwrap_or(path)
                    .to_string_lossy()
                    .to_string();
                file_stats.push(FileStats {
                    path: rel_path,
                    total,
                    blocks,
                    fns,
                    statics,
                });
            }
        }
    }
    file_stats
}

fn add_top_files(
    report: &mut String,
    title: &str,
    stats: &mut [FileStats],
    key_fn: fn(&FileStats) -> usize,
) {
    report.push_str(&format!("{}\n| File | Count |\n|---|---|\n", title));
    stats.sort_by_key(|b| std::cmp::Reverse(key_fn(b)));

    for stat in stats.iter().take(10) {
        report.push_str(&format!("| `{}` | {} |\n", stat.path, key_fn(stat)));
    }
    report.push('\n');
}

fn format_markdown_report(src_dir: &Path, mut stats: Vec<FileStats>) -> String {
    let total_unsafe: usize = stats.iter().map(|s| s.total).sum();
    let total_blocks: usize = stats.iter().map(|s| s.blocks).sum();
    let total_fns: usize = stats.iter().map(|s| s.fns).sum();
    let total_statics: usize = stats.iter().map(|s| s.statics).sum();

    let mut report = format!(
        "# Unsafe Usage Analysis Report\n\n\
        This report provides a programmatic analysis of the generated codebase,\n\
        quantifying the scale of unsafety and technical debt required.\n\n\
        - **Target Directory:** `{}`\n\
        - **Total `unsafe` Keywords:** {}\n\
        - **Total Unsafe Blocks (`unsafe {{ ... }}`):** {}\n\
        - **Total Unsafe Functions (`unsafe fn`):** {}\n\
        - **Total Global Mutable State (`static mut`):** {}\n\n",
        src_dir.file_name().unwrap_or_default().to_string_lossy(),
        total_unsafe,
        total_blocks,
        total_fns,
        total_statics
    );

    add_top_files(
        &mut report,
        "## Top 10 Files (by Total Unsafe Keywords)",
        &mut stats,
        |s| s.total,
    );
    add_top_files(
        &mut report,
        "## Top 10 Files (by Unsafe Blocks)",
        &mut stats,
        |s| s.blocks,
    );
    add_top_files(
        &mut report,
        "## Top 10 Files (by Unsafe Functions)",
        &mut stats,
        |s| s.fns,
    );
    add_top_files(
        &mut report,
        "## Top 10 Files (by Global Mutable State)",
        &mut stats,
        |s| s.statics,
    );

    report.push_str(
        "## Conclusion\n\
    Files with high unsafety contain dense pointer arithmetic, manual memory,\n\
    global mutable state, or hardware intrinsics. They represent the highest\n\
    risk for manual refactoring to idiomatic, safe Rust.\n",
    );

    report
}

fn analyze_complexity(src_dir: &Path, out_file: &Path) {
    println!("Analyzing Rust files in {:?} for unsafe usage...", src_dir);
    let stats = gather_file_stats(src_dir);

    println!("Generating Markdown report...");
    let report_content = format_markdown_report(src_dir, stats);

    if let Some(parent) = out_file.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(out_file, report_content).unwrap();
    println!("Report written successfully to {:?}", out_file);
}

fn run_pipeline(out_dir: &Path) {
    println!("Starting full libvpx transpilation pipeline simulation...");
    fs::create_dir_all(out_dir).unwrap();

    let libvpx_path = clone_libvpx(out_dir);
    configure_libvpx(&libvpx_path);
    generate_compile_commands(&libvpx_path);
    run_c2rust(&libvpx_path);

    let report_file = out_dir.join("full_libvpx_unsafe_analysis.md");
    analyze_complexity(&libvpx_path, &report_file);
    println!("Pipeline complete.");
}

fn main() {
    let args = Args::parse();
    let out_dir = args.out_dir.canonicalize().unwrap_or_else(|_| {
        fs::create_dir_all(&args.out_dir).unwrap();
        args.out_dir.canonicalize().unwrap()
    });

    if args.full_pipeline {
        run_pipeline(&out_dir);
    } else if let Some(src_dir) = args.src_dir {
        let src_dir = src_dir.canonicalize().unwrap_or(src_dir);
        if !src_dir.exists() || !src_dir.is_dir() {
            eprintln!("Source directory {:?} does not exist.", src_dir);
            std::process::exit(1);
        }

        let report_file = out_dir.join("unsafe_analysis.md");
        analyze_complexity(&src_dir, &report_file);
    } else {
        eprintln!("You must specify either --src-dir or --full-pipeline.");
        std::process::exit(1);
    }
}
