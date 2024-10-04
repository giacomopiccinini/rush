use opencv::{
    prelude::*,
    videoio,
    imgcodecs,
    core,
    Result,
};

pub fn execute(args: ExtractFramesArgs) {

    // Convert to path
    let target = PathBuf::from(&args.target);
    let output = PathBuf::from(&args.output);

    // Sanity checks
    if !target.exists() {
        panic!("Target file does not exist");
    }

    if !target.is_file() {
        panic!("Target is not a file");
    }

    if !output.exists() {
        panic!("Output directory does not exist");
    }

    if !output.is_dir() {
        panic!("Output is not a directory");
    }

    // Define allowed extensions for videos
    let video_extensions: HashSet<_> = ["mp4", "avi", "mov", "mkv", "webm", "ts"]
        .iter().map(|&s| s.to_lowercase()).collect();

    // Set up capture
    let mut cap = videoio::VideoCapture::from_file(&target, videoio::CAP_ANY)?;
    let mut frame = core::Mat::default();
    let mut frame_count = 0;
    let filename = target.file_stem().unwrap().to_str().unwrap();

    while cap.read(&mut frame)? {
        let frame_name = output.join(format!("frame_{:04}.jpg", frame_count));
        imgcodecs::imwrite(&frame_name, &frame, &core::Vector::new())?;
        frame_count += 1;
    }

    Ok(())
}