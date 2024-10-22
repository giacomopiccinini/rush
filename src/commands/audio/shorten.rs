use std::fs;
use std::path::Path;
use hound::{WavReader, WavWriter, WavSpec, SampleFormat};
use rayon::prelude::*;

use crate::AudioShortenArgs;


pub fn execute(args: AudioShortenArgs) {

    // Parse the arguments
    let input = Path::new(&args.input);
    let length: f32 = args.length;
    let output = Path::new(&args.output);
    let replace_original: bool = args.replace_original;

    // Sanity checks on args
    if input.is_dir() && output.is_file(){
        eprintln!("Input is a directory but output is a file");
        return
    }

    if output.is_dir() && !output.exists(){
        fs::create_dir_all(output).expect("Failed to create output directory");
    }

    // Process files
    process(input, length, output, replace_original);
}

fn process(input: &Path, length: f32, output: &Path, replace_original: bool) {

    // Case of single input file
    if input.is_file() {
        if input.extension().and_then(|ext| ext.to_str()).map(|s| s.to_lowercase()) == Some("wav".to_string()) {
            process_file(input, length, output, replace_original);
        }
    } 
    // Case of input being a directory
    else if input.is_dir() {

        // Find all entries (files or nested directories)
        let entries: Vec<_> = fs::read_dir(input)
            .expect("Failed to read directory")
            .collect::<Result<Vec<_>, _>>()
            .expect("Failed to collect directory entries");

        // Parallel loop over entries
        entries.par_iter().for_each(|entry| {

            // Path to the entry
            let entry_path = entry.path();

            // Relative path wrt input directory
            let relative_path = entry_path.strip_prefix(input).unwrap();

            // Nested output path
            let new_output= output.join(relative_path);

            // If the entry is again a directory 
            if entry_path.is_dir() {

                // Create the expected target nested output
                fs::create_dir_all(&new_output).expect("Failed to create output subdirectory");

                // Re-apply the same function recursively
                process(&entry_path, length, &new_output, replace_original);

            } 
            // If it's a single file and it's a wav file
            else if entry_path.extension().and_then(|ext| ext.to_str()).map(|s| s.to_lowercase()) == Some("wav".to_string()) {

                // Process the file
                process_file(&entry_path, length, &new_output.parent().unwrap(), replace_original);
            }
        });
    } else {
        eprintln!("The provided path is neither a file nor a directory");
    }
}

fn process_file(input: &Path, length: f32, output: &Path, replace_original: bool) {

    // Open the WAV file
    let mut reader = WavReader::open(&input).expect("Failed to open WAV file");

    // Extract info from file
    let spec = reader.spec();
    let sample_rate = spec.sample_rate as f32;
    let channels = spec.channels as usize;

    // Compute the requested length in samples
    let length_samples = (sample_rate * length) as usize * channels;

    // Read the samples and find the total number of samples
    let samples: Vec<i32> = reader.samples::<i32>().map(|s| s.unwrap()).collect();
    let total_samples = samples.len();

    // Raise error if not enough samples
    if length_samples > total_samples{
        eprintln!("Requested length larger than file length");
        return
    }
    else{
        // Shorten the audio
        let shortened_samples : Vec<i32> = samples[0..length_samples].to_vec();

        // Define the output
        let base_filename = input.file_name().unwrap().to_str().unwrap();
        let output_path = output.join(base_filename);

        if output_path == input && replace_original==false{
            eprintln!("Not allowed to overwrite");
            return
        }

        // Define file specs
        let spec = WavSpec {
            channels: spec.channels,
            sample_rate: spec.sample_rate,
            bits_per_sample: spec.bits_per_sample,
            sample_format: SampleFormat::Int,
        };

        // Init writer
        let mut writer = WavWriter::create(&output_path, spec).expect("Failed to create WAV writer");

        for &sample in &shortened_samples {
            writer.write_sample(sample).unwrap();
        }
    }
}
