use image::io::Reader as ImageReader;
use image::imageops::resize;
use image::imageops::FilterType;
use std::path::PathBuf;
use std::collections::HashSet;
use rayon::prelude::*;

use crate::ResizeArgs;


// Check if the file is an image file
fn is_image_file(path: &PathBuf, extensions: &HashSet<String>) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| extensions.contains(&ext.to_lowercase()))
        .unwrap_or(false)
}

// Resize one image
fn resize_image(input_path: &PathBuf, output_path: &PathBuf, width: u32, height: u32, filter: FilterType) {
    let input_img = ImageReader::open(&input_path).unwrap().decode().unwrap();
    let output_img = resize(&input_img, width, height, filter);
    output_img.save(&output_path).unwrap();
}

// Resize directory
fn resize_directory(input_paths: Vec<PathBuf>, output_paths: Vec<PathBuf>, width: u32, height: u32, filter: FilterType) {
    input_paths.into_par_iter()
        .zip(output_paths)
        .for_each(|(input, output)| {
            resize_image(&input, &output, width, height, filter);
        });
}

// Execute the resize command
pub fn execute(args: ResizeArgs) {

    // Convert to path
    let target = PathBuf::from(&args.target);
    let output = PathBuf::from(&args.output);

    // Sanity check
    if target.is_dir() && output.is_file(){
        println!("ERROR: Output cannot be a file if input is a directory.");
        return;
    }
    
    // Check if target and output are the same
    if target == output {
        println!("Warning: The target and output paths are the same. This will overwrite the original files.");
        println!("Do you want to continue? (y/n)");
        
        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input).expect("Failed to read line");
        
        if user_input.trim().to_lowercase() != "y" {
            println!("Operation cancelled.");
            return;
        }
    }

    // Define allowed extensions for images
    let image_extensions: HashSet<_> = ["jpg", "jpeg", "png", "bmp", "gif", "tiff"]
    .iter().map(|&s| s.to_lowercase()).collect();

    // Collect input files based on type of target
    let input_files = if target.is_file() {
        // If it's a file, check if it has the right extension
        if is_image_file(&target, &image_extensions) {
            vec![target.clone()]
        } else {
            println!("The provided file is not a supported image format.");
            return;
        }
    } else if target.is_dir() {
        // If it's a directory, collect all files with the right extensions
        std::fs::read_dir(&target)
            .expect("Failed to read directory")
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| path.is_file() && is_image_file(path, &image_extensions))
            .collect()
    } else {
        println!("The provided path is neither a file nor a directory.");
        return;
    };

    // Check if we found any valid image files
    if input_files.is_empty() {
        println!("No valid image files found.");
        return;
    }

    // Define the output files
    let output_files = if output.is_dir(){
        input_files.iter().map(|input_path| output.join(input_path.file_name().unwrap())).collect()
    } else {
        vec![output]
    };

    // Resize directory
    resize_directory(input_files, output_files, args.width, args.height, FilterType::Lanczos3);

}

