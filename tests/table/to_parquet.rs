use crate::utils::{cleanup_test_dir, create_test_table, setup_test_dir};
use anyhow::Result;
use rush::commands::table;
use rush::TableToParquetArgs;
use std::fs;
use std::path::Path;

#[test]
fn test_table_to_parquet_file_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test files
    let input_path = test_dir.join("test.csv");
    let output_path = test_dir.join("test.parquet");

    create_test_table(&input_path)?;

    // Test CSV to Parquet conversion
    let args = TableToParquetArgs {
        input: input_path.to_string_lossy().to_string(),
        output: output_path.to_string_lossy().to_string(),
    };
    table::to_parquet::execute(args)?;

    // Verify output file exists
    assert!(Path::new(&output_path).exists());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_table_to_parquet_nonexistent_path() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;
    let input_path = test_dir.join("nonexistent.csv");
    let output_path = test_dir.join("output.parquet");

    // Define args with nonexistent path
    let args = TableToParquetArgs {
        input: input_path.to_string_lossy().to_string(),
        output: output_path.to_string_lossy().to_string(),
    };

    // Execute command and expect error
    let result = table::to_parquet::execute(args);
    assert!(result.is_err());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_table_to_parquet_invalid_input_extension() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create a file with invalid extension
    let input_path = test_dir.join("test.txt");
    let output_path = test_dir.join("test.parquet");
    fs::write(&input_path, "test content")?;

    // Define args
    let args = TableToParquetArgs {
        input: input_path.to_string_lossy().to_string(),
        output: output_path.to_string_lossy().to_string(),
    };

    // Execute command and expect error
    let result = table::to_parquet::execute(args);
    assert!(result.is_err());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_table_to_parquet_directory_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;
    let input_dir = test_dir.join("input");
    let output_dir = test_dir.join("output");

    // Create input directory and test files
    fs::create_dir(&input_dir)?;
    create_test_table(&input_dir.join("test1.csv"))?;
    create_test_table(&input_dir.join("test2.csv"))?;

    // Test directory conversion
    let args = TableToParquetArgs {
        input: input_dir.to_string_lossy().to_string(),
        output: output_dir.to_string_lossy().to_string(),
    };
    table::to_parquet::execute(args)?;

    // Verify output files exist
    assert!(Path::new(&output_dir.join("test1.parquet")).exists());
    assert!(Path::new(&output_dir.join("test2.parquet")).exists());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}
