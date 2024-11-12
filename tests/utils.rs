use anyhow::Result;
use polars::df;
use polars::prelude::*;
use std::fs;
use std::fs::File;
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

/// Create sample wav file
pub fn create_test_wav(
    path: &Path,
    duration_sec: f32,
    sample_rate: u32,
    channels: usize,
    bits_per_sample: u16,
) -> Result<()> {
    use hound::{WavSpec, WavWriter};

    let spec = WavSpec {
        channels: channels as u16,
        sample_rate,
        bits_per_sample,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = WavWriter::create(path, spec)?;
    let num_samples = (duration_sec * sample_rate as f32) as u32;

    for t in 0..num_samples {
        let sample = (t as f32 * 440.0 * 2.0 * std::f32::consts::PI / sample_rate as f32).sin();

        // Write sample for each channel
        for _ in 0..channels {
            match bits_per_sample {
                8 => writer.write_sample((sample * i8::MAX as f32) as i8)?,
                16 => writer.write_sample((sample * i16::MAX as f32) as i16)?,
                32 => writer.write_sample((sample * i32::MAX as f32) as i32)?,
                _ => {
                    return Err(anyhow::anyhow!(
                        "Unsupported bits per sample: {}",
                        bits_per_sample
                    ))
                }
            }
        }
    }
    Ok(())
}

/// Create a test image with specified dimensions and channels
pub fn create_test_image(path: &Path, width: u32, height: u32, channels: u8) -> Result<()> {
    use image::{ImageBuffer, Luma, Rgb};

    if channels != 1 && channels != 3 {
        return Err(anyhow::anyhow!("Number of channels must be 1 or 3"));
    }

    if channels == 1 {
        // Create a grayscale image
        let img: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::from_fn(width, height, |x, y| {
            // Create a simple pattern
            let value = ((x as f32 / width as f32 + y as f32 / height as f32) * 255.0) as u8;
            Luma([value])
        });
        img.save(path)?;
    } else {
        // Create an RGB image
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(width, height, |x, y| {
            // Create a simple RGB pattern
            let r = (x as f32 / width as f32 * 255.0) as u8;
            let g = (y as f32 / height as f32 * 255.0) as u8;
            let b = ((x as f32 + y as f32) / (width as f32 + height as f32) * 255.0) as u8;
            Rgb([r, g, b])
        });
        img.save(path)?;
    }

    Ok(())
}

/// Create a test video file with specified dimensions, duration and framerate
pub fn create_test_video(
    path: &Path,
    width: u32,
    height: u32,
    duration_sec: f32,
    fps: u32,
) -> Result<()> {
    use std::process::Command;

    // First create a test image that will be used for the video
    let temp_image = path.with_extension("png");
    create_test_image(&temp_image, width, height, 3)?;

    // Use ffmpeg to create video from the image
    let output = Command::new("ffmpeg")
        .args([
            "-y", // Overwrite output file if it exists
            "-loop",
            "1", // Loop the input
            "-i",
            temp_image.to_str().unwrap(),
            "-c:v",
            "libx264",
            "-t",
            &duration_sec.to_string(),
            "-pix_fmt",
            "yuv420p",
            "-r",
            &fps.to_string(),
            "-vf",
            &format!("scale={}:{}", width, height),
            path.to_str().unwrap(),
        ])
        .output()?;

    // Check if ffmpeg command was successful
    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "FFmpeg failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    // Clean up temporary image
    std::fs::remove_file(temp_image)?;

    Ok(())
}

/// Create table
pub fn create_test_table(path: &Path) -> Result<()> {
    // Use macro
    let mut df = df! [
        "names" => ["a", "b", "c"],
        "values" => [1, 2, 3],
    ]?;

    // Find extension
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    // Create a file
    let mut file = File::create(path)?;

    match ext {
        "csv" => {
            CsvWriter::new(&mut file)
                .include_header(true)
                .with_separator(b',')
                .finish(&mut df)?;
        }
        "parquet" => {
            ParquetWriter::new(file).finish(&mut df)?;
        }
        _ => return Err(anyhow::anyhow!("Unsupported file extension: {}", ext)),
    }

    Ok(())
}
