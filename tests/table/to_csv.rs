use crate::utils::{cleanup_test_dir, create_test_table, setup_test_dir};
use anyhow::Result;
use rush::commands::table;
use rush::TableToCsvArgs;
use std::fs;
use std::path::Path;

#[test]
fn test_table_to_csv_file_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test files
    let input_path = test_dir.join("test.parquet");
    let output_path = test_dir.join("test.csv");

    create_test_table(&input_path)?;

    // Test Parquet to CSV conversion
    let args = TableToCsvArgs {
        input: input_path.to_string_lossy().to_string(),
        output: output_path.to_string_lossy().to_string(),
    };
    table::to_csv::execute(args)?;

    // Verify output file exists
    assert!(Path::new(&output_path).exists());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_table_to_csv_nonexistent_path() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;
    let input_path = test_dir.join("nonexistent.parquet");
    let output_path = test_dir.join("output.csv");

    // Define args with nonexistent path
    let args = TableToCsvArgs {
        input: input_path.to_string_lossy().to_string(),
        output: output_path.to_string_lossy().to_string(),
    };

    // Execute command and expect error
    let result = table::to_csv::execute(args);
    assert!(result.is_err());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_table_to_csv_invalid_input_extension() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create a file with invalid extension
    let input_path = test_dir.join("test.txt");
    let output_path = test_dir.join("test.csv");
    fs::write(&input_path, "test content")?;

    // Define args
    let args = TableToCsvArgs {
        input: input_path.to_string_lossy().to_string(),
        output: output_path.to_string_lossy().to_string(),
    };

    // Execute command and expect error
    let result = table::to_csv::execute(args);
    assert!(result.is_err());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_table_to_csv_directory_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;
    let input_dir = test_dir.join("input");
    let output_dir = test_dir.join("output");

    // Create input directory and test files
    fs::create_dir(&input_dir)?;
    create_test_table(&input_dir.join("test1.parquet"))?;
    create_test_table(&input_dir.join("test2.parquet"))?;

    // Test directory conversion
    let args = TableToCsvArgs {
        input: input_dir.to_string_lossy().to_string(),
        output: output_dir.to_string_lossy().to_string(),
    };
    table::to_csv::execute(args)?;

    // Verify output files exist
    assert!(Path::new(&output_dir.join("test1.csv")).exists());
    assert!(Path::new(&output_dir.join("test2.csv")).exists());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}
