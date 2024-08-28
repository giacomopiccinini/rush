use lofty::{AudioFile, Probe};
use std::collections::HashSet;
use std::path::PathBuf;
use walkdir::WalkDir;
use rayon::prelude::*;

use crate::AudiosumArgs;

// Check if the file is an audio file
fn is_audio_file(path: &PathBuf, extensions: &HashSet<String>) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| extensions.contains(&ext.to_lowercase()))
        .unwrap_or(false)
}

// Process the entire directory, return a list composed of tuples of duration and sample rate
fn process_directory(target: &PathBuf, extensions: &HashSet<String>) -> (usize, Vec<(u64, u32)>) {
    let duration_sample_rate: Vec<_> = WalkDir::new(target)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| !entry.file_type().is_dir())
        .filter(|entry| is_audio_file(&entry.path().to_path_buf(), extensions))
        .par_bridge()
        .filter_map(|entry| process_audio(entry.path()))
        .collect();

    let counter = duration_sample_rate.len();
    (counter, duration_sample_rate)
}

// Equivalent of process directory in case of a single audio file
fn process_single_audio(path: &PathBuf) -> (usize, Vec<(u64, u32)>) {
    process_audio(path)
        .map(|info| (1, [info].into_iter().collect()))
        .unwrap_or((0, Vec::new()))
}

// Function for getting relevant info (duration and sample rate) of an audio file
fn process_audio(path: &std::path::Path) -> Option<(u64, u32)>  {

    let audio_file = Probe::open(path)
    .and_then(|probe| probe.read())
    .ok()?;

    let properties = audio_file.properties();
    let duration = properties.duration().as_secs();
    let sample_rate = properties.sample_rate()?;

    Some((duration, sample_rate))
}

// Compute and print the relevant info 
fn print_audio_summary(n_files: usize, audio_info: Vec<(u64, u32)>) {

    // Compute duration in seconds
    let total_duration_seconds: u64 = audio_info.iter().map(|(duration, _)| duration).sum();

    // Compute all durations
    let unique_durations: HashSet<&u64>= audio_info.iter().map(|(duration, _)| duration).collect();

    // Find all sample rates
    let unique_sample_rates: HashSet<&u32> = audio_info.iter().map(|(_, sample_rate)| sample_rate).collect();

    // Convert duration to hours, minutes and seconds
    let hours = total_duration_seconds / 3600;
    let remainder = total_duration_seconds % 3600;
    let minutes = remainder / 60;
    let seconds = remainder % 60;

    // Print results
    println!("Total files: {}", n_files);
    println!("Total Duration: {:02}:{:02}:{:02}", hours, minutes, seconds);
    println!("Average Duration: {} s", total_duration_seconds / n_files as u64);
    println!("Sample Rates: {:?} Hz", unique_sample_rates);
    println!("Unique durations: {}", unique_durations.len());
    println!("Min duration: {} s", *unique_durations.iter().min().unwrap());
    println!("Max duration: {} s", *unique_durations.iter().max().unwrap());
    println!("Unique sample rates: {:?}", unique_sample_rates);
}

pub fn execute(args: AudiosumArgs) {

    let target = PathBuf::from(&args.target);
    let audio_extensions: HashSet<_> = ["mp3", "wav", "ogg", "flac", "aac", "m4a"]
        .iter().map(|&s| s.to_lowercase()).collect();

    if !target.is_dir() && !is_audio_file(&target, &audio_extensions) {
        println!("Target is neither a directory nor an audio file.");
        return;
    }

    let (n_files, audio_info) = if target.is_dir() {
        process_directory(&target, &audio_extensions)
    } else {
        process_single_audio(&target)
    };

    print_audio_summary(n_files, audio_info);

}