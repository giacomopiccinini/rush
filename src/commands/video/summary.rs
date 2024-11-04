extern crate ffmpeg_next as ffmpeg;

use anyhow::{Context, Result};
use rayon::prelude::*;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::utils::file_has_right_extension;

use crate::VideoSummaryArgs;

// Admissible extensions for this command
const EXTENSIONS: [&str; 4] = ["ts", "mp4", "mkv", "mov"];

pub fn execute(args: VideoSummaryArgs) -> Result<()> {
    // Parse the arguments
    let target = Path::new(&args.target);

    // Error if it does not exist at all
    if !target.exists() {
        return Err(anyhow::Error::msg(
            "Target file or directory does not exist",
        ));
    }

    // Find all admissible files
    let files: Vec<PathBuf> = match target.is_file() {
        true => {
            if file_has_right_extension(target, &EXTENSIONS).is_ok() {
                vec![target.to_path_buf()]
            } else {
                vec![]
            }
        }
        false => WalkDir::new(target)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| file_has_right_extension(e.path(), &EXTENSIONS).is_ok())
            .map(|e| e.path().to_path_buf())
            .collect(),
    };

    // Raise error if no files are admissible
    if files.is_empty() {
        return Err(anyhow::Error::msg("No admissible image files detected"));
    }

    // Process files
    let info: Vec<(f64, u32, u32, u32, u32)> = files
        .into_par_iter()
        .filter_map(|file| process_video(&file).ok())
        .collect();

    // Calculate total number of files
    let n_files = info.len();

    // Sum all durations
    let total_duration: f64 = info.iter().map(|(duration, _, _, _, _)| duration).sum();

    // Get unique values
    let unique_durations: HashSet<_> = info
        .iter()
        .map(|(duration, _, _, _, _)| *duration as u64)
        .collect();
    let unique_fps: HashSet<_> = info
        .iter()
        .map(|(_, fps_num, fps_den, _, _)| (fps_num, fps_den))
        .collect();
    let unique_shapes: HashSet<_> = info.iter().map(|(_, _, _, h, w)| (h, w)).collect();

    // Print results
    println!("Total files: {}", n_files);
    println!("Total duration: {}", total_duration);
    println!("Unique durations: {:?}", unique_durations);
    println!("Unique (height, width) pairs: {:?}", unique_shapes);
    println!("Unique  FPS: {:?}", unique_fps);

    Ok(())
}

// Function for getting relevant info of an image file by just probing it
fn process_video(path: &Path) -> Result<(f64, u32, u32, u32, u32)> {
    // Read context
    let context = ffmpeg::format::input(&path).with_context(|| "Couldn't read video")?;

    // Only select the video stream, throw away audio, subtitles etc
    if let Some(video_stream) = context
        .streams()
        .find(|s| s.parameters().medium() == ffmpeg::media::Type::Video)
    {
        // Extract duration
        let duration = video_stream.duration() as f64 * f64::from(video_stream.time_base());

        // Extract FPS
        let fps_numerator = video_stream.rate().numerator() as u32;
        let fps_denominator = video_stream.rate().denominator() as u32;

        // Create decoder
        let context_decoder = ffmpeg::codec::context::Context::from_parameters(video_stream.parameters()).with_context(|| "Failed to create decoder context from video stream parameters")?;
        let decoder = context_decoder.decoder().video().with_context(|| "Failed to create video decoder from decoder context")?;

        // Extract width and height from codec parameters directly
        let width = decoder.width();
        let height = decoder.height();

        Ok((duration, fps_numerator, fps_denominator, height, width))
    } else {
        Err(anyhow::Error::msg("No video stream found in file"))
    }
}
