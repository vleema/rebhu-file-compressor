use std::{
    fs::{self, File},
    io::{BufReader, Error},
};

pub fn compression_rate(file: &str, compressed_file: &str) -> Result<f64, Error> {
    let file_size = (fs::metadata(file)?).len() as f64;
    let compressed_file_size = (fs::metadata(compressed_file)?).len() as f64;

    println!("{:?}", compressed_file_size);
    println!("{:?}", file_size);
    println!("{:?}", compressed_file_size / file_size);
    println!("{:?}", (1.0 - compressed_file_size / file_size));

    Ok((1.0 - (compressed_file_size / file_size)) * 100.0)
}
