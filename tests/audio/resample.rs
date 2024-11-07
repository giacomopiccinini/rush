use crate::utils::{cleanup_test_dir, create_test_wav, setup_test_dir};
use anyhow::Result;
use hound::WavReader;
use rush::commands::audio;
use rush::AudioResampleArgs;
use std::fs;

#[test]
fn test_audio_resample_file_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test files
    let input_path = test_dir.join("input.wav");
    let output_path = test_dir.join("output.wav");
    create_test_wav(&input_path, 5.0, 44100)?;

    // Define args with new sample rate
    let args = AudioResampleArgs {
        input: input_path.to_string_lossy().to_string(),
        output: output_path.to_string_lossy().to_string(),
        sr: 22050,
        overwrite: false,
    };

    // Execute command
    audio::resample::execute(args)?;

    // Verify output file exists and has correct sample rate
    assert!(output_path.exists());
    let reader = WavReader::open(output_path)?;
    assert_eq!(reader.spec().sample_rate, 22050);

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_audio_resample_directory_success() -> Result<()> {
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

    create_test_wav(&wav_path1, 5.0, 44100)?;
    create_test_wav(&wav_path2, 5.0, 44100)?;

    // Define args
    let args = AudioResampleArgs {
        input: input_dir.to_string_lossy().to_string(),
        output: output_dir.to_string_lossy().to_string(),
        sr: 22050,
        overwrite: false,
    };

    // Execute command
    audio::resample::execute(args)?;

    // Verify output files exist and have correct sample rate
    let output_path1 = output_dir.join("test1.wav");
    let output_path2 = output_dir.join("nested/test2.wav");

    assert!(output_path1.exists());
    assert!(output_path2.exists());

    let reader1 = WavReader::open(output_path1)?;
    let reader2 = WavReader::open(output_path2)?;
    assert_eq!(reader1.spec().sample_rate, 22050);
    assert_eq!(reader2.spec().sample_rate, 22050);

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_audio_resample_same_rate_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test files
    let input_path = test_dir.join("input.wav");
    let output_path = test_dir.join("output.wav");
    create_test_wav(&input_path, 5.0, 44100)?;

    // Define args with same sample rate
    let args = AudioResampleArgs {
        input: input_path.to_string_lossy().to_string(),
        output: output_path.to_string_lossy().to_string(),
        sr: 44100,
        overwrite: false,
    };

    // Execute command
    audio::resample::execute(args)?;

    // Verify output file exists and has same sample rate
    assert!(output_path.exists());
    let reader = WavReader::open(output_path)?;
    assert_eq!(reader.spec().sample_rate, 44100);

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_audio_resample_overwrite_protection_error() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test file
    let input_path = test_dir.join("input.wav");
    create_test_wav(&input_path, 5.0, 44100)?;

    // Try to overwrite input file without overwrite flag
    let args = AudioResampleArgs {
        input: input_path.to_string_lossy().to_string(),
        output: input_path.to_string_lossy().to_string(),
        sr: 22050,
        overwrite: false,
    };

    // Execute command and expect error
    let result = audio::resample::execute(args);
    assert!(result.is_err());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_audio_resample_overwrite_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test file
    let input_path = test_dir.join("input.wav");
    create_test_wav(&input_path, 5.0, 44100)?;

    // Try to overwrite input file with overwrite flag
    let args = AudioResampleArgs {
        input: input_path.to_string_lossy().to_string(),
        output: input_path.to_string_lossy().to_string(),
        sr: 22050,
        overwrite: true,
    };

    // Execute command
    audio::resample::execute(args)?;

    // Verify file exists and has new sample rate
    assert!(input_path.exists());
    let reader = WavReader::open(&input_path)?;
    assert_eq!(reader.spec().sample_rate, 22050);

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_audio_resample_very_high_rate_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test files
    let input_path = test_dir.join("input.wav");
    let output_path = test_dir.join("output.wav");
    create_test_wav(&input_path, 5.0, 384_000)?;

    // Define args with higher sample rate
    let args = AudioResampleArgs {
        input: input_path.to_string_lossy().to_string(),
        output: output_path.to_string_lossy().to_string(),
        sr: 44100,
        overwrite: false,
    };

    // Execute command
    audio::resample::execute(args)?;

    // Verify output file exists and has higher sample rate
    assert!(output_path.exists());
    let reader = WavReader::open(output_path)?;
    assert_eq!(reader.spec().sample_rate, 44100);

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_audio_resample_higher_rate() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test files
    let input_path = test_dir.join("input.wav");
    let output_path = test_dir.join("output.wav");
    create_test_wav(&input_path, 5.0, 22050)?;

    // Define args with higher sample rate
    let args = AudioResampleArgs {
        input: input_path.to_string_lossy().to_string(),
        output: output_path.to_string_lossy().to_string(),
        sr: 44100,
        overwrite: false,
    };

    // Execute command
    audio::resample::execute(args)?;

    // Verify output file exists and has higher sample rate
    assert!(output_path.exists());
    let reader = WavReader::open(output_path)?;
    assert_eq!(reader.spec().sample_rate, 44100);

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}
