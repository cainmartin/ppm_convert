use std::path::Path;
use std::fs::File;
use std::io::{ BufReader, BufRead };

pub fn is_valid_file_extension(filename: &str, valid_extensions: &[&str]) -> bool {
    let path = Path::new(filename);
    if let Some(extension) = path.extension() {
        let ext = extension.to_str().unwrap_or("").to_lowercase();
        valid_extensions.iter().any(|&valid_ext| ext == valid_ext.to_lowercase())
    } else {
        false
    }
}

// TODO: Need to make this more generic and add support for the P6 binary ppm type
pub fn is_valid_ppm_type(reader: &mut BufReader<File>) -> Result<bool, Box<dyn std::error::Error>> {
    let mut header = String::new();
    reader.read_line(&mut header)?;
    Ok(header.starts_with("P3"))
}