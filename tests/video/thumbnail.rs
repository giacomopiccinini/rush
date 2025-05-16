use crate::utils::{cleanup_test_dir, create_test_video, setup_test_dir};
use anyhow::Result;
use rush::commands::video;
use rush::VideoThumbnailArgs;
use std::fs;
use std::path::Path;

#[test]
fn test_video_thumbnail_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test file
    let input_path = test_dir.join("input.mp4");
    create_test_video(&input_path, 1920, 1080, 5.0, 30)?;

    // Define args
    let args = VideoThumbnailArgs {
        input: input_path.to_string_lossy().to_string(),
        output: test_dir.to_string_lossy().to_string(),
    };

    // Execute command
    video::thumbnail::execute(args)?;

    // Check if thumbnail was created
    let thumbnail_path = test_dir.join("input.jpeg");
    assert!(Path::new(&thumbnail_path).exists());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_video_thumbnail_nonexistent_path() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;
    let nonexistent_path = test_dir.join("nonexistent.mp4");

    // Define args with nonexistent path
    let args = VideoThumbnailArgs {
        input: nonexistent_path.to_string_lossy().to_string(),
        output: test_dir.to_string_lossy().to_string(),
    };

    // Execute command and expect error
    let result = video::thumbnail::execute(args);
    assert!(result.is_err());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_video_thumbnail_invalid_extension() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create a file with invalid extension
    let invalid_path = test_dir.join("test.txt");
    fs::write(&invalid_path, "test content")?;

    // Define args
    let args = VideoThumbnailArgs {
        input: invalid_path.to_string_lossy().to_string(),
        output: test_dir.to_string_lossy().to_string(),
    };

    // Execute command and expect error
    let result = video::thumbnail::execute(args);
    assert!(result.is_err());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}
