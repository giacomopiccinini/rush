use crate::utils::{cleanup_test_dir, create_test_wav, setup_test_dir};
use anyhow::Result;
use rush::commands::audio;
use rush::AudioSpectrogramArgs;
use std::fs;

#[test]
fn test_audio_spectrogram_file_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test files
    let input_path = test_dir.join("input.wav");
    let output_path = test_dir.join("output.png");
    create_test_wav(&input_path, 2.0, 22050, 1, 16)?;

    // Define args with default parameters
    let args = AudioSpectrogramArgs {
        input: input_path.to_string_lossy().to_string(),
        output: output_path.to_string_lossy().to_string(),
        n_fft: 2048,
        hop_length: 512,
        win_length: 2048,
        n_mels: 128,
        f_min: 20.0,
        f_max: 0.0,
        sr: 22050,
        delete_original: false,
    };

    // Execute command
    audio::spectrogram::execute(args)?;

    // Verify output file exists and is a valid PNG
    assert!(output_path.exists());
    let img = image::open(&output_path)?;
    assert!(img.width() > 0);
    assert!(img.height() > 0);

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_audio_spectrogram_directory_success() -> Result<()> {
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

    create_test_wav(&wav_path1, 2.0, 22050, 1, 16)?;
    create_test_wav(&wav_path2, 2.0, 22050, 1, 16)?;

    // Define args
    let args = AudioSpectrogramArgs {
        input: input_dir.to_string_lossy().to_string(),
        output: output_dir.to_string_lossy().to_string(),
        n_fft: 2048,
        hop_length: 512,
        win_length: 2048,
        n_mels: 128,
        f_min: 20.0,
        f_max: 0.0,
        sr: 22050,
        delete_original: false,
    };

    // Execute command
    audio::spectrogram::execute(args)?;

    // Verify output files exist
    let output_path1 = output_dir.join("test1.png");
    let output_path2 = output_dir.join("nested/test2.png");

    assert!(output_path1.exists());
    assert!(output_path2.exists());

    // Verify they are valid images
    let img1 = image::open(&output_path1)?;
    let img2 = image::open(&output_path2)?;
    assert!(img1.width() > 0);
    assert!(img2.width() > 0);

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_audio_spectrogram_delete_original_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;
    let input_dir = test_dir.join("input");
    let output_dir = test_dir.join("output");
    fs::create_dir(&input_dir)?;

    // Create test file
    let input_path = input_dir.join("test.wav");
    create_test_wav(&input_path, 2.0, 22050, 1, 16)?;

    // Define args with delete_original flag
    let args = AudioSpectrogramArgs {
        input: input_dir.to_string_lossy().to_string(),
        output: output_dir.to_string_lossy().to_string(),
        n_fft: 2048,
        hop_length: 512,
        win_length: 2048,
        n_mels: 128,
        f_min: 20.0,
        f_max: 0.0,
        sr: 22050,
        delete_original: true,
    };

    // Execute command
    audio::spectrogram::execute(args)?;

    // Verify output exists and original was deleted
    let output_path = output_dir.join("test.png");
    assert!(output_path.exists());
    assert!(!input_path.exists());

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_audio_spectrogram_custom_params_success() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test files
    let input_path = test_dir.join("input.wav");
    let output_path = test_dir.join("output.png");
    create_test_wav(&input_path, 2.0, 44100, 1, 16)?;

    // Define args with custom parameters
    let args = AudioSpectrogramArgs {
        input: input_path.to_string_lossy().to_string(),
        output: output_path.to_string_lossy().to_string(),
        n_fft: 1024,
        hop_length: 256,
        win_length: 1024,
        n_mels: 64,
        f_min: 50.0,
        f_max: 8000.0,
        sr: 16000,
        delete_original: false,
    };

    // Execute command
    audio::spectrogram::execute(args)?;

    // Verify output file exists
    assert!(output_path.exists());
    let img = image::open(&output_path)?;
    assert!(img.width() > 0);
    assert!(img.height() > 0);

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_audio_spectrogram_no_mel_conversion() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test files
    let input_path = test_dir.join("input.wav");
    let output_path = test_dir.join("output.png");
    create_test_wav(&input_path, 2.0, 22050, 1, 16)?;

    // Define args without mel conversion (n_mels = 0)
    let args = AudioSpectrogramArgs {
        input: input_path.to_string_lossy().to_string(),
        output: output_path.to_string_lossy().to_string(),
        n_fft: 2048,
        hop_length: 512,
        win_length: 2048,
        n_mels: 0, // No mel conversion
        f_min: 20.0,
        f_max: 0.0,
        sr: 22050,
        delete_original: false,
    };

    // Execute command
    audio::spectrogram::execute(args)?;

    // Verify output file exists
    assert!(output_path.exists());
    let img = image::open(&output_path)?;
    assert!(img.width() > 0);
    assert!(img.height() > 0);

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_audio_spectrogram_output_to_directory() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;
    let output_dir = test_dir.join("output");
    fs::create_dir(&output_dir)?;

    // Create test file
    let input_path = test_dir.join("input.wav");
    create_test_wav(&input_path, 2.0, 22050, 1, 16)?;

    // Define args with output as directory
    let args = AudioSpectrogramArgs {
        input: input_path.to_string_lossy().to_string(),
        output: output_dir.to_string_lossy().to_string(),
        n_fft: 2048,
        hop_length: 512,
        win_length: 2048,
        n_mels: 128,
        f_min: 20.0,
        f_max: 0.0,
        sr: 22050,
        delete_original: false,
    };

    // Execute command
    audio::spectrogram::execute(args)?;

    // Verify output file exists with .png extension
    let output_path = output_dir.join("input.wav.png");
    assert!(output_path.exists());
    let img = image::open(&output_path)?;
    assert!(img.width() > 0);
    assert!(img.height() > 0);

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_audio_spectrogram_stereo_input() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test files with stereo audio
    let input_path = test_dir.join("input.wav");
    let output_path = test_dir.join("output.png");
    create_test_wav(&input_path, 2.0, 22050, 2, 16)?;

    // Define args
    let args = AudioSpectrogramArgs {
        input: input_path.to_string_lossy().to_string(),
        output: output_path.to_string_lossy().to_string(),
        n_fft: 2048,
        hop_length: 512,
        win_length: 2048,
        n_mels: 128,
        f_min: 20.0,
        f_max: 0.0,
        sr: 22050,
        delete_original: false,
    };

    // Execute command
    audio::spectrogram::execute(args)?;

    // Verify output file exists
    assert!(output_path.exists());
    let img = image::open(&output_path)?;
    assert!(img.width() > 0);
    assert!(img.height() > 0);

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}

#[test]
fn test_audio_spectrogram_high_sample_rate() -> Result<()> {
    // Set up the directory for testing
    let test_dir = setup_test_dir()?;

    // Create test files with high sample rate
    let input_path = test_dir.join("input.wav");
    let output_path = test_dir.join("output.png");
    create_test_wav(&input_path, 2.0, 48000, 1, 16)?;

    // Define args
    let args = AudioSpectrogramArgs {
        input: input_path.to_string_lossy().to_string(),
        output: output_path.to_string_lossy().to_string(),
        n_fft: 2048,
        hop_length: 512,
        win_length: 2048,
        n_mels: 128,
        f_min: 20.0,
        f_max: 16000.0,
        sr: 48000,
        delete_original: false,
    };

    // Execute command
    audio::spectrogram::execute(args)?;

    // Verify output file exists
    assert!(output_path.exists());
    let img = image::open(&output_path)?;
    assert!(img.width() > 0);
    assert!(img.height() > 0);

    // Clean up dir
    cleanup_test_dir(&test_dir)?;

    Ok(())
}
