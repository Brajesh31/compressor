use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::fs::File;
use std::io::{self, copy, BufReader};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

#[derive(Debug)]
struct CompressionReport {
    source_path: PathBuf,
    target_path: PathBuf,
    source_bytes: u64,
    target_bytes: u64,
    elapsed: Duration,
}

fn compress_file(source: &Path, target: &Path) -> io::Result<CompressionReport> {
    if source == target {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "source and target paths must be different",
        ));
    }

    let input_file = File::open(source)?;
    let source_bytes = input_file.metadata()?.len();
    let input = BufReader::new(input_file);
    let output = File::create(target)?;
    let mut encoder = GzEncoder::new(output, Compression::default());

    let start = Instant::now();
    let mut input = input;
    copy(&mut input, &mut encoder)?;
    let output = encoder.finish()?;
    let elapsed = start.elapsed();
    let target_bytes = output.metadata()?.len();

    Ok(CompressionReport {
        source_path: source.to_path_buf(),
        target_path: target.to_path_buf(),
        source_bytes,
        target_bytes,
        elapsed,
    })
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 4] = ["B", "KB", "MB", "GB"];
    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{} {}", bytes, UNITS[unit])
    } else {
        format!("{value:.2} {}", UNITS[unit])
    }
}

fn print_usage(program: &str) {
    eprintln!("Usage: {program} <source> <target.gz>");
    eprintln!();
    eprintln!("Compress a file into the gzip format.");
    eprintln!("The target file is created or overwritten.");
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args_os();
    let program = args
        .next()
        .unwrap_or_else(|| "compressor".into());
    let source = args.next().map(PathBuf::from);
    let target = args.next().map(PathBuf::from);
    let extra = args.next();

    let (Some(source), Some(target), None) = (source, target, extra) else {
        print_usage(&program.to_string_lossy());
        return Err("expected exactly two file paths".into());
    };

    let report = compress_file(&source, &target)?;
    println!("✅ Compression completed");
    println!("Source: {}", report.source_path.display());
    println!("Target: {}", report.target_path.display());
    println!("Source size: {}", format_bytes(report.source_bytes));
    println!("Target size: {}", format_bytes(report.target_bytes));
    if report.source_bytes > 0 {
        let ratio = report.target_bytes as f64 / report.source_bytes as f64 * 100.0;
        println!("Output/input: {ratio:.2}%");
        println!("Space saved: {:.2}%", 100.0 - ratio);
    } else {
        println!("Output/input: n/a (empty source file)");
    }
    println!("Elapsed: {:?}", report.elapsed);
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("❌ Error: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::read::GzDecoder;
    use std::fs;
    use std::io::Read;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_path(name: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("compressor-{nonce}-{name}"))
    }

    #[test]
    fn compresses_and_can_decompress_payload() {
        let source = temp_path("source.txt");
        let target = temp_path("target.gz");
        let payload = b"Coding Bird compressor test payload\n".repeat(64);
        fs::write(&source, &payload).expect("write source");

        let report = compress_file(&source, &target).expect("compress source");
        assert_eq!(report.source_bytes, payload.len() as u64);
        assert!(report.target_bytes > 0);

        let mut decoder = GzDecoder::new(File::open(&target).expect("open gzip"));
        let mut restored = Vec::new();
        decoder.read_to_end(&mut restored).expect("decompress");
        assert_eq!(restored, payload);

        fs::remove_file(source).ok();
        fs::remove_file(target).ok();
    }

    #[test]
    fn supports_empty_files() {
        let source = temp_path("empty.txt");
        let target = temp_path("empty.gz");
        fs::write(&source, []).expect("create empty source");
        let report = compress_file(&source, &target).expect("compress empty source");
        assert_eq!(report.source_bytes, 0);
        assert!(report.target_bytes > 0);
        fs::remove_file(source).ok();
        fs::remove_file(target).ok();
    }

    #[test]
    fn rejects_same_source_and_target() {
        let source = temp_path("same.txt");
        fs::write(&source, b"do not overwrite me").expect("write source");
        let error = compress_file(&source, &source).expect_err("same path must fail");
        assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
        fs::remove_file(source).ok();
    }

    #[test]
    fn reports_missing_source_without_panicking() {
        let source = temp_path("missing.txt");
        let target = temp_path("missing.gz");
        let error = compress_file(&source, &target).expect_err("missing source must fail");
        assert_eq!(error.kind(), io::ErrorKind::NotFound);
        assert!(!target.exists());
    }

    #[test]
    fn formats_byte_values() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
    }
}
