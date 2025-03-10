use anyhow::{Context, Result};
use std::path::Path;

use crate::utils::read_table;

use crate::TableSummaryArgs;

pub fn execute(args: TableSummaryArgs) -> Result<()> {
    // Parse the arguments
    let target = Path::new(&args.target);

    // Error if it does not exist at all
    if !target.exists() {
        return Err(anyhow::Error::msg("File does not exist"));
    }

    // Check that the target is a file
    if target.is_dir() {
        return Err(anyhow::Error::msg(format!("{:?} is a directory", target)));
    }

    // Read the df
    let df = read_table(target)
        .with_context(|| format!("File {:?} could not be read", target))?
        .collect()
        .with_context(|| format!("Cannot collect Dataframe"))?;

    println!("{:?}", df.head(Some(5_usize)));
    println!("------------");
    println!("{:?}", df.tail(Some(5_usize)));

    Ok(())
}
