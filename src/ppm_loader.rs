use std::fs::File;
use std::io::{BufReader, BufRead};
use std::num::ParseIntError;

pub struct Header {
    pub width: u32,
    pub height: u32,
    pub _max_color_value: u32,
} 

pub fn ppm_header_loader(reader: &mut BufReader<File>) -> Result<Header, Box<dyn std::error::Error>> {
    let mut line = String::new();
    reader.read_line(&mut line)?;

    let dimensions: Vec<u32> = line
        .split_whitespace()
        .map(|s| s.parse().map_err(|e: ParseIntError| e.to_string()))
        .collect::<Result<_, _>>()?;
    
    if dimensions.len() < 2 {
        return Err("Not enough dimensions provided".into());
    }

    line.clear();
    
    reader.read_line(&mut line)?;
    let max_color_value = line.trim().parse().map_err(|e: ParseIntError| e.to_string())?;

    Ok(Header { 
        width: dimensions[0], 
        height: dimensions[1], 
        _max_color_value: max_color_value
    })
}

pub fn ppm_data_loader(reader: &mut BufReader<File>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut pixels = Vec::new();
    for line in reader.lines() {
        let line = line?;
        for value in line.split_whitespace() {
            pixels.push(value.parse::<u8>().map_err(|e| e.to_string())?);
        }
    }

    Ok(pixels)
}

