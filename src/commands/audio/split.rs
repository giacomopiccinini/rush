use std::fs;
use std::path::Path;
use hound::{WavReader, WavWriter, WavSpec, SampleFormat};
use rayon::prelude::*;

use crate::AudioSplitArgs;


pub fn execute(args: AudioSplitArgs) {

    // Parse the arguments
    let input_path = Path::new(&args.input);
    let chunk_duration_sec: f32 = args.chunk_duration;
    let output_directory_path = Path::new(&args.output);
    let delete_original: bool = args.delete_original;

    // Sanity checks on args
    if !output_directory_path.exists() {
        fs::create_dir_all(output_directory_path).expect("Failed to create output directory");
    } else if output_directory_path.is_file() {
        eprintln!("The provided output directory path is a file");
        return;
    }

    // Process files
    process_path(input_path, chunk_duration_sec, output_directory_path, delete_original);
}

fn process_path(path: &Path, chunk_duration_sec: f32, output_directory_path: &Path, delete_original: bool) {
    if path.is_file() {
        if path.extension().and_then(|ext| ext.to_str()).map(|s| s.to_lowercase()) == Some("wav".to_string()) {
            process_file(path, chunk_duration_sec, output_directory_path);
            if delete_original {
                fs::remove_file(path).expect("Failed to delete original file");
            }
        }
    } else if path.is_dir() {
        let entries: Vec<_> = fs::read_dir(path)
            .expect("Failed to read directory")
            .collect::<Result<Vec<_>, _>>()
            .expect("Failed to collect directory entries");

        entries.par_iter().for_each(|entry| {
            let entry_path = entry.path();
            let relative_path = entry_path.strip_prefix(path).unwrap();
            let new_output_path = output_directory_path.join(relative_path);

            if entry_path.is_dir() {
                fs::create_dir_all(&new_output_path).expect("Failed to create output subdirectory");
                process_path(&entry_path, chunk_duration_sec, &new_output_path, delete_original);
            } else if entry_path.extension().and_then(|ext| ext.to_str()).map(|s| s.to_lowercase()) == Some("wav".to_string()) {
                process_file(&entry_path, chunk_duration_sec, new_output_path.parent().unwrap());
                if delete_original {
                    fs::remove_file(&entry_path).expect("Failed to delete original file");
                }
            }
        });
    } else {
        eprintln!("The provided path is neither a file nor a directory");
    }
}

fn process_file(path: &Path, chunk_duration_sec: f32, output_directory_path: &Path) {
    // Open the WAV file
    let mut reader = WavReader::open(path).expect("Failed to open WAV file");

    // Extract info from file
    let spec = reader.spec();
    let sample_rate = spec.sample_rate as f32;
    let channels = spec.channels as usize;
    let chunk_size = (sample_rate * chunk_duration_sec) as usize * channels;

    // Read the samples and find the total number of samples
    let samples: Vec<i32> = reader.samples::<i32>().map(|s| s.unwrap()).collect();
    let total_samples = samples.len();

    // Calculate the number of chunks
    let num_chunks = (total_samples + chunk_size - 1) / chunk_size;

    let base_filename = path.file_stem().unwrap().to_str().unwrap();

    // Calculate the number of digits needed for padding
    let padding_width = format!("{}", num_chunks - 1).len();

    for i in 0..num_chunks {
        let start = i * chunk_size;
        let end = usize::min(start + chunk_size, total_samples);
        let mut chunk_samples: Vec<i32> = samples[start..end].to_vec();

        // Pad with zeros if the chunk is not full
        if chunk_samples.len() < chunk_size {
            chunk_samples.extend(vec![0; chunk_size - chunk_samples.len()]);
        }

        let output_filename = format!("{}@{:0width$}.wav", base_filename, i, width = padding_width);
        let output_path = output_directory_path.join(output_filename);

        let spec = WavSpec {
            channels: spec.channels,
            sample_rate: spec.sample_rate,
            bits_per_sample: spec.bits_per_sample,
            sample_format: SampleFormat::Int,
        };

        let mut writer = WavWriter::create(&output_path, spec).expect("Failed to create WAV writer");

        for &sample in &chunk_samples {
            writer.write_sample(sample).unwrap();
        }
    }
}
