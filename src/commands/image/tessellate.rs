use image::io::Reader as ImageReader;
use std::path::PathBuf;
use std::collections::HashSet;
use rayon::prelude::*;

use crate::ImageTessellateArgs;


// Check if the file is an image file
fn is_image_file(path: &PathBuf, extensions: &HashSet<String>) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| extensions.contains(&ext.to_lowercase()))
        .unwrap_or(false)
}

// Tessellate one image
fn tessellate_image(input_path: &PathBuf, output: &PathBuf, n_vertical: u32, n_horizontal: u32) {
    // Open and decode the input image
    let input_img = ImageReader::open(input_path).unwrap().decode().unwrap();

    // Get image dimensions
    let image_width = input_img.width();
    let image_height = input_img.height();

    // Compute the dimensions of each crop (patch)
    let patch_width = image_width / n_horizontal;
    let patch_height = image_height / n_vertical;

    // Calculate the remaining pixels that can't be evenly distributed
    let patch_width_left_over = image_width % n_horizontal;
    let patch_height_left_over = image_height % n_vertical;

    // Initialize vectors to store the start and end coordinates for each patch
    let mut horizontal_coordinates = vec![[0u32; 2]; n_horizontal as usize];
    let mut vertical_coordinates = vec![[0u32; 2]; n_vertical as usize];

    // Populate initial coordinates assuming equal distribution
    for i in 0..n_horizontal as usize {
        horizontal_coordinates[i] = [i as u32 * patch_width, (i as u32 + 1) * patch_width];
    }

    for i in 0..n_vertical as usize {
        vertical_coordinates[i] = [i as u32 * patch_height, (i as u32 + 1) * patch_height];
    }

    // Distribute the remaining pixels among patches
    // This ensures that some patches are 1 pixel wider/taller to account for all pixels
    for i in 0..patch_height_left_over as usize {
        vertical_coordinates[i][1] += 1;
        for j in (i + 1)..n_vertical as usize {
            vertical_coordinates[j][0] += 1;
            vertical_coordinates[j][1] += 1;
        }
    }

    for i in 0..patch_width_left_over as usize {
        horizontal_coordinates[i][1] += 1;
        for j in (i + 1)..n_horizontal as usize {
            horizontal_coordinates[j][0] += 1;
            horizontal_coordinates[j][1] += 1;
        }
    }

    // Convert relative coordinates to absolute coordinates
    // This step accumulates the widths/heights to get the actual pixel positions
    let mut cumsum = 0;
    for coord in vertical_coordinates.iter_mut() {
        cumsum += coord[1] - coord[0];
        coord[1] = cumsum;
    }

    cumsum = 0;
    for coord in horizontal_coordinates.iter_mut() {
        cumsum += coord[1] - coord[0];
        coord[1] = cumsum;
    }

    // Looping over all patches
    for (index, (vertical_slice, horizontal_slice)) in vertical_coordinates.iter().flat_map(|v| horizontal_coordinates.iter().map(move |h| (v, h))).enumerate() {
        // Create the correct filename
        let temp_filename = format!(
            "{}_id{}_w{}-{}_h{}-{}.{}",
            input_path.file_stem().unwrap().to_str().unwrap(),
            index,
            horizontal_slice[0],
            horizontal_slice[1],
            vertical_slice[0],
            vertical_slice[1],
            input_path.extension().unwrap().to_str().unwrap()
        );

        // Take the corresponding patch
        let patch = input_img.crop_imm(
            horizontal_slice[0],
            vertical_slice[0],
            horizontal_slice[1] - horizontal_slice[0],
            vertical_slice[1] - vertical_slice[0]
        );

        // Save the images
        let output_path = output.join(temp_filename);
        patch.save(output_path).unwrap();
    }

}

// Tessellate directory
fn tessellate_directory(input_paths: Vec<PathBuf>, output: &PathBuf, n_vertical: u32, n_horizontal: u32) {
    input_paths.into_par_iter()
        .for_each(|input_path| {
            tessellate_image(&input_path, output, n_vertical, n_horizontal);
        });
}

// Execute the resize command
pub fn execute(args: ImageTessellateArgs) {

    // Convert to path
    let target = PathBuf::from(&args.target);
    let output = PathBuf::from(&args.output);

    if !output.exists() {
        // Create output directory if it does not exist
        std::fs::create_dir_all(output.clone()).unwrap();
    }

    // Check if output is a directory
    if !output.is_dir() {
        println!("ERROR: Output must be a directory.");
        return;
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

    // Resize directory
    tessellate_directory(input_files, &output, args.n_vertical, args.n_horizontal);

}

