use image::{Rgb, RgbImage, ImageError};
use std::fs::File;
use std::io::{BufReader};
use crate::validators::is_valid_file_extension;
use crate::validators::is_valid_ppm_type;
use crate::ppm_loader::ppm_header_loader;
use crate::ppm_loader::ppm_data_loader;

pub fn create_image_buffer(width: u32, height: u32, pixel_data: &Vec<u8>) -> RgbImage {
    let mut image_buffer = RgbImage::new(width, height);
    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        let i = (y * width + x) as usize;
        *pixel = Rgb([pixel_data[i * 3], pixel_data[i * 3 + 1], pixel_data[i * 3 + 2]]);
    }

    image_buffer
}

pub fn save_image(image_data: &RgbImage, file_name: &str) -> Result<(), ImageError> {
    image_data.save(file_name)?;
    Ok(())
}

pub fn load_and_convert(input: &str, output: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let file = File::open(input)?;
    let mut reader = BufReader::new(file);

    let input_extensions = &["ppm"];

    if !is_valid_file_extension(input, input_extensions) {
        return Err("The input file does not have a .ppm extension". into());
    }

    let image_extensions = &["jpg", "jpeg", "png"];

    if !(is_valid_file_extension(output, image_extensions)) {
        return Err("Export file must be of type .jpg or .png".into());
    }

    match is_valid_ppm_type(&mut reader) {
        Ok(is_ppm) => {
            if !is_ppm {
                return Err("ERROR: file is not a valid ppm file".into());
            }
        }
        Err(err) => {
            return Err(format!("ERROR: failed to check if the file is a ppm file: {}", err).into());
        }
    }

    let header = ppm_header_loader(&mut reader)?;
    let image_data = ppm_data_loader(&mut reader)?;
    let image_buffer = create_image_buffer(header.width, header.height, &image_data);

    save_image(&image_buffer, output).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    
    Ok(true)
}