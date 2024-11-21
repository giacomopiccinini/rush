use crate::utils::{cleanup_test_dir, create_test_image, setup_test_dir};
use anyhow::Result;
use rush::commands::image;
use rush::ImageToPortraitArgs;
use std::fs;

#[test]
fn test_image_to_portrait_file_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test files
    let input_path = test_dir.join("input.png");
    let output_path = test_dir.join("output.png");
    create_test_image(&input_path, 100, 50, 3)?;

    // Define args
    let args = ImageToPortraitArgs {
        input: input_path.to_string_lossy().to_string(),
        output: output_path.to_string_lossy().to_string(),
        overwrite: false,
    };

    // Execute command
    image::to_portrait::execute(args)?;

    // Verify output file exists
    assert!(output_path.exists());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_image_to_portrait_directory_success() -> Result<()> {
    // Set up the directories for testing
    let test_dir = setup_test_dir()?;
    let input_dir = test_dir.join("input");
    let output_dir = test_dir.join("output");
    fs::create_dir(&input_dir)?;

    // Create test files in nested structure
    let img_path1 = input_dir.join("test1.png");
    let nested_dir = input_dir.join("nested");
    fs::create_dir(&nested_dir)?;
    let img_path2 = nested_dir.join("test2.png");

    create_test_image(&img_path1, 100, 50, 3)?;
    create_test_image(&img_path2, 100, 200, 3)?;

    // Define args
    let args = ImageToPortraitArgs {
        input: input_dir.to_string_lossy().to_string(),
        output: output_dir.to_string_lossy().to_string(),
        overwrite: false,
    };

    // Execute command
    image::to_portrait::execute(args)?;

    // Verify output files exist
    assert!(output_dir.join("test1.png").exists());
    assert!(output_dir.join("nested/test2.png").exists());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_image_to_portrait_overwrite_protection_error() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test file
    let input_path = test_dir.join("input.png");
    create_test_image(&input_path, 100, 50, 3)?;

    // Try to overwrite input file without overwrite flag
    let args = ImageToPortraitArgs {
        input: input_path.to_string_lossy().to_string(),
        output: input_path.to_string_lossy().to_string(),
        overwrite: false,
    };

    // Execute command and expect error
    let result = image::to_portrait::execute(args);
    assert!(result.is_err());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_image_to_portrait_overwrite_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test file
    let input_path = test_dir.join("input.png");
    create_test_image(&input_path, 100, 50, 3)?;

    // Try to overwrite input file with overwrite flag
    let args = ImageToPortraitArgs {
        input: input_path.to_string_lossy().to_string(),
        output: input_path.to_string_lossy().to_string(),
        overwrite: true,
    };

    // Execute command
    image::to_portrait::execute(args)?;

    // Verify file still exists
    assert!(input_path.exists());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}
