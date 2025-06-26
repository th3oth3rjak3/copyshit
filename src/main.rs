use std::{fs, path::PathBuf};

use clap::Parser;
use walkdir::{DirEntry, WalkDir};

/// A simple program to copy some shit with the provided file extension.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The source directory
    #[arg(short, long, default_value = "./")]
    source: PathBuf,

    /// The destination directory
    #[arg(short, long)]
    destination: PathBuf,

    /// The extension that the files must end with
    #[arg(short, long)]
    extension: String,
}

fn humanize_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut args = Args::parse();

    args.extension = args.extension.trim_start_matches(".").to_string();

    let mut files_to_copy: Vec<DirEntry> = Vec::new();

    println!("{:?}", args);
    let walker = WalkDir::new(&args.source);

    let file_iter = walker
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file());

    for entry in file_iter {
        if let Some(ext) = entry.path().extension() {
            if args.extension == ext.to_str().unwrap() {
                files_to_copy.push(entry);
            }
        }
    }

    if !args.destination.exists() {
        eprintln!("Destination directory does not exist. Create it first, then try again.");
        return Ok(());
    }

    for file_entry in files_to_copy {
        let source_path = file_entry.path();

        let dest_path = args.destination.join(source_path.file_name().unwrap());

        if dest_path.exists() {
            println!("Skipping {} - already exists", dest_path.display());
            continue;
        }

        match std::fs::copy(source_path, &dest_path) {
            Ok(bytes) => println!("Copied {} ({})", dest_path.display(), humanize_bytes(bytes)),
            Err(e) => eprintln!("Failed to copy {}: {}", source_path.display(), e),
        }
    }

    return Ok(());
}
