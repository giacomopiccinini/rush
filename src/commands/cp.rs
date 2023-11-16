use std::fs::{copy, create_dir_all};
use std::path::PathBuf;
use walkdir::WalkDir;

// Import the argument struct
use crate::CpArgs; 

pub fn execute(args: CpArgs) {
    // Convert to path object
    let source_path = PathBuf::from(&args.source);
    let target_path = PathBuf::from(&args.target);

    // Understand if source path is a directory or a file
    let source_is_dir = source_path.is_dir();

    // Only apply if the source is a directory
    if source_is_dir {
        // Fetch all the subdirectories, these will need to be created first
        // Notice: we start at depth 1 so we remove the source itself
        let sub_directories = WalkDir::new(&source_path)
            .min_depth(1)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.path().is_dir())
            .map(|e| e.path().strip_prefix(&args.source).map(|p| p.to_path_buf()))
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        // Create all sub-directories i.e. recreate the directory structure
        for sub_directory in sub_directories.into_iter() {
            create_dir_all(&target_path.join(&sub_directory))
                .expect("Failed to create sub-directory");
        }
    };

    // Get only the files, exclude the directories
    let files = if source_is_dir {
        // If it is a directory, we retrieve all files recursively
        // and we strip of the source part, because this will need to be replaced
        WalkDir::new(&source_path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.path().is_file())
            .map(|e| e.path().strip_prefix(&args.source).map(|p| p.to_path_buf()))
            .filter_map(Result::ok)
            .collect::<Vec<_>>()
    } else {
        // If it is a single file, we simply get its name and put it into a vector
        vec![PathBuf::from(
            &source_path.file_name().expect("Can't read file"),
        )]
    };

    // Underdtand if the target path is a directory or not
    let target_is_dir = if target_path.exists() {
        // If it exists already, return the truth value of is_dir
        target_path.is_dir()
    } else {
        // If it does not exists:
        // If the source is a directory, the target must be a directory too
        if source_is_dir {
            // Create the target directory because it does not exist
            create_dir_all(&target_path).expect("Failed to create target directory");
            // Return that the target is a directory
            true
        } else {
            // The source is just a file, we don't need to create it
            // and the target will not be a dir as well
            false
        }
    };

    // Copy all files
    for file in files {
        // If the target is a directory
        if target_is_dir == true {
            // If the source if a file, copy it in the target directory
            if !source_is_dir {
                copy(&source_path, &target_path.join(&file)).expect("Can't copy file");
            } else {
                // If it is a directory copy all files recursively
                copy(&source_path.join(&file), &target_path.join(&file)).expect("Can't copy file");
            }
        } else {
            copy(&source_path, &target_path).unwrap();
        }
    }
}

