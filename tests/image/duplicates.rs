use crate::utils::{cleanup_test_dir, create_test_image, setup_test_dir};
use anyhow::Result;
use rush::commands::image;
use rush::ImageDuplicatesArgs;

#[test]
fn test_image_duplicates_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test images - two identical and one different
    let image_path1 = test_dir.join("image1.png");
    let image_path2 = test_dir.join("image2.png");
    let image_path3 = test_dir.join("image3.png");

    // Create identical images (same parameters)
    create_test_image(&image_path1, 100, 100, 3)?;
    create_test_image(&image_path2, 100, 100, 3)?;
    // Create different image
    create_test_image(&image_path3, 200, 200, 3)?;

    // Define args
    let args = ImageDuplicatesArgs {
        target: test_dir.to_string_lossy().to_string(),
    };

    // Execute command
    image::duplicates::execute(args)?;

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_image_duplicates_nonexistent_path() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;
    let nonexistent_path = test_dir.join("nonexistent");

    // Define args with nonexistent path
    let args = ImageDuplicatesArgs {
        target: nonexistent_path.to_string_lossy().to_string(),
    };

    // Execute command and expect error
    let result = image::duplicates::execute(args);
    assert!(result.is_err());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_image_duplicates_not_a_directory() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create a single image file
    let image_path = test_dir.join("image.png");
    create_test_image(&image_path, 100, 100, 3)?;

    // Define args with a file path instead of directory
    let args = ImageDuplicatesArgs {
        target: image_path.to_string_lossy().to_string(),
    };

    // Execute command and expect error
    let result = image::duplicates::execute(args);
    assert!(result.is_err());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_image_duplicates_insufficient_files() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create a single image file
    let image_path = test_dir.join("image.png");
    create_test_image(&image_path, 100, 100, 3)?;

    // Define args
    let args = ImageDuplicatesArgs {
        target: test_dir.to_string_lossy().to_string(),
    };

    // Execute command and expect error (needs at least 2 files)
    let result = image::duplicates::execute(args);
    assert!(result.is_err());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_image_duplicates_multiple_formats() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test images with different extensions
    let image_path1 = test_dir.join("image1.png");
    let image_path2 = test_dir.join("image2.jpg");
    let image_path3 = test_dir.join("image3.bmp");

    // Create images with same content parameters
    create_test_image(&image_path1, 100, 100, 3)?;
    create_test_image(&image_path2, 100, 100, 3)?;
    create_test_image(&image_path3, 100, 100, 3)?;

    // Define args
    let args = ImageDuplicatesArgs {
        target: test_dir.to_string_lossy().to_string(),
    };

    // Execute command
    image::duplicates::execute(args)?;

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}
