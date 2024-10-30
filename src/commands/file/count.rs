use crate::CountArgs;
use anyhow::Result;
use rayon::prelude::*;
use std::path::Path;
use walkdir::WalkDir;

pub fn execute(args: CountArgs) -> Result<()> {
    // Convert to path object
    let target_path = Path::new(&args.target);

    if !target_path.exists() {
        return Err(anyhow::Error::msg("Target does not exist"));
    }

    // If target is a file, return trivial counting
    if target_path.is_file() {
        println!("Files: 1");
        println!("Directories: 0");
    }
    // If it is a directory
    else {
        // Collect entries first
        let entries: Vec<_> = WalkDir::new(target_path)
            .max_depth(1)
            .into_iter()
            .filter_map(Result::ok)
            .collect();

        // Count files and directories in parallel
        let (n_files, n_sub_directories) = entries
            .par_iter()
            .fold(
                || (0, 0),
                |(file_count, dir_count), entry| {
                    if entry.path().is_file() {
                        (file_count + 1, dir_count)
                    } else if entry.path().is_dir() && entry.depth() == 1 {
                        (file_count, dir_count + 1)
                    } else {
                        (file_count, dir_count)
                    }
                },
            )
            .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1));

        // Print results
        println!("Files: {}", n_files);
        println!("Directories: {}", n_sub_directories);
    }

    Ok(())
}
