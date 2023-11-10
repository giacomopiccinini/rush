use clap::Parser;
use std::path::PathBuf;
use walkdir::WalkDir;
use std::fs::{copy, create_dir_all};


/// Rust implementation of bash' cp
#[derive(Parser, Debug)]
struct Args {
    /// Source directory or file
    #[arg(required = true)]
    source: String,

    /// Target directory or file
    #[arg(required = true)]
    target: String,
}

fn main() {

    // Parse the arguments
    let args = Args::parse();

    // Convert to path object
    let source_path = PathBuf::from(&args.source);
    let target_path = PathBuf::from(&args.target);

    // Get only the files, exclude the directories
    let files = if source_path.is_dir(){
        // If it is a directory, we retrieve all files recursively
        // and we strip of the source part, because this will need to be replaced
        WalkDir::new(&source_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
        .map(|e| e.path()
                  .strip_prefix(&args.source)
                  .map(|p| p.to_path_buf()))
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
    } else {
        // If it is a single file, we simply get its name and put it into a vector
        vec![PathBuf::from(source_path.clone().file_name().unwrap())]
    };

    // Underdtand if the target path is a directory or not
    let target_is_dir = if target_path.exists(){
        // If it exists already, return the truth value of is_dir
        target_path.is_dir()
    } else {
        // If it does not exists:
        // If the source is a directory, the target must be a directory too
        if source_path.is_dir() {
            // Create the target directory because it does not exist
            create_dir_all(&target_path).unwrap();
            // Return that the target is a directory
            true
        } else{
            // The source is just a file, we don't need to create it
            // and the target will not be a dir as well
            false
        }
    };

    // Copy all files 
    for file in files{
        // If the target is a directory
        if target_is_dir == true {
            // If the source if a file, copy it in the target directory
            if source_path.is_file(){
                copy(&source_path, &target_path.join(&file)).unwrap();
            } else{
                // If it is a directory copy all files recursively
                copy(&source_path.join(&file), &target_path.join(&file)).unwrap();
            }
        } else {
            copy(&source_path, &target_path).unwrap();
        }
    }

}