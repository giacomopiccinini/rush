use crate::utils::{cleanup_test_dir, create_test_wav, setup_test_dir};
use anyhow::Result;
use hound::WavReader;
use rush::commands::audio;
use rush::AudioSplitArgs;
use std::fs;

#[test]
fn test_audio_split_file_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test files
    let input_path = test_dir.join("input.wav");
    let output_dir = test_dir.join("output");
    create_test_wav(&input_path, 5.0, 44100)?;

    // Define args
    let args = AudioSplitArgs {
        input: input_path.to_string_lossy().to_string(),
        output: output_dir.to_string_lossy().to_string(),
        chunk_duration: 1.0,
        delete_original: false,
    };

    // Execute command
    audio::split::execute(args)?;

    // Verify output files exist and have correct chunk_duration
    let output_files: Vec<_> = fs::read_dir(&output_dir)?
        .filter_map(|entry| entry.ok())
        .collect();

    assert_eq!(output_files.len(), 5); // 5 second file split into 1 second chunks

    for file in output_files {
        let reader = WavReader::open(file.path())?;
        let chunk_duration = reader.duration() as f32 / reader.spec().sample_rate as f32;
        assert!((chunk_duration - 1.0).abs() < 0.1); // Allow small rounding differences
    }

    // Check file has not been cancelled
    assert!(input_path.exists());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_audio_split_directory_success() -> Result<()> {
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

    create_test_wav(&wav_path1, 3.0, 44100)?;
    create_test_wav(&wav_path2, 2.0, 44100)?;

    // Define args
    let args = AudioSplitArgs {
        input: input_dir.to_string_lossy().to_string(),
        output: output_dir.to_string_lossy().to_string(),
        chunk_duration: 1.0,
        delete_original: false,
    };

    // Execute command
    audio::split::execute(args)?;

    let files1: Vec<_> = fs::read_dir(&output_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .collect();
    let files2: Vec<_> = fs::read_dir(&output_dir.join("nested"))?
        .filter_map(|entry| entry.ok())
        .collect();

    assert_eq!(files1.len(), 3); // 3 second file split into 1 second chunks
    assert_eq!(files2.len(), 2); // 2 second file split into 1 second chunks

    // Check input files are present
    assert!(wav_path1.exists());
    assert!(wav_path2.exists());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_audio_split_file_with_delete_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test files
    let input_path = test_dir.join("input.wav");
    let output_dir = test_dir.join("output");
    create_test_wav(&input_path, 5.0, 44100)?;

    // Define args
    let args = AudioSplitArgs {
        input: input_path.to_string_lossy().to_string(),
        output: output_dir.to_string_lossy().to_string(),
        chunk_duration: 1.0,
        delete_original: true,
    };

    // Execute command
    audio::split::execute(args)?;

    // Verify output files exist and have correct chunk_duration
    let output_files: Vec<_> = fs::read_dir(&output_dir)?
        .filter_map(|entry| entry.ok())
        .collect();

    assert_eq!(output_files.len(), 5); // 5 second file split into 1 second chunks

    for file in output_files {
        let reader = WavReader::open(file.path())?;
        let chunk_duration = reader.duration() as f32 / reader.spec().sample_rate as f32;
        assert!((chunk_duration - 1.0).abs() < 0.1); // Allow small rounding differences
    }

    // Check file has not been cancelled
    assert!(!input_path.exists());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_audio_split_directory_with_delete_success() -> Result<()> {
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

    create_test_wav(&wav_path1, 3.0, 44100)?;
    create_test_wav(&wav_path2, 2.0, 44100)?;

    // Define args
    let args = AudioSplitArgs {
        input: input_dir.to_string_lossy().to_string(),
        output: output_dir.to_string_lossy().to_string(),
        chunk_duration: 1.0,
        delete_original: true,
    };

    // Execute command
    audio::split::execute(args)?;

    let files1: Vec<_> = fs::read_dir(&output_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .collect();
    let files2: Vec<_> = fs::read_dir(&output_dir.join("nested"))?
        .filter_map(|entry| entry.ok())
        .collect();

    assert_eq!(files1.len(), 3); // 3 second file split into 1 second chunks
    assert_eq!(files2.len(), 2); // 2 second file split into 1 second chunks

    // Check input files are present
    assert!(!wav_path1.exists());
    assert!(!wav_path2.exists());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}
