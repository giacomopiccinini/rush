use anyhow::{Context, Result};
use image::io::Reader as ImageReader;
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::utils::{file_has_right_extension, perform_io_sanity_check};

// Admissible extensions for this command
const EXTENSIONS: [&str; 6] = ["jpg", "jpeg", "png", "bmp", "gif", "tiff"];

use crate::ImageTessellateArgs;

// Execute the resize command
pub fn execute(args: ImageTessellateArgs) -> Result<()> {
    // Parse the arguments
    let input = Path::new(&args.input);
    let output = Path::new(&args.output);

    let n_vertical: u32 = args.n_vertical;
    let n_horizontal: u32 = args.n_horizontal;

    let delete_original: bool = args.delete_original;

    // Sanity checks on I/O
    perform_io_sanity_check(input, output, false, false).with_context(|| "Sanity check failed")?;

    // Process files
    process(input, n_vertical, n_horizontal, output, delete_original)
        .with_context(|| "Processing failed")?;

    Ok(())
}

// Process all the content (single file or directory of files)
fn process(
    input: &Path,
    n_vertical: u32,
    n_horizontal: u32,
    output: &Path,
    delete_original: bool,
) -> Result<()> {
    // Case of single input file
    if input.is_file() {
        // Check if the file has the right extension and process it
        file_has_right_extension(input, &EXTENSIONS)?;
        process_file(input, n_vertical, n_horizontal, output)
            .with_context(|| format!("Failed to process file: {:?}", input))?;
        if delete_original {
            fs::remove_file(input)
                .with_context(|| format!("Failed to delete file: {:?}", input))?;
        }
    }
    // Case of input being a directory
    else {
        // Find all files
        let files: Vec<PathBuf> = WalkDir::new(input)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| file_has_right_extension(e.path(), &EXTENSIONS).is_ok())
            .map(|e| e.path().to_path_buf())
            .collect();

        // Parallel loop over entries
        files.par_iter().try_for_each(|file| -> Result<()> {
            // Relative path wrt input directory
            let relative_path = file
                .strip_prefix(input)
                .with_context(|| format!("Failed to strip prefix from path: {:?}", file))?;

            let joined_path = output.join(relative_path);
            let output_directory = joined_path.parent().with_context(|| {
                format!("Failed to get parent directory of: {:?}", relative_path)
            })?;

            // Create output directory
            std::fs::create_dir_all(output_directory).with_context(|| {
                format!("Failed to create output directory: {:?}", output_directory)
            })?;

            // Process the file
            process_file(file, n_vertical, n_horizontal, output_directory)
                .with_context(|| format!("Failed to process file: {:?}", file))?;

            if delete_original {
                fs::remove_file(file)
                    .with_context(|| format!("Failed to delete file: {:?}", file))?;
            }

            Ok(())
        })?;
    }
    Ok(())
}

// Process one file
fn process_file(input: &Path, n_vertical: u32, n_horizontal: u32, output: &Path) -> Result<()> {
    // Read image
    let input_img = ImageReader::open(input)
        .with_context(|| "Can't open image")?
        .decode()
        .with_context(|| "Can't decode image")?;

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

    // Populate initial coordinates using iterators
    horizontal_coordinates
        .iter_mut()
        .enumerate()
        .take(n_horizontal as usize)
        .for_each(|(i, coord)| {
            *coord = [i as u32 * patch_width, (i as u32 + 1) * patch_width];
        });

    vertical_coordinates
        .iter_mut()
        .enumerate()
        .take(n_vertical as usize)
        .for_each(|(i, coord)| {
            *coord = [i as u32 * patch_height, (i as u32 + 1) * patch_height];
        });

    // Distribute remaining pixels using iterators
    for i in 0..patch_height_left_over as usize {
        vertical_coordinates[i][1] += 1;
        vertical_coordinates
            .iter_mut()
            .skip(i + 1)
            .take(n_vertical as usize - (i + 1))
            .for_each(|coord| {
                coord[0] += 1;
                coord[1] += 1;
            });
    }

    for i in 0..patch_width_left_over as usize {
        horizontal_coordinates[i][1] += 1;
        horizontal_coordinates
            .iter_mut()
            .skip(i + 1)
            .take(n_horizontal as usize - (i + 1))
            .for_each(|coord| {
                coord[0] += 1;
                coord[1] += 1;
            });
    }

    // Convert to absolute coordinates using fold
    let mut cumsum = 0;
    vertical_coordinates.iter_mut().for_each(|coord| {
        cumsum += coord[1] - coord[0];
        coord[1] = cumsum;
    });

    cumsum = 0;
    horizontal_coordinates.iter_mut().for_each(|coord| {
        cumsum += coord[1] - coord[0];
        coord[1] = cumsum;
    });

    let stem = input
        .file_stem()
        .with_context(|| "Can't extract stem")?
        .to_str()
        .with_context(|| "Can't convert to string")?;
    let ext = input
        .extension()
        .with_context(|| "Can't extract extension")?
        .to_str()
        .with_context(|| "Can't convert to string")?;

    // Looping over all patches
    for (index, (vertical_slice, horizontal_slice)) in vertical_coordinates
        .iter()
        .flat_map(|v| horizontal_coordinates.iter().map(move |h| (v, h)))
        .enumerate()
    {
        // Create the correct filename
        let temp_filename = format!(
            "{}_id{}_w{}-{}_h{}-{}.{}",
            stem,
            index,
            horizontal_slice[0],
            horizontal_slice[1],
            vertical_slice[0],
            vertical_slice[1],
            ext
        );

        // Take the corresponding patch
        let patch = input_img.crop_imm(
            horizontal_slice[0],
            vertical_slice[0],
            horizontal_slice[1] - horizontal_slice[0],
            vertical_slice[1] - vertical_slice[0],
        );

        // Save the images
        let output_path = output.join(temp_filename);

        // Save image
        patch
            .save(&output_path)
            .with_context(|| format!("Couldn't save image to {:?}", output_path))?;
    }

    Ok(())
}
