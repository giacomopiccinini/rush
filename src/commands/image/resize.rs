use image::io::Reader as ImageReader;
use image::imageops::{resize, FilterType};
use std::path::{PathBuf, Path};
use rayon::prelude::*;
use anyhow::{Context, Result};
use walkdir::WalkDir;

use crate::utils::{file_has_right_extension, perform_io_sanity_check};

// Admissible extensions for this command
const EXTENSIONS : [&str; 6] = ["jpg", "jpeg", "png", "bmp", "gif", "tiff"];

use crate::ImageResizeArgs;

// Execute the resize command
pub fn execute(args: ImageResizeArgs) -> Result<()> {

    // Parse the arguments
    let input = Path::new(&args.input);
    let output = Path::new(&args.output);

    let height: u32 = args.height;
    let width: u32 = args.width;

    let overwrite: bool = args.overwrite;

    // Sanity checks on I/O
    perform_io_sanity_check(input, output, false, true).with_context(|| "Sanity check failed")?;

    // Process files
    process(input, height, width, output, overwrite).with_context(|| "Processing failed")?;

    Ok(())

}

// Process all the content (single file or directory of files)
fn process(input: &Path, height: u32, width: u32, output: &Path, overwrite: bool) -> Result<()> {

    // Case of single input file
    if input.is_file() {
        // Check if the file has the right extension and process it
        file_has_right_extension(input, &EXTENSIONS)?;
        process_file(input, height, width, output, overwrite)
            .with_context(|| format!("Failed to process file: {:?}", input))?;
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
            let relative_path = file.strip_prefix(input)
                .with_context(|| format!("Failed to strip prefix from path: {:?}", file))?;

            // Nested output path
            let file_output = output.join(relative_path);

            // Ensure the output directory exists
            if let Some(parent) = file_output.parent() {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create output directory: {:?}", parent))?;
            }

            // Process the file
            process_file(file, height, width, &file_output, overwrite)
                .with_context(|| format!("Failed to process file: {:?}", file))?;

            Ok(())
        })?;
    } 
    Ok(())
}

// Process a single file
fn process_file(input: &Path, height: u32, width: u32, output: &Path, overwrite: bool) -> Result<()>{

    // Check that we can overwrite
    if input == output && !overwrite {
        return Err(anyhow::Error::msg("Can't overwrite files"))
    }

    // Read image
    let input_img = ImageReader::open(input).with_context(|| "Can't open image")?.decode().with_context(|| "Can't decode image")?;

    // Resize image
    let output_img = resize(&input_img, width, height, FilterType::Lanczos3);

    // Save image
    output_img.save(&output).with_context(|| format!("Couldn't save image to {:?}", output))?;

    Ok(())
}

