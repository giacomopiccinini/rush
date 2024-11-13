use crate::utils::{cleanup_test_dir, create_test_image, setup_test_dir};
use anyhow::Result;
use rush::commands::image;
use rush::ImageTessellateArgs;
use std::fs;

#[test]
fn test_image_tessellate_file_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test files
    let input_path = test_dir.join("input.png");
    let output_path = test_dir.join("output");
    create_test_image(&input_path, 100, 100, 3)?;

    // Define args
    let args = ImageTessellateArgs {
        input: input_path.to_string_lossy().to_string(),
        output: output_path.to_string_lossy().to_string(),
        n_vertical: 2,
        n_horizontal: 2,
        delete_original: false,
    };

    // Execute command
    image::tessellate::execute(args)?;

    // Verify output files exist
    assert!(output_path.join("input_id0_w0-50_h0-50.png").exists());
    assert!(output_path.join("input_id1_w50-100_h0-50.png").exists());
    assert!(output_path.join("input_id2_w0-50_h50-100.png").exists());
    assert!(output_path.join("input_id3_w50-100_h50-100.png").exists());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_image_tessellate_directory_success() -> Result<()> {
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

    create_test_image(&img_path1, 100, 100, 1)?;
    create_test_image(&img_path2, 200, 200, 3)?;

    // Define args
    let args = ImageTessellateArgs {
        input: input_dir.to_string_lossy().to_string(),
        output: output_dir.to_string_lossy().to_string(),
        n_vertical: 2,
        n_horizontal: 2,
        delete_original: false,
    };

    // Execute command
    image::tessellate::execute(args)?;

    // Verify output files exist for first image
    assert!(output_dir.join("test1_id0_w0-50_h0-50.png").exists());
    assert!(output_dir.join("test1_id1_w50-100_h0-50.png").exists());
    assert!(output_dir.join("test1_id2_w0-50_h50-100.png").exists());
    assert!(output_dir.join("test1_id3_w50-100_h50-100.png").exists());

    // Verify output files exist for second image in nested directory
    assert!(output_dir
        .join("nested/test2_id0_w0-100_h0-100.png")
        .exists());
    assert!(output_dir
        .join("nested/test2_id1_w100-200_h0-100.png")
        .exists());
    assert!(output_dir
        .join("nested/test2_id2_w0-100_h100-200.png")
        .exists());
    assert!(output_dir
        .join("nested/test2_id3_w100-200_h100-200.png")
        .exists());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_image_tessellate_with_delete_original() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test file
    let input_path = test_dir.join("input.png");
    let output_path = test_dir.join("output");
    create_test_image(&input_path, 100, 100, 3)?;

    // Define args with delete_original set to true
    let args = ImageTessellateArgs {
        input: input_path.to_string_lossy().to_string(),
        output: output_path.to_string_lossy().to_string(),
        n_vertical: 2,
        n_horizontal: 2,
        delete_original: true,
    };

    // Execute command
    image::tessellate::execute(args)?;

    // Verify output files exist
    assert!(output_path.join("input_id0_w0-50_h0-50.png").exists());
    assert!(output_path.join("input_id1_w50-100_h0-50.png").exists());
    assert!(output_path.join("input_id2_w0-50_h50-100.png").exists());
    assert!(output_path.join("input_id3_w50-100_h50-100.png").exists());

    // Verify original file was deleted
    assert!(!input_path.exists());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}
