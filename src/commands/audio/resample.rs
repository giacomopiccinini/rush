use std::fs::copy;
use std::path::Path;
use hound::{WavReader, WavWriter, WavSpec, SampleFormat};
use rayon::prelude::*;
use rubato::{FftFixedInOut, Resampler};
use walkdir::WalkDir;
use std::path::PathBuf;
use anyhow::{Context, Result};

use crate::utils::{file_has_right_extension, perform_io_sanity_check};
use crate::AudioResampleArgs;

// Admissible extensions for this command
const EXTENSIONS : [&str; 1] = ["wav"];

pub fn execute(args: AudioResampleArgs) -> Result<()> {

    // Parse the arguments
    let input = Path::new(&args.input);
    let output = Path::new(&args.output);

    let sr: u32 = args.sr;

    let overwrite: bool = args.overwrite;

    // Sanity checks on I/O
    perform_io_sanity_check(input, output, false, true).with_context(|| "Sanity check failed")?;

    // Process files
    process(input, sr, output, overwrite).with_context(|| "Processing failed")?;

    Ok(())
}

// Process all the content (single file or directory of files)
fn process(input: &Path, sr: u32, output: &Path, overwrite: bool) -> Result<()> {

    // Case of single input file
    if input.is_file() {
        // Check if the file has the right extension and process it
        file_has_right_extension(input, &EXTENSIONS)?;
        process_file(input, sr, output, overwrite)
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
            process_file(file, sr, &file_output, overwrite)
                .with_context(|| format!("Failed to process file: {:?}", file))?;

            Ok(())
        })?;
    } 
    Ok(())
}

// Process a single file
fn process_file(input: &Path, sr: u32, output: &Path, overwrite: bool) -> Result<()>{

    // Check that we can overwrite
    if input == output && !overwrite {
        return Err(anyhow::Error::msg("Can't overwrite files"))
    }

    // Open the WAV file
    let mut reader = WavReader::open(input).with_context(|| "Failed to open WavReader")?;

    // Extract info from file
    let spec = reader.spec();
    let original_sr = spec.sample_rate;
    let channels = spec.channels as usize;

    // If the original sample rate is the same as the target, no need to resample
    if original_sr == sr {

        // Just copy the file if input does not coincide with output
        if input != output {
            copy(input, output).with_context(|| "Failed to copy file")?;
        }

        return Ok(());
    }

    // Read samples, convert to f64, and deinterleave into channels in one pass
    let mut samples: Vec<Vec<f64>> = vec![Vec::new(); channels];
    reader
        .samples::<i32>()
        .map(|s| s.with_context(|| format!("Couldn't read samples from {:?}", input)))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .enumerate()
        .for_each(|(i, &sample)| {
            samples[i % channels].push(sample as f64 / i32::MAX as f64);
        });

    // Initialize the resampler
    let mut resampler = FftFixedInOut::<f64>::new(
        original_sr as usize,
        sr as usize,
        samples[0].len(),  // Number of frames per channel
        channels,
    ).with_context(|| "Can't initiate resampler")?;

    // Perform the resampling
    let resampled_64 = resampler.process(&samples, None).with_context(|| "Can't resample file")?;

    // Create a vector to store the interleaved i32 samples with pre-allocated capacity
    let mut resampled_32 = Vec::with_capacity(resampled_64[0].len() * channels);
    // Iterate through each frame
    resampled_32.extend(
        (0..resampled_64[0].len())
            .flat_map(|i| resampled_64.iter()
                .take(channels)
                .map(move |channel| (channel[i] * i32::MAX as f64) as i32))
    );

    // Create a new WAV specification for the resampled audio
    let resampled_spec = WavSpec {
        channels: channels as u16,
        sample_rate: sr,
        bits_per_sample: spec.bits_per_sample,
        sample_format: SampleFormat::Int,
    };

    // Init writer
    let mut writer = WavWriter::create(output, resampled_spec).with_context(|| format!("Couldn't write to {:?}", output))?;

    // Write to file
    resampled_32.iter()
        .try_for_each(|&sample| {
            writer.write_sample(sample)
                .with_context(|| "Failed to write audio sample")
        })?;

    Ok(())

}
