use std::path::Path;
use anyhow::{Context, Result};

use crate::utils::{read_table};

use crate::TableSchemaArgs;

// Execute the schema command
pub fn execute(args: TableSchemaArgs) -> Result<()>{

    // Convert to path
    let input = Path::new(&args.input);

    // Sanity check
    if !input.exists(){
        return Err(anyhow::Error::msg(format!("File {:?} does not exist", input)));
    }

    if input.is_dir(){
        return Err(anyhow::Error::msg(format!("{:?} is a directory", input)));
    }

    // Read the df
    let mut df = read_table(input).with_context(|| format!("File {:?} could not be read", input))?;

    // Extract schema
    let schema = df.collect_schema().with_context(|| format!("Couldn't extract schema from {:?}", input))?;

    // Print schema
    println!("{:?}", schema);

    Ok(())
}

