use rayon::prelude::*;
use std::collections::HashSet;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::ImagesumArgs;

// Execute the imagesum command
pub fn execute(args: ImagesumArgs) {
    let target = PathBuf::from(&args.target);
    let image_extensions: HashSet<_> = ["jpg", "jpeg", "png", "bmp", "gif", "tiff"]
        .iter().map(|&s| s.to_lowercase()).collect();

    if !target.is_dir() && !is_image_file(&target, &image_extensions) {
        println!("Target is neither a directory nor an image file.");
        return;
    }

    let (counter, dimensions) = if target.is_dir() {
        process_directory(&target, &image_extensions)
    } else {
        process_single_image(&target)
    };

    // Print the results
    println!("Total images: {}", counter);
    println!("Unique (height, width) pairs: {:?}", dimensions);
}

// Check if the file is an image file
fn is_image_file(path: &PathBuf, extensions: &HashSet<String>) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| extensions.contains(&ext.to_lowercase()))
        .unwrap_or(false)
}

fn process_directory(target: &PathBuf, extensions: &HashSet<String>) -> (usize, HashSet<(u32, u32)>) {
    let results: Vec<_> = WalkDir::new(target)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| !entry.file_type().is_dir())
        .filter(|entry| is_image_file(&entry.path().to_path_buf(), extensions))
        .par_bridge()
        .filter_map(|entry| process_image(entry.path()))
        .collect();

    let counter = results.len();
    let dimensions: HashSet<_> = results.into_iter().collect();
    (counter, dimensions)
}

// If image succesfully processed, return (1, {height, width})
// If not, return (0, {})
fn process_single_image(path: &PathBuf) -> (usize, HashSet<(u32, u32)>) {
    process_image(path)
        .map(|dims| (1, [dims].into_iter().collect()))
        .unwrap_or((0, HashSet::new()))
}

/// Process a single image file and return its dimensions.
fn process_image(path: &std::path::Path) -> Option<(u32, u32)> {

    image::image_dimensions(path)
        .map(|(width, height)| (height, width))
        .ok()

}
