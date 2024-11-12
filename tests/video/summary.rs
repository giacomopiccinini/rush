use crate::utils::{cleanup_test_dir, create_test_video, setup_test_dir};
use anyhow::Result;
use rush::commands::video;
use rush::VideoSummaryArgs;
use std::fs;

#[test]
fn test_video_summary_file_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test file
    let input_path = test_dir.join("input.mp4");
    create_test_video(&input_path, 1920, 1080, 5.0, 30)?;

    // Define args
    let args = VideoSummaryArgs {
        target: input_path.to_string_lossy().to_string(),
    };

    // Execute command
    video::summary::execute(args)?;

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_video_summary_directory_success() -> Result<()> {
    // Set up the directories for testing
    let test_dir = setup_test_dir()?;
    let input_dir = test_dir.join("input");
    fs::create_dir(&input_dir)?;

    // Create test files in nested structure
    let video_path1 = input_dir.join("test1.mp4");
    let nested_dir = input_dir.join("nested");
    fs::create_dir(&nested_dir)?;
    let video_path2 = nested_dir.join("test2.mp4");

    create_test_video(&video_path1, 1280, 720, 3.0, 24)?;
    create_test_video(&video_path2, 1920, 1080, 5.0, 30)?;

    // Define args
    let args = VideoSummaryArgs {
        target: input_dir.to_string_lossy().to_string(),
    };

    // Execute command
    video::summary::execute(args)?;

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_video_summary_nonexistent_path() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;
    let nonexistent_path = test_dir.join("nonexistent.mp4");

    // Define args with nonexistent path
    let args = VideoSummaryArgs {
        target: nonexistent_path.to_string_lossy().to_string(),
    };

    // Execute command and expect error
    let result = video::summary::execute(args);
    assert!(result.is_err());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_video_summary_invalid_extension() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create a file with invalid extension
    let invalid_path = test_dir.join("test.txt");
    fs::write(&invalid_path, "test content")?;

    // Define args
    let args = VideoSummaryArgs {
        target: invalid_path.to_string_lossy().to_string(),
    };

    // Execute command and expect error
    let result = video::summary::execute(args);
    assert!(result.is_err());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}
