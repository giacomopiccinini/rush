extern crate ffmpeg_next as ffmpeg;

use std::collections::HashSet;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::VideoSummaryArgs;

pub fn execute(args: VideoSummaryArgs) {
    // Convert to path object
    let target = PathBuf::from(&args.target);

    // Init FFMPEG
    ffmpeg::init().unwrap();

    if target.is_dir() {
        // Init the image counter
        let mut counter = 0;

        // Init the total length counter
        let mut total_duration = 0.0;

        // Init the set of unique dimensions
        let mut dimensions: HashSet<(u32, u32)> = HashSet::new();

        // Init the hash set of unique lengths
        let mut lengths: HashSet<u64> = HashSet::new();

        // Init the set of fps
        let mut unique_fps: HashSet<(u32, u32)> = HashSet::new();

        // Iterate over the files in the directory
        //for path in paths {
        for path in WalkDir::new(&target).into_iter().filter_map(|e| e.ok()) {
            // Get the path
            let path = path.path();

            if path.is_dir() {
                continue;
            }

            // If the path has an extension
            if let Some(ext) = path.extension() {
                // If the extension is an image extension
                if ["ts", "mp4", "mkv"].contains(&ext.to_str().unwrap()) {
                    // Increment the image counter
                    counter += 1;

                    match ffmpeg::format::input(&path) {
                        Ok(context) => {
                            for stream in context.streams() {
                                // Get the duration
                                let duration =
                                    stream.duration() as f64 * f64::from(stream.time_base());

                                // Add the duration to the total duration
                                total_duration += duration;

                                // Add the duration to the set of unique lengths
                                lengths.insert(duration as u64);

                                // Get the fps in a rational form
                                let fps_rational = stream.rate();

                                // Extract the numerator and denominator
                                let fps_numerator = fps_rational.numerator() as u32;
                                let fps_denominator = fps_rational.denominator() as u32;

                                // Add the fps to the set of unique fps
                                unique_fps.insert((fps_numerator, fps_denominator));

                                // Get the codec
                                let codec = stream.codec();

                                // If the codec is a video codec
                                if codec.medium() == ffmpeg::media::Type::Video {
                                    if let Ok(video) = codec.decoder().video() {
                                        // Get the height and width
                                        dimensions.insert((video.height(), video.width()));
                                    }
                                }
                            }
                        }
                        Err(error) => println!("error: {}", error),
                    }
                }
            }
        }

        // Get the hours, minutes and seconds from the total duration
        let hours = total_duration as u64 / 3600;
        let remainder = total_duration as u64 % 3600;
        let minutes = remainder / 60;
        let seconds = remainder % 60;

        // Get info about the lengths
        let len_count = lengths.len();
        let is_unique = len_count == 1;
        let len_min = lengths.iter().min().unwrap();
        let len_max = lengths.iter().max().unwrap();

        // Print the results
        println!("Total videos: {}", counter);
        println!("Total Duration: {:02}:{:02}:{:02}", hours, minutes, seconds);
        println!("Unique FPS: {:?}", unique_fps);
        println!("Unique Dimensions: {:?}", dimensions);
        println!("Equal Length: {}", is_unique);
        println!("Unique Lengths: {}", len_count);
        println!("Min Length: {} s", len_min);
        println!("Max Length: {} s", len_max);
    } else if ["ts", "mp4", "mkv"].contains(
        &target
            .extension()
            .expect("Error with extension")
            .to_str()
            .unwrap(),
    ) {
        match ffmpeg::format::input(&target) {
            Ok(context) => {
                for stream in context.streams() {
                    // Get the duration
                    let duration = stream.duration() as f64 * f64::from(stream.time_base());

                    // Get the hours, minutes and seconds from the total duration
                    let hours = duration as u64 / 3600;
                    let remainder = duration as u64 % 3600;
                    let minutes = remainder / 60;
                    let seconds = remainder % 60;

                    // Get the fps in a rational form
                    let fps_rational = stream.rate();

                    // Extract the numerator and denominator
                    let fps_numerator = fps_rational.numerator() as u32;
                    let fps_denominator = fps_rational.denominator() as u32;

                    // Get the codec
                    let codec = stream.codec();

                    println!("Total videos: {}", 1);
                    println!("Total Duration: {:02}:{:02}:{:02}", hours, minutes, seconds);
                    println!("FPS: {}/{}", fps_numerator, fps_denominator);

                    // If the codec is a video codec
                    if codec.medium() == ffmpeg::media::Type::Video {
                        if let Ok(video) = codec.decoder().video() {
                            let height = video.height();
                            let width = video.width();

                            println!("Dimensions: {}x{}", height, width);
                        }
                    }
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
