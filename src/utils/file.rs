use std::{
    fs::{self, File},
    io::{self, BufReader, BufWriter, Error},
};

use bitvec::vec::BitVec;

pub fn compression_rate(file: &str, compressed_file: &str) -> Result<f64, Error> {
    let file_size = (fs::metadata(file)?).len() as f64;
    let compressed_file_size = (fs::metadata(compressed_file)?).len() as f64;

    Ok((1.0 - (compressed_file_size / file_size)) * 100.0)
}

pub fn initialize_writer(filename: &str) -> io::Result<BufWriter<File>> {
    let file = File::create(filename)?;
    Ok(BufWriter::new(file))
}

pub fn initialize_reader(filename: &str) -> io::Result<BufReader<File>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file))
}

pub fn drain_byte(bitvec: &mut BitVec) -> BitVec<u8> {
    bitvec.drain(..8).rev().collect::<BitVec<u8>>()
}
