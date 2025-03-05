use crate::utils::{cleanup_test_dir, create_test_video, setup_test_dir};
use anyhow::Result;
use rush::commands::video;
use rush::VideoDuplicatesArgs;

#[test]
fn test_video_duplicates_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test videos - two identical and one different
    let video_path1 = test_dir.join("video1.mp4");
    let video_path2 = test_dir.join("video2.mp4");
    let video_path3 = test_dir.join("video3.mp4");

    // Create identical videos (same parameters)
    create_test_video(&video_path1, 1280, 720, 3.0, 24)?;
    create_test_video(&video_path2, 1280, 720, 3.0, 24)?;
    // Create different video
    create_test_video(&video_path3, 1920, 1080, 5.0, 30)?;

    // Define args
    let args = VideoDuplicatesArgs {
        target: test_dir.to_string_lossy().to_string(),
    };

    // Execute command
    video::duplicates::execute(args)?;

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_video_duplicates_nonexistent_path() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;
    let nonexistent_path = test_dir.join("nonexistent");

    // Define args with nonexistent path
    let args = VideoDuplicatesArgs {
        target: nonexistent_path.to_string_lossy().to_string(),
    };

    // Execute command and expect error
    let result = video::duplicates::execute(args);
    assert!(result.is_err());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_video_duplicates_not_a_directory() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create a single video file
    let video_path = test_dir.join("video.mp4");
    create_test_video(&video_path, 1280, 720, 3.0, 24)?;

    // Define args with a file path instead of directory
    let args = VideoDuplicatesArgs {
        target: video_path.to_string_lossy().to_string(),
    };

    // Execute command and expect error
    let result = video::duplicates::execute(args);
    assert!(result.is_err());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_video_duplicates_insufficient_files() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create a single video file
    let video_path = test_dir.join("video.mp4");
    create_test_video(&video_path, 1280, 720, 3.0, 24)?;

    // Define args
    let args = VideoDuplicatesArgs {
        target: test_dir.to_string_lossy().to_string(),
    };

    // Execute command and expect error (needs at least 2 files)
    let result = video::duplicates::execute(args);
    assert!(result.is_err());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_video_duplicates_multiple_formats() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test videos with different extensions
    let video_path1 = test_dir.join("video1.mp4");
    let video_path2 = test_dir.join("video2.mkv");
    let video_path3 = test_dir.join("video3.mov");

    // Create videos with same content parameters
    create_test_video(&video_path1, 1280, 720, 3.0, 24)?;
    create_test_video(&video_path2, 1280, 720, 3.0, 24)?;
    create_test_video(&video_path3, 1280, 720, 3.0, 24)?;

    // Define args
    let args = VideoDuplicatesArgs {
        target: test_dir.to_string_lossy().to_string(),
    };

    // Execute command
    video::duplicates::execute(args)?;

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}
