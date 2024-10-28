use std::fs;
use std::path::Path;
use hound::{WavReader, WavWriter, WavSpec, SampleFormat};
use rayon::prelude::*;
use walkdir::WalkDir;
use std::path::PathBuf;
use anyhow::{Context, Result};

use crate::utils::{file_has_right_extension, perform_io_sanity_check};
use crate::AudioSplitArgs;

// Admissible extensions for this command
const EXTENSIONS : [&str; 1] = ["wav"];

pub fn execute(args: AudioSplitArgs) -> Result<()>{

    // Parse the arguments
    let input = Path::new(&args.input);
    let output = Path::new(&args.output);

    let chunk_duration_sec: f32 = args.chunk_duration;

    let delete_original: bool = args.delete_original;

    // Sanity checks on I/O
    perform_io_sanity_check(input, output, false, false).with_context(|| "Sanity check failed")?;

    // Process files
    process(input, chunk_duration_sec, output, delete_original).with_context(|| "Processing failed")?;

    Ok(())

}

// Process all the content (single file or directory of files)
fn process(input: &Path, chunk_duration_sec: f32, output: &Path, delete_original: bool) -> Result<()> {

    // Case of single input file
    if input.is_file() {
        // Check if the file has the right extension and process it
        file_has_right_extension(input, &EXTENSIONS)?;
        process_file(input, chunk_duration_sec, output)
            .with_context(|| format!("Failed to process file: {:?}", input))?;
        if delete_original {
            fs::remove_file(input).with_context(|| format!("Failed to delete file: {:?}", input))?;
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
            let relative_path = file.strip_prefix(input)
                .with_context(|| format!("Failed to strip prefix from path: {:?}", file))?;

            let joined_path = output.join(relative_path);
            let output_directory = joined_path.parent()
                .with_context(|| format!("Failed to get parent directory of: {:?}", relative_path))?;

            // Create output directory
            std::fs::create_dir_all(output_directory)
                .with_context(|| format!("Failed to create output directory: {:?}", output_directory))?;

            // Process the file
            process_file(file, chunk_duration_sec, output_directory)
                .with_context(|| format!("Failed to process file: {:?}", file))?;

            if delete_original {
                fs::remove_file(file).with_context(|| format!("Failed to delete file: {:?}", file))?;
            }

            Ok(())
        })?;
    } 
    Ok(())
}

// Process a single file
fn process_file(input: &Path, chunk_duration_sec: f32, output: &Path) -> Result<()> {

    // Open the WAV file
    let mut reader = WavReader::open(input).with_context(|| "Failed to open WavReader")?;

    // Extract info from file
    let spec = reader.spec();
    let sample_rate = spec.sample_rate as f32;
    let channels = spec.channels as usize;

    // Compute the expected size in samples for a chunk
    let chunk_size = (sample_rate * chunk_duration_sec) as usize * channels;
    
    // Read the samples and find the total number of samples
    let samples: Vec<i32> = reader
        .samples::<i32>()
        .collect::<Result<Vec<i32>, _>>().with_context(|| format!("Couldn't read samples from {:?}", input))?;
    let total_samples = samples.len();

    // Calculate the number of chunks the file will be split into
    let num_chunks = (total_samples + chunk_size - 1) / chunk_size;

    // Calculate the number of digits needed when padding the name with 0's
    let padding_width = format!("{}", num_chunks - 1).len();

    // Calculate the stem
    let stem = input.file_stem()
        .with_context(|| format!("Failed to extract stem from: {:?}", input))?
        .to_str()
        .with_context(|| format!("Failed to convert stem to string for: {:?}", input))?;

    for i in 0..num_chunks {

        // Determine start and end of each chunk wrt original file
        let start = i * chunk_size;
        let end = usize::min(start + chunk_size, total_samples);

        // Extract the samples for that chunk
        let mut chunk_samples: Vec<i32> = samples[start..end].to_vec();

        // Pad with zeros if the chunk is not full
        if chunk_samples.len() < chunk_size {
            chunk_samples.extend(vec![0; chunk_size - chunk_samples.len()]);
        }

        // Define the output path for the chunk
        let output_path = output.join(format!("{}@{:0width$}.wav", stem, i, width = padding_width));

        // Define the specs for writing to file
        let spec = WavSpec {
            channels: spec.channels,
            sample_rate: spec.sample_rate,
            bits_per_sample: spec.bits_per_sample,
            sample_format: SampleFormat::Int,
        };

        // Init writer
        let mut writer = WavWriter::create(&output_path, spec).with_context(|| format!("Couldn't write to {:?}", output_path))?;

        // Write to file
        chunk_samples.iter()
            .try_for_each(|&sample| {
                writer.write_sample(sample)
                    .with_context(|| "Failed to write audio sample")
            })?;

    }

    Ok(())
}
