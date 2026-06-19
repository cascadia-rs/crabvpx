//! Portable decode-correctness gate.
//!
//! Decodes every `test_data/*.ivf` vector with the CrabVPX decoder and compares
//! the per-frame MD5 of each displayed frame against the precomputed reference
//! digests in the matching `*.ivf.md5` file. These references were produced by
//! libvpx, so this is a true differential test against the C oracle, but it
//! needs no libvpx build at test time -- making it runnable anywhere.

use crabvpx::api::{Decoder, Vp8Decoder};
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

/// Minimal IVF demuxer: 32-byte file header, then per-frame 12-byte headers
/// (4-byte LE size + 8-byte timestamp) followed by the payload.
struct IvfParser {
    file: File,
}

impl IvfParser {
    fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut header = [0u8; 32];
        file.read_exact(&mut header)?;
        if &header[0..4] != b"DKIF" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid IVF header",
            ));
        }
        Ok(Self { file })
    }

    fn next_frame(&mut self) -> io::Result<Option<Vec<u8>>> {
        let mut frame_header = [0u8; 12];
        match self.file.read_exact(&mut frame_header) {
            Ok(_) => {}
            Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
            Err(e) => return Err(e),
        }
        let size = u32::from_le_bytes(frame_header[0..4].try_into().unwrap()) as usize;
        let mut payload = vec![0u8; size];
        self.file.read_exact(&mut payload)?;
        Ok(Some(payload))
    }
}

/// Reads the reference `.ivf.md5` file: one `"<md5hex>  <name>.i420"` per line.
fn read_reference_md5s(path: &Path) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("failed to read {:?}: {e}", path))
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.split_whitespace().next().unwrap().to_string())
        .collect()
}

fn decode_md5s(ivf_path: &Path) -> Vec<String> {
    let mut ivf = IvfParser::new(ivf_path).expect("failed to parse IVF");
    let mut crab = Vp8Decoder::new();
    crab.init().expect("CrabVPX init failed");

    let mut hashes = Vec::new();
    while let Some(payload) = ivf.next_frame().expect("IVF read error") {
        crab.decode(&payload).expect("CrabVPX decode failed");
        // A single compressed frame may yield zero or one displayed frame.
        while let Some(frame) = crab.get_frame().expect("CrabVPX get_frame failed") {
            hashes.push(frame.md5());
        }
    }
    hashes
}

#[test]
fn test_differential_md5() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let test_data_dir = manifest_dir.join("test_data");
    assert!(
        test_data_dir.exists(),
        "test_data/ not found at {:?}",
        test_data_dir
    );

    let mut ivf_files: Vec<PathBuf> = fs::read_dir(&test_data_dir)
        .expect("failed to read test_data/")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|ext| ext == "ivf"))
        .collect();
    ivf_files.sort();
    assert!(!ivf_files.is_empty(), "no .ivf vectors found in test_data/");

    let mut failures = Vec::new();
    for ivf in &ivf_files {
        let md5_path = ivf.with_extension("ivf.md5");
        if !md5_path.exists() {
            continue;
        }
        let expected = read_reference_md5s(&md5_path);
        let actual = decode_md5s(ivf);

        if actual.len() != expected.len() {
            failures.push(format!(
                "{:?}: frame count mismatch (got {}, expected {})",
                ivf.file_name().unwrap(),
                actual.len(),
                expected.len()
            ));
            continue;
        }
        for (i, (a, e)) in actual.iter().zip(expected.iter()).enumerate() {
            if a != e {
                failures.push(format!(
                    "{:?}: frame {} mismatch (got {}, expected {})",
                    ivf.file_name().unwrap(),
                    i,
                    a,
                    e
                ));
            }
        }
    }

    assert!(
        failures.is_empty(),
        "differential MD5 mismatches:\n{}",
        failures.join("\n")
    );
}
