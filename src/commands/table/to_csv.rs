use anyhow::{Context, Result};
use polars::prelude::*;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::utils::{file_has_right_extension, perform_io_sanity_check};

// Admissible extensions for this command
const EXTENSIONS: [&str; 1] = ["parquet"];

use crate::TableToCsvArgs;

// Execute the resize command
pub fn execute(args: TableToCsvArgs) -> Result<()> {
    // Parse the arguments
    let input = Path::new(&args.input);
    let output = Path::new(&args.output);

    // Sanity checks on I/O
    perform_io_sanity_check(input, output, false, true).with_context(|| "Sanity check failed")?;

    // Process files
    process(input, output).with_context(|| "Processing failed")?;

    Ok(())
}

// Process all the content (single file or directory of files)
fn process(input: &Path, output: &Path) -> Result<()> {
    // Case of single input file
    if input.is_file() {
        // Check if the file has the right extension and process it
        file_has_right_extension(input, &EXTENSIONS)?;
        process_file(input, output)
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
            let relative_path = file
                .strip_prefix(input)
                .with_context(|| format!("Failed to strip prefix from path: {:?}", file))?;

            // Nested output path with .parquet extension
            let file_output = output.join(relative_path).with_extension("csv");

            // Ensure the output directory exists
            if let Some(parent) = file_output.parent() {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create output directory: {:?}", parent))?;
            }

            // Process the file
            process_file(file, &file_output)
                .with_context(|| format!("Failed to process file: {:?}", file))?;

            Ok(())
        })?;
    }
    Ok(())
}

// Process a single file
fn process_file(input: &Path, output: &Path) -> Result<()> {
    // Read file
    let input_file =
        std::fs::File::open(input).with_context(|| format!("Couldn't open file {:?}", input))?;

    // Read df from CSV file
    let mut df = ParquetReader::new(input_file)
        .finish()
        .with_context(|| format!("Failed to finish parquet reading: {:?}", input))?;

    // Open output file
    let mut output_file = std::fs::File::create(output)
        .with_context(|| format!("Failed to open parquet file: {:?}", output))?;

    // Write file to parquet
    CsvWriter::new(&mut output_file)
        .finish(&mut df)
        .with_context(|| format!("Failed to write csv file: {:?}", output))?;

    Ok(())
}
