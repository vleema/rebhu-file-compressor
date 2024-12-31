use std::{
    fs::{self, File},
    io::{BufReader, Error},
};

pub fn open_file(filename: &str) -> Result<BufReader<File>, Error> {
    let file = File::open(filename);
    match file {
        Ok(file) => Ok(BufReader::new(file)),
        Err(e) => Err(Error::new(
            e.kind(),
            format!("Unable to open file '{}', cause: {}", filename, e),
        )),
    }
}

pub fn compression_rate(file: &str, compressed_file: &str) -> Result<f64, Error> {
    let file_size = (fs::metadata(file)?).len() as f64;
    let compressed_file_size = (fs::metadata(compressed_file)?).len() as f64;

    Ok((1.0 - (compressed_file_size / file_size)) * 100.0)
}
