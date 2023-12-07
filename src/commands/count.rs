use crate::CountArgs;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn execute(args: CountArgs) {
    // Convert to path object
    let target_path = PathBuf::from(&args.target);

    // If target is a file, return trivial counting
    if target_path.is_file() {
        println!("Files: 1");
        println!("Directories: 0");
    }
    // If it is a directory
    else {
        // Count number of files
        let n_files = WalkDir::new(&target_path)
            .max_depth(1)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.path().is_file())
            .collect::<Vec<_>>()
            .len();

        // Count number of directories
        let n_sub_directories = WalkDir::new(&target_path)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.path().is_dir())
            .collect::<Vec<_>>()
            .len();

        // Print results
        println!("Files: {}", n_files);
        println!("Directories: {}", n_sub_directories);
    }
}
