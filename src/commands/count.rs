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
        // Count files and directories
        let (n_files, n_sub_directories) = WalkDir::new(&target_path)
            .max_depth(1)
            .into_iter()
            .filter_map(Result::ok)
            .fold((0, 0), |(file_count, dir_count), entry| {
                if entry.path().is_file() {
                    (file_count + 1, dir_count)
                } else if entry.path().is_dir() && entry.depth() == 1 {
                    (file_count, dir_count + 1)
                } else {
                    (file_count, dir_count)
                }
            });

        // Print results
        println!("Files: {}", n_files);
        println!("Directories: {}", n_sub_directories);
    }
}
