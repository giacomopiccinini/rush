use crate::utils::{cleanup_test_dir, create_test_video, setup_test_dir};
use anyhow::Result;
use rush::commands::video;
use rush::VideoToFramesArgs;
use std::fs;

#[test]
fn test_video_to_frames_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test file
    let input_path = test_dir.join("input.mp4");
    create_test_video(&input_path, 1920, 1080, 5.0, 30)?;

    // Define args
    let args = VideoToFramesArgs {
        input: input_path.to_string_lossy().to_string(),
        output: test_dir.to_string_lossy().to_string(),
    };

    // Execute command
    video::to_frames::execute(args)?;

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_video_to_frames_nonexistent_path() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;
    let nonexistent_path = test_dir.join("nonexistent.mp4");

    // Define args with nonexistent path
    let args = VideoToFramesArgs {
        input: nonexistent_path.to_string_lossy().to_string(),
        output: "output".to_string(),
    };

    // Execute command and expect error
    let result = video::to_frames::execute(args);
    assert!(result.is_err());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_video_to_frames_invalid_extension() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create a file with invalid extension
    let invalid_path = test_dir.join("test.txt");
    fs::write(&invalid_path, "test content")?;

    // Define args
    let args = VideoToFramesArgs {
        input: invalid_path.to_string_lossy().to_string(),
        output: "output".to_string(),
    };

    // Execute command and expect error
    let result = video::to_frames::execute(args);
    assert!(result.is_err());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}
