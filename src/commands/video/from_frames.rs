use anyhow::{Context, Result};
use gstreamer as gst;
use gstreamer::prelude::*;
use regex::Regex;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use tempfile;

use crate::utils::{file_has_right_extension, perform_io_sanity_check};

use crate::VideoFromFramesArgs;

// Admissible extensions for this command
const EXTENSIONS: [&str; 3] = ["png", "jpg", "jpeg"];

pub fn execute(args: VideoFromFramesArgs) -> Result<()> {
    // Parse the arguments
    let input = Path::new(&args.input);
    let output = Path::new(&args.output);
    let fps = &args.fps;

    // Perform sanity check on I/O
    perform_io_sanity_check(input, output, true, true).with_context(|| "Sanity check failed")?;

    // Check if input is already sanitised
    let input_frames_are_ok = check_frames_obey_rule(input, r"frame\d+\.png");

    // Create a new input path to be used
    let input_path = if input_frames_are_ok.is_err() {
        create_sanitised_directory(input)?
    } else {
        input
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Failed to convert input path to string"))?
            .to_string()
    };

    // Process files
    process_file(&input_path, fps, output).with_context(|| "Processing failed")?;

    // If we created a temporary directory, clean it up
    if input_frames_are_ok.is_err() {
        fs::remove_dir_all(&input_path)
            .with_context(|| format!("Failed to clean up temporary directory {}", input_path))?;
    }

    Ok(())
}

// Function to check if frames have the correct structure
fn check_frames_obey_rule(input: &Path, rule: &str) -> Result<bool> {
    // Create regex pattern
    let pattern = Regex::new(&format!(r"^{}", input.join(rule).to_string_lossy()))
        .with_context(|| "Failed to format regex")?;

    // Find all images in a directory
    let invalid_frames: Vec<String> = fs::read_dir(input)
        .with_context(|| "Failed to read directory")?
        .filter_map(|e| e.ok())
        .filter(|e| file_has_right_extension(&e.path(), &EXTENSIONS).is_ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|e| !pattern.is_match(e))
        .collect();

    // Return to if everything is ok and all the frames respect the regex
    Ok(invalid_frames.is_empty())
}

// Create temporary directory. Images in input are read, sorted, and copied in the
// temp directory following the frame%d.png pattern
fn create_sanitised_directory(input: &Path) -> Result<String> {
    // Create temporary directory
    let temp_dir = tempfile::tempdir().with_context(|| "Failed to create temporary directory")?;

    // Find all valid image files and sort them
    let mut image_files: Vec<PathBuf> = fs::read_dir(input)
        .with_context(|| "Failed to read directory")?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| file_has_right_extension(p, &EXTENSIONS).is_ok())
        .collect();

    // Sort images
    image_files.sort();

    // Copy files to temp directory with sanitized names
    for (i, src_path) in image_files.iter().enumerate() {
        let dest_path = temp_dir.path().join(format!("frame{}.png", i));
        fs::copy(src_path, &dest_path).with_context(|| {
            format!(
                "Failed to copy {} to {}",
                src_path.display(),
                dest_path.display()
            )
        })?;
    }

    // Get the path as a string
    let temp_path = temp_dir
        .path()
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Failed to convert temp directory path to string"))?
        .to_string();

    // Keep temp_dir alive by moving it into a variable that lives until the end of the function
    std::mem::forget(temp_dir); // Prevent the directory from being deleted when temp_dir is dropped

    Ok(temp_path)
}

// Function for getting relevant info of an image file by just probing it
fn process_file(input: &String, fps: &i32, output: &Path) -> Result<()> {
    // The pipeline converting frames to video would be of the form
    //
    //      gst-launch-1.0 \
    //      multifilesrc location="frame%d.png" index=0 caps="image/png,framerate=25/1" ! \
    //      pngdec ! \
    //      videoconvert ! \
    //      x264enc bitrate=2000 speed-preset=medium ! \
    //      mp4mux ! \
    //      filesink location=output.mp4
    //
    // Our goal is to replicate that pipeline using Rust bindings

    // Initialize GStreamer
    gst::init().with_context(|| "Failed to init GStreamer".to_string())?;

    // Create pipeline
    let pipeline = gst::Pipeline::new();

    // Create elements
    // multifilesrc: Reads multiple files as a continuous stream, useful for reading frame sequences
    let multifilesrc = gst::ElementFactory::make_with_name("multifilesrc", Some("file-source"))
        .with_context(|| "Failed to create multifilesrc element".to_string())?;
    multifilesrc.set_property("location", format!("{}/frame%d.png", input));
    multifilesrc.set_property("index", 0i32);
    let caps = gst::Caps::builder("image/png")
        .field("framerate", gst::Fraction::new(*fps, 1))
        .build();
    multifilesrc.set_property("caps", &caps);

    // pngdec: Decodes PNG images into raw video frames
    let pngdec = gst::ElementFactory::make_with_name("pngdec", Some("pngdec"))
        .with_context(|| "Failed to create pngdec element".to_string())?;

    // videoconvert: Converts video frames between different color spaces and formats
    let videoconvert = gst::ElementFactory::make_with_name("videoconvert", Some("videoconvert"))
        .with_context(|| "Failed to create videoconvert element".to_string())?;

    // x264enc: H.264 video encoder that compresses raw video frames
    let x264enc = gst::ElementFactory::make_with_name("x264enc", Some("x264enc"))
        .with_context(|| "Failed to create x264enc element".to_string())?;
    x264enc.set_property("bitrate", 2000u32); // Sets video bitrate to 2000 kbps
    x264enc.set_property_from_str("speed-preset", "medium");

    // mp4mux: Multiplexes encoded video into an MP4 container format
    let mp4mux = gst::ElementFactory::make_with_name("mp4mux", Some("mp4mux"))
        .with_context(|| "Failed to create mp4mux element".to_string())?;

    // filesink: Writes the final MP4 stream to a file
    let filesink = gst::ElementFactory::make_with_name("filesink", Some("filesink"))
        .with_context(|| "Failed to create filesink element".to_string())?;
    filesink.set_property("location", output.to_str().unwrap());

    // Add all elements to the pipeline
    pipeline
        .add_many([
            &multifilesrc,
            &pngdec,
            &videoconvert,
            &x264enc,
            &mp4mux,
            &filesink,
        ])
        .with_context(|| "Failed adding elements to GStreamer pipeline".to_string())?;

    // Link all elements in the pipeline
    gst::Element::link_many([
        &multifilesrc,
        &pngdec,
        &videoconvert,
        &x264enc,
        &mp4mux,
        &filesink,
    ])
    .with_context(|| "Failed to link elements in pipeline".to_string())?;

    // Start playing the pipeline
    pipeline
        .set_state(gst::State::Playing)
        .with_context(|| "Failed playing pipeline".to_string())?;

    // Listen for messages on the pipeline's bus
    let bus = pipeline
        .bus()
        .with_context(|| "Failed creating pipeline bus".to_string())?;
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        match msg.view() {
            gst::MessageView::Eos(..) => {
                // End-of-Stream message: all data has been processed
                break;
            }
            gst::MessageView::Error(err) => {
                // Error message: something went wrong in the pipeline
                eprintln!(
                    "Error received from {:?}: {:?} ({:?})",
                    msg.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            _ => {
                // Other messages (state changes, etc.) are ignored
            }
        }
    }

    // Shut down the pipeline
    pipeline
        .set_state(gst::State::Null)
        .with_context(|| "Failed to shut down pipeline".to_string())?;

    Ok(())
}
