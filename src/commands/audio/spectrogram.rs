use anyhow::{Context, Result};
use rayon::prelude::*;
use spectrs::io::audio::{read_audio_file_mono, resample};
use spectrs::io::image::save_spectrogram_image;
use spectrs::spectrogram::mel::{convert_to_mel, MelScale};
use spectrs::spectrogram::stft::{compute_spectrogram, par_compute_spectrogram, SpectrogramType};
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::utils::{file_has_right_extension, perform_io_sanity_check};
use crate::AudioSpectrogramArgs;

// Admissible extensions for this command
const EXTENSIONS: [&str; 1] = ["wav"];

pub fn execute(args: AudioSpectrogramArgs) -> Result<()> {
    // Parse the arguments
    let input = Path::new(&args.input);
    let output = Path::new(&args.output);

    let n_fft: usize = args.n_fft;
    let hop_length: usize = args.hop_length;
    let win_length: usize = args.win_length;
    let n_mels: usize = args.n_mels;
    let f_min: f32 = args.f_min;
    let f_max: f32 = args.f_max;
    let sr: u32 = args.sr;
    let delete_original: bool = args.delete_original;

    // Sanity checks on I/O
    perform_io_sanity_check(input, output, false, true).with_context(|| "Sanity check failed")?;

    // Process files
    process(
        input,
        output,
        n_fft,
        hop_length,
        win_length,
        n_mels,
        f_min,
        f_max,
        sr,
        delete_original,
    )
    .with_context(|| "Processing failed")?;

    Ok(())
}

/// Process all the content (single file or directory of files)
#[allow(clippy::too_many_arguments)]
fn process(
    input: &Path,
    output: &Path,
    n_fft: usize,
    hop_length: usize,
    win_length: usize,
    n_mels: usize,
    f_min: f32,
    f_max: f32,
    sr: u32,
    delete_original: bool,
) -> Result<()> {
    // Case of single input file
    if input.is_file() {
        // Check if the file has the right extension and process it
        file_has_right_extension(input, &EXTENSIONS)?;
        process_file_parallel(
            input,
            output,
            n_fft,
            hop_length,
            win_length,
            n_mels,
            f_min,
            f_max,
            sr,
            delete_original,
        )
        .with_context(|| format!("Failed to process file: {:?}", input))?;
    }
    // Case of input being a directory
    else {
        // Find all files
        let files: Vec<PathBuf> = WalkDir::new(input)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| file_has_right_extension(e.path(), &EXTENSIONS).is_ok())
            .map(|e| e.path().to_path_buf())
            .collect();

        // Parallel loop over entries
        files.par_iter().try_for_each(|file| -> Result<()> {
            // Relative path wrt input directory
            let relative_path = file
                .strip_prefix(input)
                .with_context(|| format!("Failed to strip prefix from path: {:?}", file))?;

            // Change extension to .png
            let mut output_filename = relative_path.to_path_buf();
            output_filename.set_extension("png");

            // Nested output path
            let file_output = output.join(&output_filename);

            // Ensure the output directory exists
            if let Some(parent) = file_output.parent() {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create output directory: {:?}", parent))?;
            }

            // Process the file (non-parallelized version for directory processing)
            process_file_sequential(
                file,
                &file_output,
                n_fft,
                hop_length,
                win_length,
                n_mels,
                f_min,
                f_max,
                sr,
            )
            .with_context(|| format!("Failed to process file: {:?}", file))?;

            // Delete original if requested
            if delete_original {
                std::fs::remove_file(file)
                    .with_context(|| format!("Failed to delete original file: {:?}", file))?;
            }

            Ok(())
        })?;
    }
    Ok(())
}

/// Process a single file using parallelized spectrogram computation
#[allow(clippy::too_many_arguments)]
fn process_file_parallel(
    input: &Path,
    output: &Path,
    n_fft: usize,
    hop_length: usize,
    win_length: usize,
    n_mels: usize,
    f_min: f32,
    f_max: f32,
    sr: u32,
    delete_original: bool,
) -> Result<()> {
    // Determine the output path
    let output_path = if output.is_dir() {
        // If output is a directory, use the same filename with .png extension
        let mut filename = input
            .file_name()
            .with_context(|| "Failed to get filename")?
            .to_os_string();
        filename.push(".png");
        output.join(filename)
    } else {
        output.to_path_buf()
    };

    // Read audio file
    let (audio, original_sr) = read_audio_file_mono(
        input
            .to_str()
            .with_context(|| "Failed to convert path to string")?,
    )
    .with_context(|| "Failed to read audio file")?;

    // Resample if needed
    let audio_resampled = if sr > 0 && original_sr != sr {
        resample(audio, original_sr, sr).with_context(|| "Failed to resample audio")?
    } else {
        audio
    };

    let final_sr = if sr > 0 { sr } else { original_sr };

    // Compute spectrogram (parallelized version)
    let spec = par_compute_spectrogram(
        &audio_resampled,
        n_fft,
        hop_length,
        win_length,
        true, // center
        SpectrogramType::Power,
    );

    // Convert to mel spectrogram if requested
    let final_spec = if n_mels > 0 {
        let f_max_value = if f_max > 0.0 {
            Some(f_max)
        } else {
            Some((final_sr / 2) as f32)
        };
        convert_to_mel(
            &spec,
            final_sr,
            n_fft,
            n_mels,
            Some(f_min),
            f_max_value,
            MelScale::Slaney,
        )
    } else {
        spec
    };

    // Ensure the output directory exists
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create output directory: {:?}", parent))?;
    }

    // Save spectrogram as image
    save_spectrogram_image(
        &final_spec,
        output_path
            .to_str()
            .with_context(|| "Failed to convert path to string")?,
    )
    .with_context(|| "Failed to save spectrogram image")?;

    // Delete original if requested
    if delete_original {
        std::fs::remove_file(input)
            .with_context(|| format!("Failed to delete original file: {:?}", input))?;
    }

    Ok(())
}

/// Process a single file using non-parallelized spectrogram computation
#[allow(clippy::too_many_arguments)]
fn process_file_sequential(
    input: &Path,
    output: &Path,
    n_fft: usize,
    hop_length: usize,
    win_length: usize,
    n_mels: usize,
    f_min: f32,
    f_max: f32,
    sr: u32,
) -> Result<()> {
    // Read audio file
    let (audio, original_sr) = read_audio_file_mono(
        input
            .to_str()
            .with_context(|| "Failed to convert path to string")?,
    )
    .with_context(|| "Failed to read audio file")?;

    // Resample if needed
    let audio_resampled = if sr > 0 && original_sr != sr {
        resample(audio, original_sr, sr).with_context(|| "Failed to resample audio")?
    } else {
        audio
    };

    let final_sr = if sr > 0 { sr } else { original_sr };

    // Compute spectrogram (non-parallelized version)
    let spec = compute_spectrogram(
        &audio_resampled,
        n_fft,
        hop_length,
        win_length,
        true, // center
        SpectrogramType::Power,
    );

    // Convert to mel spectrogram if requested
    let final_spec = if n_mels > 0 {
        let f_max_value = if f_max > 0.0 {
            Some(f_max)
        } else {
            Some((final_sr / 2) as f32)
        };
        convert_to_mel(
            &spec,
            final_sr,
            n_fft,
            n_mels,
            Some(f_min),
            f_max_value,
            MelScale::Slaney,
        )
    } else {
        spec
    };

    // Save spectrogram as image
    save_spectrogram_image(
        &final_spec,
        output
            .to_str()
            .with_context(|| "Failed to convert path to string")?,
    )
    .with_context(|| "Failed to save spectrogram image")?;

    Ok(())
}
