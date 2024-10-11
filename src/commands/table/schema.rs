use std::path::PathBuf;
use std::collections::HashSet;
use polars::prelude::*;

use crate::TableSchemaArgs;

// Check if the file is an table file
fn is_table_file(path: &PathBuf, extensions: &HashSet<String>) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| extensions.contains(&ext.to_lowercase()))
        .unwrap_or(false)
}

// Read table
fn read_table(path: &PathBuf) -> Result<LazyFrame, PolarsError> {
    // Extract extension
    let extension = path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
        .unwrap_or_default();

    match extension.as_str() {
        "parquet" => LazyFrame::scan_parquet(path, Default::default()),
        "csv" =>  LazyCsvReader::new(path).finish(),
        _ => Err(PolarsError::ComputeError("Unsupported file format".into())),
    }
}

// Execute the schema command
pub fn execute(args: TableSchemaArgs) {

    // Convert to path
    let target = PathBuf::from(&args.target);

    // Sanity checks
    if target.is_dir(){
        println!("ERROR: Target must be a file");
        return;
    }
    if !target.exists(){
        println!("ERROR: Target does not exist");
        return
    }

    // Define allowed extensions for images
    let table_extensions: HashSet<_> = ["parquet", "csv"]
    .iter().map(|&s| s.to_lowercase()).collect();

    // Check that the file is a valid table
    if !is_table_file(&target, &table_extensions){
        println!("ERROR: File format not admissible");
        return
    }

    // Read the df
    match read_table(&target) {
        Ok(mut df) => {
            let schema = df.collect_schema().unwrap();
            println!("{:?}", schema);
        },
        Err(e) => println!("Error reading table: {:?}", e),
    }
}

