use std::fs;
use std::path::Path;
use hound::{WavReader, WavWriter, WavSpec, SampleFormat};
use rayon::prelude::*;
use rubato::{FftFixedInOut, Resampler};

use crate::AudioResampleArgs;

pub fn execute(args: AudioResampleArgs) {

    // Parse the arguments
    let input_path = Path::new(&args.input);  // Convert the input path string to a Path
    let target_sample_rate: u32 = args.sr;
    let output_directory_path = Path::new(&args.output);  // Convert the output directory path string to a Path
    let replace_original: bool = args.replace_original;

    // Perform sanity checks on the output directory
    if !output_directory_path.exists() {
        // If the output directory doesn't exist, create it
        fs::create_dir_all(output_directory_path).expect("Failed to create output directory");
    } else if output_directory_path.is_file() {
        // If the output path is a file instead of a directory, print an error and exit
        eprintln!("The provided output directory path is a file");
        return;
    }

    // Start processing the files
    process_path(input_path, target_sample_rate, output_directory_path, replace_original);
}

fn process_path(path: &Path, target_sample_rate: u32, output_directory_path: &Path, replace_original: bool) {
    if path.is_file() {
        // If the path is a file, check if it's a WAV file and process it
        if path.extension().and_then(|ext| ext.to_str()).map(|s| s.to_lowercase()) == Some("wav".to_string()) {
            resample_file(path, target_sample_rate, output_directory_path, replace_original);
        }
    } else if path.is_dir() {
        // If the path is a directory, read all entries in the directory
        let entries: Vec<_> = fs::read_dir(path)
            .expect("Failed to read directory")
            .collect::<Result<Vec<_>, _>>()
            .expect("Failed to collect directory entries");

        // Process each entry in parallel using rayon
        entries.par_iter().for_each(|entry| {
            let entry_path = entry.path();
            let relative_path = entry_path.strip_prefix(path).unwrap();
            let new_output_path = output_directory_path.join(relative_path);

            if entry_path.is_dir() {
                // If the entry is a directory, create a corresponding output directory and process it recursively
                fs::create_dir_all(&new_output_path).expect("Failed to create output subdirectory");
                process_path(&entry_path, target_sample_rate, &new_output_path, replace_original);
            } else if entry_path.extension().and_then(|ext| ext.to_str()).map(|s| s.to_lowercase()) == Some("wav".to_string()) {
                // If the entry is a WAV file, process it
                resample_file(&entry_path, target_sample_rate, &new_output_path.parent().unwrap(), replace_original);
            }
        });
    } else {
        // If the path is neither a file nor a directory, print an error
        eprintln!("The provided path is neither a file nor a directory");
    }
}

fn resample_file(path: &Path, target_sample_rate: u32, output_directory_path: &Path, replace_original: bool) {
    // Open the WAV file for reading
    let mut reader = WavReader::open(&path).expect("Failed to open WAV file");

    // Extract information from the WAV file
    let spec = reader.spec();
    let channels = spec.channels as usize;
    let original_sample_rate = spec.sample_rate;

    // If the original sample rate is the same as the target, no need to resample
    if original_sample_rate == target_sample_rate {
        return;
    }

    // Read all samples from the WAV file
    let samples: Vec<i32> = reader.samples::<i32>().map(|s| s.unwrap()).collect();

    // Convert samples to f64 and deinterleave into separate channels
    let mut input: Vec<Vec<f64>> = vec![Vec::new(); channels];
    for (i, &sample) in samples.iter().enumerate() {
        input[i % channels].push(sample as f64 / i32::MAX as f64);
    }

    // Initialize the resampler
    let input_frames = input[0].len(); // Number of frames per channel
    let mut resampler = FftFixedInOut::<f64>::new(
        original_sample_rate as usize,
        target_sample_rate as usize,
        input_frames,
        channels,
    ).unwrap();

    // Perform the resampling
    let output = resampler.process(&input, None).unwrap();

    // Convert the resampled data back to i32 and interleave channels
    let num_channels = output.len();
    let num_samples_per_channel = output[0].len();
    let mut resampled: Vec<i32> = Vec::with_capacity(num_samples_per_channel * num_channels);
    for i in 0..num_samples_per_channel {
        for ch in 0..num_channels {
            let sample = output[ch][i];
            resampled.push((sample * i32::MAX as f64) as i32);
        }
    }

    // Create a new WAV specification for the resampled audio
    let new_spec = WavSpec {
        channels: spec.channels,
        sample_rate: target_sample_rate,
        bits_per_sample: spec.bits_per_sample,
        sample_format: SampleFormat::Int,
    };

    // Determine the output path for the resampled file
    let output_path = if replace_original {
        path.to_path_buf()
    } else {
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let new_file_name = format!("{}_{}Hz.wav", file_name.trim_end_matches(".wav"), target_sample_rate);
        output_directory_path.join(new_file_name)
    };

    // Write the resampled data to a new WAV file
    let mut writer = WavWriter::create(&output_path, new_spec).expect("Failed to create WAV file for writing");
    for sample in resampled {
        writer.write_sample(sample).unwrap();
    }

    // Finalize the writer to ensure all data is written
    writer.finalize().expect("Failed to finalize WAV file");

    // If replacing the original and the output path is different, remove the original file
    if replace_original && output_path != path {
        fs::remove_file(path).expect("Failed to remove original file");
    }
}
