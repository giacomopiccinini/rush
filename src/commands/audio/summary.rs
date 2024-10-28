use lofty::{AudioFile, Probe};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use rayon::prelude::*;
use anyhow::{Context, Result};

use crate::utils::{file_has_right_extension};
use crate::AudioSummaryArgs;

// Admissible extensions for this command
const EXTENSIONS : [&str; 6] = ["mp3", "wav", "ogg", "flac", "aac", "m4a"];


pub fn execute(args: AudioSummaryArgs) -> Result<()> {

    // Parse the arguments
    let target = Path::new(&args.target);

    // Error if it does not exist at all
    if !target.exists() {
        return Err(anyhow::Error::msg("Target file or directory does not exist"));
    }

    // Find all admissible files
    let files: Vec<PathBuf> = match target.is_file() {
        true => {
            if file_has_right_extension(target, &EXTENSIONS).is_ok() {
                vec![target.to_path_buf()]
            } else {
                vec![]
            }
        },
        false => {
            WalkDir::new(target)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| file_has_right_extension(e.path(), &EXTENSIONS).is_ok())
            .map(|e| e.path().to_path_buf())
            .collect()
        },
    };

    // Raise error if no files are admissible
    if files.is_empty(){
        return Err(anyhow::Error::msg("No admissible audio files detected"));
    }

    // Process files
    let info: Vec<(u128, u32, u8, u8)> = files.into_par_iter()
        .filter_map(|file| process_audio(&file).ok())
        .collect();

    // Calculate total number of files
    let n_files = info.len();

    // Compute duration in seconds
    let total_duration_seconds: u64 = info.iter()
        .map(|(duration, _, _, _)| (duration / 1_000_000_000) as u64)
        .sum();

    // Get unique values
    let unique_sample_rates: HashSet<_> = info.iter().map(|(_, sr, _, _)| sr).collect();
    let unique_channels: HashSet<_> = info.iter().map(|(_, _, channels, _)| channels).collect();
    let unique_bit_depths: HashSet<_> = info.iter().map(|(_, _, _, depth)| depth).collect();
    let unique_durations: HashSet<_> = info.iter().map(|(duration, _, _, _)| duration).collect();

    // Format duration
    let hours = total_duration_seconds / 3600;
    let remainder = total_duration_seconds % 3600;
    let minutes = remainder / 60;
    let seconds = remainder % 60;

    // Print results
    println!("Total files: {}", n_files);
    println!("Total Duration: {:02}:{:02}:{:02}", hours, minutes, seconds);
    println!("Average Duration: {} s", total_duration_seconds / n_files as u64);
    println!("Sample Rates: {:?} Hz", unique_sample_rates);
    println!("Channels: {:?}", unique_channels);
    println!("Bit Depths: {:?}", unique_bit_depths);
    println!("Unique durations: {}", unique_durations.len());

    if let (Some(min), Some(max)) = (unique_durations.iter().min(), unique_durations.iter().max()) {
        println!("Min duration: {:} s", (**min as f64 / 1_000_000_000_f64));
        println!("Max duration: {:} s", (**max as f64 / 1_000_000_000_f64));
    }


    Ok(())
}

// Function for getting relevant info of an audio file by just probing it
fn process_audio(file: &Path) -> Result<(u128, u32, u8, u8)>  {

    // Probe the audio file
    let audio_file = Probe::open(file)
        .with_context(|| format!("Failed to open audio file: {:?}", file))?
        .read()
        .with_context(|| format!("Failed to read audio metadata from: {:?}", file))?;

    // Read all audio file properties
    let properties = audio_file.properties();

    // Get the number of channels
    let channels = properties.channels().with_context(|| "Failed to read channels")?;

    // Get the duration in seconds
    let duration = properties.duration().as_nanos();

    // Get the sample rate
    let sample_rate = properties.sample_rate().with_context(|| "Failed to read sample rate")?;

    // Get the bit depth
    let bit_depth = properties.bit_depth().with_context(|| "Failed to read bit depth")?;

    Ok((duration, sample_rate, channels, bit_depth))
}