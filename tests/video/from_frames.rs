use crate::utils::{cleanup_test_dir, create_test_image, setup_test_dir};
use anyhow::Result;
use rush::commands::video;
use rush::VideoFromFramesArgs;

#[test]
fn test_video_from_frames_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test frames
    for i in 0..30 {
        let frame_path = test_dir.join(format!("frame_{:04}.png", i));
        create_test_image(&frame_path, 1920, 1080, 3)?;
    }

    // Define output path
    let output_path = test_dir.join("output.mp4");

    // Define args
    let args = VideoFromFramesArgs {
        input: test_dir.to_string_lossy().to_string(),
        output: output_path.to_string_lossy().to_string(),
        fps: 30,
    };

    // Execute command
    video::from_frames::execute(args)?;

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_video_from_frames_nonexistent_path() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;
    let nonexistent_path = test_dir.join("nonexistent");

    // Define output path
    let output_path = test_dir.join("output.mp4");

    // Define args with nonexistent path
    let args = VideoFromFramesArgs {
        input: nonexistent_path.to_string_lossy().to_string(),
        output: output_path.to_string_lossy().to_string(),
        fps: 30,
    };

    // Execute command and expect error
    let result = video::from_frames::execute(args);
    assert!(result.is_err());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}