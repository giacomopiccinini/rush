use crate::utils::{cleanup_test_dir, create_test_table, setup_test_dir};
use anyhow::Result;
use rush::commands::table;
use rush::TableSchemaArgs;
use std::fs;

#[test]
fn test_table_schema_file_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test files
    let csv_path = test_dir.join("test.csv");
    let parquet_path = test_dir.join("test.parquet");

    create_test_table(&csv_path)?;
    create_test_table(&parquet_path)?;

    // Test CSV file
    let csv_args = TableSchemaArgs {
        input: csv_path.to_string_lossy().to_string(),
    };
    table::schema::execute(csv_args)?;

    // Test Parquet file
    let parquet_args = TableSchemaArgs {
        input: parquet_path.to_string_lossy().to_string(),
    };
    table::schema::execute(parquet_args)?;

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_table_schema_nonexistent_path() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;
    let nonexistent_path = test_dir.join("nonexistent.csv");

    // Define args with nonexistent path
    let args = TableSchemaArgs {
        input: nonexistent_path.to_string_lossy().to_string(),
    };

    // Execute command and expect error
    let result = table::schema::execute(args);
    assert!(result.is_err());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_table_schema_directory() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Define args with directory path
    let args = TableSchemaArgs {
        input: test_dir.to_string_lossy().to_string(),
    };

    // Execute command and expect error
    let result = table::schema::execute(args);
    assert!(result.is_err());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_table_schema_invalid_extension() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create a file with invalid extension
    let invalid_path = test_dir.join("test.txt");
    fs::write(&invalid_path, "test content")?;

    // Define args
    let args = TableSchemaArgs {
        input: invalid_path.to_string_lossy().to_string(),
    };

    // Execute command and expect error
    let result = table::schema::execute(args);
    assert!(result.is_err());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}
