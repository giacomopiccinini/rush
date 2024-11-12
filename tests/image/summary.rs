use crate::utils::{cleanup_test_dir, create_test_image, setup_test_dir};
use anyhow::Result;
use rush::commands::image;
use rush::ImageSummaryArgs;
use std::fs;

#[test]
fn test_image_summary_file_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test file
    let input_path = test_dir.join("input.png");
    create_test_image(&input_path, 100, 100, 3)?;

    // Define args
    let args = ImageSummaryArgs {
        target: input_path.to_string_lossy().to_string(),
    };

    // Execute command
    image::summary::execute(args)?;

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_image_summary_directory_success() -> Result<()> {
    // Set up the directories for testing
    let test_dir = setup_test_dir()?;
    let input_dir = test_dir.join("input");
    fs::create_dir(&input_dir)?;

    // Create test files in nested structure
    let img_path1 = input_dir.join("test1.png");
    let nested_dir = input_dir.join("nested");
    fs::create_dir(&nested_dir)?;
    let img_path2 = nested_dir.join("test2.png");

    create_test_image(&img_path1, 100, 100, 1)?;
    create_test_image(&img_path2, 200, 200, 3)?;

    // Define args
    let args = ImageSummaryArgs {
        target: input_dir.to_string_lossy().to_string(),
    };

    // Execute command
    image::summary::execute(args)?;

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_image_summary_nonexistent_path() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;
    let nonexistent_path = test_dir.join("nonexistent.png");

    // Define args with nonexistent path
    let args = ImageSummaryArgs {
        target: nonexistent_path.to_string_lossy().to_string(),
    };

    // Execute command and expect error
    let result = image::summary::execute(args);
    assert!(result.is_err());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_image_summary_invalid_extension() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create a file with invalid extension
    let invalid_path = test_dir.join("test.txt");
    fs::write(&invalid_path, "test content")?;

    // Define args
    let args = ImageSummaryArgs {
        target: invalid_path.to_string_lossy().to_string(),
    };

    // Execute command and expect error
    let result = image::summary::execute(args);
    assert!(result.is_err());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}
