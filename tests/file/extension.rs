use crate::utils::{cleanup_test_dir, setup_test_dir};
use anyhow::Result;
use rush::commands::file;
use rush::FileExtensionArgs;
use std::fs;

#[test]
fn test_file_extension_single_file_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test file
    let test_file = test_dir.join("test.txt");
    fs::write(&test_file, "test content")?;

    // Define args
    let args = FileExtensionArgs {
        target: test_file.to_string_lossy().to_string(),
    };

    // Execute command
    file::extension::execute(args)?;

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_file_extension_directory_with_files_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test files with different extensions
    let file1 = test_dir.join("test1.txt");
    let file2 = test_dir.join("test2.md");
    let file3 = test_dir.join("test3.txt");
    fs::write(&file1, "test content 1")?;
    fs::write(&file2, "test content 2")?;
    fs::write(&file3, "test content 3")?;

    // Define args
    let args = FileExtensionArgs {
        target: test_dir.to_string_lossy().to_string(),
    };

    // Execute command
    file::extension::execute(args)?;

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_file_extension_no_extension() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test file without extension
    let test_file = test_dir.join("testfile");
    fs::write(&test_file, "test content")?;

    // Define args
    let args = FileExtensionArgs {
        target: test_file.to_string_lossy().to_string(),
    };

    // Execute command
    file::extension::execute(args)?;

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_file_extension_mixed_content() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create files with and without extensions
    let file1 = test_dir.join("test1.txt");
    let file2 = test_dir.join("test2");
    let file3 = test_dir.join("test3.rs");
    fs::write(&file1, "test content 1")?;
    fs::write(&file2, "test content 2")?;
    fs::write(&file3, "test content 3")?;

    // Define args
    let args = FileExtensionArgs {
        target: test_dir.to_string_lossy().to_string(),
    };

    // Execute command
    file::extension::execute(args)?;

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_file_extension_nonexistent_path() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;
    let nonexistent_path = test_dir.join("nonexistent");

    // Define args with nonexistent path
    let args = FileExtensionArgs {
        target: nonexistent_path.to_string_lossy().to_string(),
    };

    // Execute command and expect error
    let result = file::extension::execute(args);
    assert!(result.is_err());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}
