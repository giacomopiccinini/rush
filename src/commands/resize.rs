use image::io::Reader as ImageReader;
use image::imageops::resize;
use image::imageops::FilterType;

use crate::ResizeArgs;

// Execute the resize command
pub fn execute(args: ResizeArgs) {

    // Read image
    let input_img = ImageReader::open(&args.target).unwrap().decode().unwrap();

    // Resize image
    let output_img = resize(&input_img, args.width, args.height, FilterType::Lanczos3);

    // Save image
    output_img.save(&args.output).unwrap();

}

