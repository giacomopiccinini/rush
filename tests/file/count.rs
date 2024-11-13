use crate::utils::{cleanup_test_dir, setup_test_dir};
use anyhow::Result;
use rush::commands::file;
use rush::CountArgs;
use std::fs;

#[test]
fn test_file_count_single_file_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test file
    let test_file = test_dir.join("test.txt");
    fs::write(&test_file, "test content")?;

    // Define args
    let args = CountArgs {
        target: test_file.to_string_lossy().to_string(),
    };

    // Execute command
    file::count::execute(args)?;

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_file_count_empty_directory_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Define args
    let args = CountArgs {
        target: test_dir.to_string_lossy().to_string(),
    };

    // Execute command
    file::count::execute(args)?;

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_file_count_directory_with_files_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test files
    let file1 = test_dir.join("test1.txt");
    let file2 = test_dir.join("test2.txt");
    fs::write(&file1, "test content 1")?;
    fs::write(&file2, "test content 2")?;

    // Define args
    let args = CountArgs {
        target: test_dir.to_string_lossy().to_string(),
    };

    // Execute command
    file::count::execute(args)?;

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_file_count_nested_structure_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create nested directory structure
    let subdir1 = test_dir.join("subdir1");
    let subdir2 = test_dir.join("subdir2");
    fs::create_dir(&subdir1)?;
    fs::create_dir(&subdir2)?;

    // Create test files
    let file1 = test_dir.join("test1.txt");
    let file2 = subdir1.join("test2.txt");
    let file3 = subdir2.join("test3.txt");
    fs::write(&file1, "test content 1")?;
    fs::write(&file2, "test content 2")?;
    fs::write(&file3, "test content 3")?;

    // Define args
    let args = CountArgs {
        target: test_dir.to_string_lossy().to_string(),
    };

    // Execute command
    file::count::execute(args)?;

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_file_count_nonexistent_path() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;
    let nonexistent_path = test_dir.join("nonexistent");

    // Define args with nonexistent path
    let args = CountArgs {
        target: nonexistent_path.to_string_lossy().to_string(),
    };

    // Execute command and expect error
    let result = file::count::execute(args);
    assert!(result.is_err());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}
