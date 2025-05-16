use anyhow::Error;
use anyhow::{Context, Result};
use gstreamer as gst;
use gstreamer::prelude::*;
use std::path::Path;

use crate::utils::{file_has_right_extension, perform_io_sanity_check};

use crate::VideoThumbnailArgs;

// Admissible extensions for this command
const EXTENSIONS: [&str; 5] = ["ts", "mp4", "mkv", "mov", "webm"];

pub fn execute(args: VideoThumbnailArgs) -> Result<()> {
    // Parse the arguments
    let input = Path::new(&args.input);
    let output = Path::new(&args.output);

    // Perform sanity check on I/O
    perform_io_sanity_check(input, output, false, false).with_context(|| "Sanity check failed")?;

    // Check extension
    file_has_right_extension(input, &EXTENSIONS)
        .with_context(|| "File extension is not admissible")?;

    // Process files
    process_file(input, output).with_context(|| "Processing failed")?;

    Ok(())
}

// Function for getting relevant info of an image file by just probing it
fn process_file(input: &Path, output: &Path) -> Result<()> {
    // The pipeline converting a video to frames would be of the form
    //
    //      gst-launch-1.0 \
    //      filesrc location=<PATH_TO_VIDEO> ! \
    //      decodebin ! \
    //      videoconvert ! \
    //      jpegenc snapshot=TRUE ! \
    //      filesink location="<VIDEO_STEM>.jpeg"
    //
    // Our goal is to replicate that pipeline using Rust bindings

    // GStreamer must be initialized.
    // This command initializes all internal structures and loads available plugins.
    gst::init().with_context(|| "Failed to init GStreamer".to_string())?;

    // A pipeline is a top-level container that manages the data flow and
    // synchronization of its contained elements
    let pipeline = gst::Pipeline::new();

    // filesrc: Reads data from a file
    let filesrc = gst::ElementFactory::make_with_name("filesrc", Some("file-source"))
        .with_context(|| "Failed to create filesrc element".to_string())?;
    filesrc.set_property("location", input.to_str());

    // decodebin: Auto-detects the type of encoded stream and decodes it
    let decodebin = gst::ElementFactory::make_with_name("decodebin", Some("decodebin"))
        .with_context(|| "Failed to create decodebin element".to_string())?;
    // videoconvert: Converts video frames between different formats
    let videoconvert = gst::ElementFactory::make_with_name("videoconvert", Some("videoconvert"))
        .with_context(|| "Failed to create videoconvert".to_string())?;
    // jpegenc: Encodes raw video frames into PNG format
    let jpegenc = gst::ElementFactory::make_with_name("jpegenc", Some("jpegenc"))
        .with_context(|| "Failed to create jpegenc".to_string())?;
    // Set property snapshot to true so that we extract a single frame
    jpegenc.set_property("snapshot", &(true));
    // filesink: Saves buffers to a series of sequentially-named files
    let stem = input
        .file_stem()
        .and_then(|s| s.to_str())
        .with_context(|| "Failed to get input filename")?;
    let filesink = gst::ElementFactory::make_with_name("filesink", Some("filesink"))
        .with_context(|| "Failed to create filesink".to_string())?;
    filesink.set_property(
        "location",
        output.join(format!("{}.jpeg", stem)).to_str(),
    );

    // Add all elements to the pipeline
    // Elements must be added to the pipeline before they can be used
    // This is the replica of the CLI pipeline we defined above
    pipeline
        .add_many([
            &filesrc,
            &decodebin,
            &videoconvert,
            &jpegenc,
            &filesink,
        ])
        .with_context(|| "Failed adding elements to GStreamer pipeline".to_string())?;

    // Link what we can statically:
    // Elements are linked in the order that data should flow through them.
    // Some elements (like decodebin) create their output pads dynamically,
    // so we can only link what we know about at this point.

    // Link filesrc → decodebin
    gst::Element::link_many([&filesrc, &decodebin])
        .with_context(|| "Failed to link filesrc and decodebin".to_string())?;

    // Link videoconvert → jpegenc → filesink
    gst::Element::link_many([&videoconvert, &jpegenc, &filesink])
        .with_context(|| "Failed to link videoconvert, jpegenc and filesink".to_string())?;

    // Connect decodebin's "pad-added" signal
    // Since decodebin creates its output pads dynamically (only after it has detected
    // the type of the input stream), we need to wait for it to create its pad and then
    // link it to the next element (videoconvert) in the pipeline
    decodebin.connect_pad_added(move |dbin, src_pad| {
        // First, we need to get access to the pipeline from decodebin
        let pipeline = match dbin
            .parent()
            .and_then(|parent| parent.dynamic_cast::<gst::Pipeline>().ok())
        {
            Some(pipeline) => pipeline,
            None => {
                eprintln!(
                    "{}",
                    Error::msg("Failed to get Pipeline from decodebin's parent")
                );
                return;
            }
        };

        // Find the videoconvert element in our pipeline using its name
        let convert = match pipeline.by_name("videoconvert") {
            Some(elem) => elem,
            None => {
                eprintln!("{}", Error::msg("Failed to find videoconvert in pipeline"));
                return;
            }
        };

        // Get the sink pad of videoconvert that we'll link to
        let sink_pad = match convert.static_pad("sink") {
            Some(pad) => pad,
            None => {
                eprintln!("{}", Error::msg("Failed to get sink pad from videoconvert"));
                return;
            }
        };
        // Check if the pad is already linked - we only want to link once
        if sink_pad.is_linked() {
            return;
        }

        // Get the capabilities of the new pad to check if it's video
        let new_pad_caps = match src_pad.current_caps() {
            Some(caps) => caps,
            None => return,
        };
        let new_pad_struct = new_pad_caps.structure(0).unwrap();
        let new_pad_type = new_pad_struct.name();

        // Only link if this is a video pad (ignore audio or other types)
        if new_pad_type.starts_with("video/") {
            let link_ok = src_pad.link(&sink_pad);
            if let Err(err) = link_ok {
                eprintln!(
                    "Failed to link decodebin src pad to videoconvert sink pad: {:?}",
                    err
                );
            }
        }
    });

    // Start playing the pipeline
    // Set the pipeline state to Playing, which starts the data flow
    pipeline
        .set_state(gst::State::Playing)
        .with_context(|| "Failed playing pipeline".to_string())?;

    // Listen for messages on the pipeline's bus
    // The bus carries messages from the pipeline elements about various events
    let bus = pipeline
        .bus()
        .with_context(|| "Failed creating pipeline bus".to_string())?;
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        match msg.view() {
            gst::MessageView::Eos(..) => {
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
    // Set the state to Null, which stops everything and frees resources
    pipeline
        .set_state(gst::State::Null)
        .with_context(|| "Failed to shut down pipeline".to_string())?;

    Ok(())
}
