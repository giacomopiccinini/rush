use anyhow::{Context, Result};
use image::image_dimensions;
use rayon::prelude::*;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::utils::file_has_right_extension;

use crate::ImageSummaryArgs;

// Admissible extensions for this command
const EXTENSIONS: [&str; 7] = ["jpg", "jpeg", "png", "bmp", "gif", "tiff", "tif"];

pub fn execute(args: ImageSummaryArgs) -> Result<()> {
    // Parse the arguments
    let target = Path::new(&args.target);

    // Error if it does not exist at all
    if !target.exists() {
        return Err(anyhow::Error::msg(
            "Target file or directory does not exist",
        ));
    }

    // Find all admissible files
    let files: Vec<PathBuf> = match target.is_file() {
        true => {
            if file_has_right_extension(target, &EXTENSIONS).is_ok() {
                vec![target.to_path_buf()]
            } else {
                vec![]
            }
        }
        false => WalkDir::new(target)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| file_has_right_extension(e.path(), &EXTENSIONS).is_ok())
            .map(|e| e.path().to_path_buf())
            .collect(),
    };

    // Raise error if no files are admissible
    if files.is_empty() {
        return Err(anyhow::Error::msg("No admissible image files detected"));
    }

    // Process files
    let info: Vec<(u32, u32)> = files
        .into_par_iter()
        .filter_map(|file| process_image(&file).ok())
        .collect();

    // Calculate total number of files
    let n_files = info.len();

    // Get unique values
    let unique_shapes: HashSet<_> = info.into_iter().collect();

    // Print results
    println!("Total files: {}", n_files);
    println!("Unique (height, width) pairs: {:?}", unique_shapes);

    Ok(())
}

// Function for getting relevant info of an image file by just probing it
fn process_image(path: &Path) -> Result<(u32, u32)> {
    image_dimensions(path)
        .map(|(width, height)| (height, width))
        .with_context(|| "Error extracting image dimensions")
}
