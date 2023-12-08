use lofty::{AudioFile, Probe};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use crate::AudiosumArgs;

pub fn execute(args: AudiosumArgs) {
    // Init file counter
    let mut n_files: i32 = 0;

    // Convert to path object
    let path = PathBuf::from(&args.target);
    let verbose = &args.verbose;

    // Initialize a counter for the total duration
    let mut total_duration: u64 = 0;

    // Hash set for unique sample rates
    let mut unique_sample_rates: HashSet<u32> = HashSet::new();

    // Check if the given path is a file
    if path.is_file() {
        // Get the audio info for that single file
        let (duration, sample_rate) = audio_info(&path);

        // Add the duration of the file to the total duration
        total_duration += duration;

        // Add the sample rate
        unique_sample_rates.insert(sample_rate);

        // Increase file number
        n_files += 1;

        // If verbose, print the audio info for each audio file
        if *verbose {
            // Define output string
            let output = format!(
                "File: {} | Duration: {} s| Sample Rate: {} Hz",
                path.display(),
                duration,
                sample_rate
            );

            // Print the output
            println!("{}", output);
        }

    // If the given path is a directory
    } else if path.is_dir() {
        // Loop over all files in the directory
        for audio in fs::read_dir(path).expect("Unable to read directory") {
            // Get path to audio file
            let audio_path = audio.expect("Failed to read entry").path();

            // If the path is a file
            if audio_path.is_file() {
                // If the extension is not mp3 or wav, skip the file
                // Use this to avoid that file with no extension (e.g. .gitkeep) make
                // the script panic
                if let Some(ext) = audio_path.extension() {
                    if ext != "mp3" && ext != "wav" {
                        continue;
                    }
                } else {
                    continue; // Skip files with no extension
                }

                // Get duration and sample rate
                let (duration, sample_rate) = audio_info(&audio_path);

                // Add the duration of the file to the total duration
                total_duration += duration;

                // Add the sample rate
                unique_sample_rates.insert(sample_rate);

                // Increase file number
                n_files += 1;

                // If verbose, print the audio info for each audio file
                if *verbose {
                    // Define output string
                    let output = format!(
                        "File: {} | Duration: {} s| Sample Rate: {} Hz",
                        audio_path.display(),
                        duration,
                        sample_rate
                    );

                    // Print the output
                    println!("{}", output);
                }
            }
        }
    } else {
        panic!("ERROR: Invalid path!");
    }

    // Get the hours, minutes and seconds from the total duration
    let hours = total_duration / 3600;
    let remainder = total_duration % 3600;
    let minutes = remainder / 60;
    let seconds = remainder % 60;

    // Always print the total duration and sample rates
    println!("Total Files: {}", n_files);
    println!("Total Duration: {:02}:{:02}:{:02}", hours, minutes, seconds);
    println!("Sample Rates: {:?} Hz", unique_sample_rates);
}

// Function for getting relevant info of an audio file
fn audio_info(audio_path: &PathBuf) -> (u64, u32) {
    // Try to open the file for probing. If it fails, throw an error.
    // Then try to read the file. If reading fails, throw another error.
    let tagged_file = Probe::open(audio_path)
        .expect("ERROR: Bad path provided!")
        .read()
        .expect("ERROR: Failed to read file!");

    // Retrieve audio properties of the tagged file.
    let properties = tagged_file.properties();

    // Calculate the duration of the audio in minutes and seconds.
    let duration = properties.duration().as_secs();

    // Get the sample rate
    let sample_rate = properties.sample_rate().unwrap_or(0);

    return (duration, sample_rate);
}
