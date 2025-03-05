use anyhow::{Context, Result};
use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::utils::file_has_right_extension;

use crate::VideoDuplicatesArgs;

// Admissible extensions for this command
const EXTENSIONS: [&str; 4] = ["ts", "mp4", "mkv", "mov"];

pub fn execute(args: VideoDuplicatesArgs) -> Result<()> {
    // Parse the arguments
    let target = Path::new(&args.target);

    // Error if it does not exist at all
    if !target.exists() {
        return Err(anyhow::Error::msg("Target directory does not exist"));
    }

    // Error if target is not a directory
    if !target.is_dir() {
        return Err(anyhow::Error::msg("Target must be a directory"));
    }

    // Find all admissible files
    let video_files: Vec<PathBuf> = WalkDir::new(target)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| file_has_right_extension(e.path(), &EXTENSIONS).is_ok())
        .map(|e| e.path().to_path_buf())
        .collect();

    // Raise error if impossible to find duplicates
    if video_files.len() < 2 {
        return Err(anyhow::Error::msg("Directory contains less than 2 files"));
    }

    // Calculate hashes
    let hashes: Vec<(String, PathBuf)> = video_files
        .par_iter()
        .filter_map(|file| {
            process_video(file).ok().map(|hash| (hash, file.clone()))
        })
        .collect();

    // Group files by hash to find duplicates
    let mut hash_map: HashMap<String, Vec<PathBuf>> = HashMap::new();
    for (hash, path) in hashes {
        hash_map.entry(hash).or_default().push(path);
    }

    // Print duplicates
    let mut found_duplicates = false;
    for (hash, files) in hash_map.iter() {
        if files.len() > 1 {
            found_duplicates = true;
            println!("Duplicate files with hash {}:", hash);
            for file in files {
                println!("  {}", file.display());
            }
            println!();
        }
    }

    if !found_duplicates {
        println!("No duplicate files found.");
    }

    Ok(())
}

fn process_video(path: &Path) -> Result<String> {
    // Open file
    let file = File::open(path).with_context(|| "Impossible to open file")?;

    // Init buffer
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = vec![0; 1024 * 1024];

    // Read bytes untile there are left
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    // Finalise the hash
    let result = hasher.finalize();

    // Return the hash as a result
    Ok(format!("{:x}", result))
}
