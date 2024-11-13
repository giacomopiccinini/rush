use anyhow::{Context, Result};
use hound::{SampleFormat, WavReader, WavSpec, WavWriter};
use rayon::prelude::*;
use rubato::{FftFixedIn, Resampler};
use std::fs::{copy, File};
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::utils::{file_has_right_extension, perform_io_sanity_check};
use crate::AudioResampleArgs;

// Admissible extensions for this command
const EXTENSIONS: [&str; 1] = ["wav"];

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

/// Process all the content (single file or directory of files)
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
            let relative_path = file
                .strip_prefix(input)
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

/// Read samples and convert to f64
fn read_samples(
    reader: &mut WavReader<BufReader<File>>,
    channels: usize,
    bits_per_sample: u16,
) -> Result<Vec<Vec<f64>>> {
    // Init samples vec
    let mut samples: Vec<Vec<f64>> = vec![Vec::new(); channels];

    // Calculate the maximum value based on bits_per_sample
    let max_value = 2_f64.powi(bits_per_sample as i32 - 1);

    // Read into samples vec
    reader
        .samples::<i32>()
        .map(|s| s.with_context(|| "Couldn't read samples".to_string()))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .enumerate()
        .for_each(|(i, &sample)| {
            // Normalize by dividing by max_value
            samples[i % channels].push(sample as f64 / max_value);
        });

    Ok(samples)
}

/// Process a single file
fn process_file(input: &Path, sr: u32, output: &Path, overwrite: bool) -> Result<()> {
    // Check that we can overwrite
    if input == output && !overwrite {
        return Err(anyhow::Error::msg("Can't overwrite files"));
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

    // Read samples
    let samples = read_samples(&mut reader, channels, spec.bits_per_sample)
        .with_context(|| "Couldn't read file")?;

    // Initialize the resampler
    let mut resampler = FftFixedIn::<f64>::new(
        original_sr as usize,
        sr as usize,
        samples[0].len(), // Number of frames per channel
        1024,
        channels,
    )
    .with_context(|| "Can't initiate resampler")?;

    // Perform the resampling
    let resampled_64 = resampler
        .process(&samples, None)
        .with_context(|| "Can't resample file")?;

    // Create a new WAV specification for the resampled audio
    let resampled_spec = WavSpec {
        channels: channels as u16,
        sample_rate: sr,
        bits_per_sample: spec.bits_per_sample,
        sample_format: SampleFormat::Int,
    };

    // Init writer
    let mut writer = WavWriter::create(output, resampled_spec)
        .with_context(|| format!("Couldn't write to {:?}", output))?;

    // Calculate the max value based on bits_per_sample for proper scaling
    let max_value = 2_f64.powi((spec.bits_per_sample - 1) as i32);

    // Write samples interleaved
    for i in 0..resampled_64[0].len() {
        for channel in &resampled_64 {
            // Scale back to the appropriate integer range
            let scaled_sample = (channel[i] * max_value).round();

            // Write sample based on bits_per_sample
            match spec.bits_per_sample {
                8 => writer.write_sample(scaled_sample as i8)?,
                16 => writer.write_sample(scaled_sample as i16)?,
                24 | 32 => writer.write_sample(scaled_sample as i32)?,
                _ => {
                    return Err(anyhow::Error::msg(format!(
                        "Unsupported bits per sample: {}",
                        spec.bits_per_sample
                    )))
                }
            }
        }
    }

    Ok(())
}
