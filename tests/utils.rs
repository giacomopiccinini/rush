use anyhow::Result;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use uuid::Uuid;

/// Creates a fresh test directory for running tests
pub fn setup_test_dir() -> Result<PathBuf> {
    // Create a unique directory name by concatenating strings
    let dir_name = format!("test-data-{}", Uuid::new_v4());
    let test_dir = PathBuf::from(dir_name);
    
    if test_dir.exists() {
        fs::remove_dir_all(&test_dir)?;
    }
    fs::create_dir(&test_dir)?;
    Ok(test_dir)
}

/// Cleans up the test directory after tests are complete
pub fn cleanup_test_dir(test_dir: &Path) -> Result<()> {
    if test_dir.exists() {
        fs::remove_dir_all(test_dir)?;
    }
    Ok(())
}

pub fn create_test_wav(path: &Path, duration_sec: f32, sample_rate: u32) -> Result<()> {
    use hound::{WavSpec, WavWriter};

    let spec = WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = WavWriter::create(path, spec)?;
    let num_samples = (duration_sec * sample_rate as f32) as u32;

    for t in 0..num_samples {
        let sample = (t as f32 * 440.0 * 2.0 * std::f32::consts::PI / sample_rate as f32).sin();
        writer.write_sample((sample * i16::MAX as f32) as i16)?;
    }
    Ok(())
}
