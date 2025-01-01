use std::{
    fs::{self},
    io::Error,
};

pub fn compression_rate(file: &str, compressed_file: &str) -> Result<f64, Error> {
    let file_size = (fs::metadata(file)?).len() as f64;
    let compressed_file_size = (fs::metadata(compressed_file)?).len() as f64;

    Ok((1.0 - (compressed_file_size / file_size)) * 100.0)
}
