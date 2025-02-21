use anyhow::Result;
use rayon::prelude::*;
use std::path::Path;
use walkdir::WalkDir;
use std::collections::HashMap;


use crate::FileExtensionArgs;

// Execute the extension command
pub fn execute(args: FileExtensionArgs) -> Result<()> {
    // Parse the arguments
    let target = Path::new(&args.target);

    // Check if the target directory actually exists
    if !target.exists() {
        return Err(anyhow::Error::msg("Directory does not exist"));
    }

    // If target is file, return its extension
    if target.is_file(){
        println!("{:?}: 1", target.extension().and_then(|ext| ext.to_str()));
    }
    // If it's a directory, find all extensions
    else{
        // Collect extensions and count them using a HashMap
        let extension_counts: HashMap<String, usize> = WalkDir::new(target)
            .max_depth(1)
            .into_iter()
            .filter_map(Result::ok)
            .par_bridge() // Convert to parallel iterator
            .filter_map(|entry| entry.path().extension()?.to_str().map(String::from))
            .fold(
                || HashMap::new(),
                |mut acc, ext| {
                    *acc.entry(ext).or_insert(0) += 1;
                    acc
                },
            )
            .reduce(
                || HashMap::new(),
                |mut map1, map2| {
                    map2.into_iter().for_each(|(key, value)| {
                        *map1.entry(key).or_insert(0) += value;
                    });
                    map1
                },
            );

        // Print each extension and its count
        for (ext, count) in extension_counts.iter() {
            println!("{}: {}", ext, count);
        }
    }

    Ok(())
}
