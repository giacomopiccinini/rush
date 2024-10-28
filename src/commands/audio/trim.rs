use std::path::Path;
use hound::{WavReader, WavWriter, WavSpec, SampleFormat};
use rayon::prelude::*;
use anyhow::{Context, Result};
use walkdir::WalkDir;
use std::path::PathBuf;

use crate::utils::{file_has_right_extension, perform_io_sanity_check};
use crate::AudioTrimArgs;

// Admissible extensions for this command
const EXTENSIONS : [&str; 1] = ["wav"];

pub fn execute(args: AudioTrimArgs) {

    // Parse the arguments
    let input = Path::new(&args.input);
    let output = Path::new(&args.output);

    let offset: f32 = args.offset;
    let length: f32 = args.length;

    let overwrite: bool = args.overwrite;

    // Sanity checks on I/O
    if let Err(e) = perform_io_sanity_check(input, output, false) {
        eprintln!("Error - Can't proceed. Reason: {}", e);
    }

    // Process files
    if let Err(e) = process(input, offset, length, output, overwrite) {
        eprintln!("Error - Can't process. Error chain:");
        for (i, cause) in e.chain().enumerate() {
            eprintln!("  Cause {}: {}", i, cause);
        }
    }
}


// Process all the content (single file or directory of files)
fn process(input: &Path, offset: f32, length: f32, output: &Path, overwrite: bool) -> Result<()> {

    // Case of single input file
    if input.is_file() {
        // Check if the file has the right extension and process it
        file_has_right_extension(input, &EXTENSIONS)?;
        process_file(input, offset, length, output, overwrite)
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
            process_file(file, offset, length, &file_output, overwrite)
                .with_context(|| format!("Failed to process file: {:?}", file))?;

            Ok(())
        })?;
    } 
    Ok(())
}

// Process a single file
fn process_file(input: &Path, offset: f32, length: f32, output: &Path, overwrite: bool) -> Result<()> {

    // Check that we can overwrite
    if input == output && !overwrite {
        return Err(anyhow::Error::msg("Can't overwrite files"))
    }

    // Open the WAV file
    let mut reader = WavReader::open(input).with_context(|| "Failed to open WavReader")?;

    // Extract info from file
    let spec = reader.spec();
    let sample_rate = spec.sample_rate as f32;
    let channels = spec.channels as usize;

    // Compute the requested length in samples
    let offset_samples = (sample_rate * offset) as usize * channels;
    let length_samples = (sample_rate * length) as usize * channels;

    // Read the samples and find the total number of samples
    let samples: Vec<i32> = reader
        .samples::<i32>()
        .collect::<Result<Vec<i32>, _>>().with_context(|| format!("Couldn't read samples from {:?}", input))?;
    let total_samples = samples.len();

    // Raise error if offset longer than file length
    if offset_samples > total_samples {
        Err(anyhow::Error::msg("Requested offset larger than file length"))
    }
    // Raise error if combined offset and length is longer than file length
    else if offset_samples + length_samples > total_samples {
        Err(anyhow::Error::msg("Requested length larger than file length"))
    }
    else{
        // Trim the audio
        let trimmed_samples : Vec<i32> = samples[offset_samples..length_samples].to_vec();

        // Define file specs
        let spec = WavSpec {
            channels: spec.channels,
            sample_rate: spec.sample_rate,
            bits_per_sample: spec.bits_per_sample,
            sample_format: SampleFormat::Int,
        };

        // Init writer
        let mut writer = WavWriter::create(output, spec).with_context(|| format!("Couldn't write to {:?}", output))?;

        trimmed_samples.iter()
            .try_for_each(|&sample| {
                writer.write_sample(sample)
                    .with_context(|| "Failed to write audio sample")
            })?;

        Ok(())
    }
}
