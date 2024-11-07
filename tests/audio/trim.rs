use crate::utils::{cleanup_test_dir, create_test_wav, setup_test_dir};
use anyhow::Result;
use rush::commands::audio;
use rush::AudioTrimArgs;
use std::fs;

#[test]
fn test_audio_trim_file_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test files
    let input_path = test_dir.join("input.wav");
    let output_path = test_dir.join("output.wav");
    create_test_wav(&input_path, 10.0, 44100)?;

    // Define args
    let args = AudioTrimArgs {
        input: input_path.to_string_lossy().to_string(),
        output: output_path.to_string_lossy().to_string(),
        offset: 2.0,
        length: 5.0,
        overwrite: false,
    };

    // Execute command
    audio::trim::execute(args)?;

    // Verify output file exists
    assert!(output_path.exists());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_audio_trim_directory_success() -> Result<()> {
    // Set up the directories for testing
    let test_dir = setup_test_dir()?;
    let input_dir = test_dir.join("input");
    let output_dir = test_dir.join("output");
    fs::create_dir(&input_dir)?;

    // Create test files in nested structure
    let wav_path1 = input_dir.join("test1.wav");
    let nested_dir = input_dir.join("nested");
    fs::create_dir(&nested_dir)?;
    let wav_path2 = nested_dir.join("test2.wav");

    create_test_wav(&wav_path1, 10.0, 44100)?;
    create_test_wav(&wav_path2, 10.0, 44100)?;

    // Define args
    let args = AudioTrimArgs {
        input: input_dir.to_string_lossy().to_string(),
        output: output_dir.to_string_lossy().to_string(),
        offset: 2.0,
        length: 5.0,
        overwrite: false,
    };

    // Execute command
    audio::trim::execute(args)?;

    // Verify output files exist
    assert!(output_dir.join("test1.wav").exists());
    assert!(output_dir.join("nested/test2.wav").exists());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_audio_trim_invalid_offset_error() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test files
    let input_path = test_dir.join("input.wav");
    let output_path = test_dir.join("output.wav");
    create_test_wav(&input_path, 5.0, 44100)?;

    // Define args with offset larger than file duration
    let args = AudioTrimArgs {
        input: input_path.to_string_lossy().to_string(),
        output: output_path.to_string_lossy().to_string(),
        offset: 6.0,
        length: 2.0,
        overwrite: false,
    };

    // Execute command and expect error
    let result = audio::trim::execute(args);
    assert!(result.is_err());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_audio_trim_invalid_length_error() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test files
    let input_path = test_dir.join("input.wav");
    let output_path = test_dir.join("output.wav");
    create_test_wav(&input_path, 5.0, 44100)?;

    // Define args with offset + length larger than file duration
    let args = AudioTrimArgs {
        input: input_path.to_string_lossy().to_string(),
        output: output_path.to_string_lossy().to_string(),
        offset: 2.0,
        length: 4.0,
        overwrite: false,
    };

    // Execute command and expect error
    let result = audio::trim::execute(args);
    assert!(result.is_err());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_audio_trim_overwrite_protection_error() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test file
    let input_path = test_dir.join("input.wav");
    create_test_wav(&input_path, 10.0, 44100)?;

    // Try to overwrite input file without overwrite flag
    let args = AudioTrimArgs {
        input: input_path.to_string_lossy().to_string(),
        output: input_path.to_string_lossy().to_string(),
        offset: 2.0,
        length: 5.0,
        overwrite: false,
    };

    // Execute command and expect error
    let result = audio::trim::execute(args);
    assert!(result.is_err());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_audio_trim_overwrite_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test file
    let input_path = test_dir.join("input.wav");
    create_test_wav(&input_path, 10.0, 44100)?;

    // Try to overwrite input file with overwrite flag
    let args = AudioTrimArgs {
        input: input_path.to_string_lossy().to_string(),
        output: input_path.to_string_lossy().to_string(),
        offset: 2.0,
        length: 5.0,
        overwrite: true,
    };

    // Execute command
    audio::trim::execute(args)?;

    // Verify file still exists
    assert!(input_path.exists());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}
