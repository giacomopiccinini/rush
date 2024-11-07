use crate::utils::{cleanup_test_dir, create_test_wav, setup_test_dir};
use anyhow::Result;
use rush::commands::audio;
use rush::AudioSummaryArgs;

#[test]
fn test_audio_summary_directory_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test files
    let wav_path = test_dir.join("test.wav");
    create_test_wav(&wav_path, 10.0, 44100)?;

    // Define args
    let args = AudioSummaryArgs {
        target: test_dir.to_string_lossy().to_string(),
    };

    // Execute command
    audio::summary::execute(args)?;

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_audio_summary_file_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test files
    let wav_path = test_dir.join("test.wav");
    create_test_wav(&wav_path, 10.0, 44100)?;

    // Define args
    let args = AudioSummaryArgs {
        target: wav_path.to_string_lossy().to_string(),
    };

    // Execute command
    audio::summary::execute(args)?;

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_audio_summary_empty_directory_error() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Run summary command
    let args = AudioSummaryArgs {
        target: test_dir.to_string_lossy().to_string(),
    };

    // Execute and expect an error
    let result = audio::summary::execute(args);

    // Assert that it's an error and optionally check the error message
    assert!(result.is_err());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_audio_summary_non_existing_directory_error() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;
    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    // Run summary command
    let args = AudioSummaryArgs {
        target: test_dir.to_string_lossy().to_string(),
    };

    // Execute and expect an error
    let result = audio::summary::execute(args);

    // Assert that it's an error and optionally check the error message
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_audio_summary_nested_directories_success() -> Result<()> {
    // Set up the root test directory
    let test_dir = setup_test_dir()?;

    // Create nested directory structure
    let subdir1 = test_dir.join("subdir1");
    let subdir2 = test_dir.join("subdir1/subdir2");
    let subdir3 = test_dir.join("subdir1/subdir2/subdir3");
    std::fs::create_dir_all(&subdir3)?;

    // Create test files in different directories
    let wav_path1 = test_dir.join("test1.wav");
    let wav_path2 = subdir1.join("test2.wav");
    let wav_path3 = subdir2.join("test3.wav");
    let wav_path4 = subdir3.join("test4.wav");

    // Create test WAV files with different durations
    create_test_wav(&wav_path1, 10.0, 44100)?;
    create_test_wav(&wav_path2, 15.0, 44100)?;
    create_test_wav(&wav_path3, 20.0, 44100)?;
    create_test_wav(&wav_path4, 25.0, 44100)?;

    // Define args to scan the root directory
    let args = AudioSummaryArgs {
        target: test_dir.to_string_lossy().to_string(),
    };

    // Execute command
    audio::summary::execute(args)?;

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}
