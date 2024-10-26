use std::path::Path;
use std::io;
use std::fs;

// Check if file with given path has one of the desired extensions
pub fn file_has_right_extension(path: &Path, extensions: &[&str]) -> Result<(), io::Error> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) if extensions.iter().any(|&e| ext.eq_ignore_ascii_case(e)) => Ok(()),
        _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid file extension")),
    }
}

// Check that I/O make sense
pub fn perform_io_sanity_check(input: &Path, output: &Path, allow_many_to_one: bool) -> Result<(), io::Error> {

    // Check if input exists (be it a file or a directory)
    if !input.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Input file or directory does not exist"));
    }

    // Determine if output is intended to be a file or directory
    let output_is_file = output.extension().is_some();

    // If we are aiming for a directory and it does not exist, we create it
    if !output_is_file && !output.exists(){
        std::fs::create_dir_all(output)?;
    }

    // Unless explicitly requested, it is not allowed to turn content of a directory into a single file
    if input.is_dir() && output_is_file && !allow_many_to_one {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Cannot convert directory contents to a single file"));
    }

    Ok(())
}


// Find files and directories recursively
pub fn find_files_and_directories_recursively(input: &Path) -> Result<Vec<fs::DirEntry>, io::Error> {
    fs::read_dir(input)?.collect::<Result<Vec<_>, _>>()
}

