use bitvec::vec::BitVec;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufWriter, Error, Write},
};

use super::huffman::{build_tree, Header};
use crate::utils::file::open_file;

type GenericError = Box<dyn std::error::Error>;

pub fn huff_compress(filename: &str, result_filename: &str) -> Result<(), GenericError> {
    let (freq, contents) = read_file(filename)?;
    let tree = build_tree(&freq)?;
    let huff_table = tree.pre_order_code(BitVec::new());

    let file = File::create(result_filename)?;
    let mut writer = BufWriter::new(file);
    write_header(freq, &mut writer)?;
    write_encoded_content(huff_table, contents, &mut writer)
}

fn read_file(filename: &str) -> Result<(HashMap<char, u32>, String), Error> {
    let file = open_file(filename)?;
    let mut freq = HashMap::new();
    let mut contents = String::new();
    for line in file.lines() {
        match line {
            Err(e) => {
                return Err(Error::new(
                    e.kind(),
                    format!("Error while reading a line from file '{}': {}", filename, e),
                ))
            }
            Ok(chars) => {
                count_char_frequency(&chars, &mut freq);
                contents.push_str(&chars);
            }
        }
    }
    Ok((freq, contents))
}

fn write_header(
    freq: HashMap<char, u32>,
    writer: &mut BufWriter<File>,
) -> Result<(), GenericError> {
    let header = Header {
        frequency_list: freq,
    };
    let serialized_header = bincode::serialize(&header)?;
    writer.write_all(&(serialized_header.len() as u32).to_le_bytes())?;
    writer.write_all(serialized_header.as_slice())?;
    Ok(())
}

fn write_encoded_content(
    huff_table: HashMap<char, BitVec>,
    contents: String,
    writer: &mut BufWriter<File>,
) -> Result<(), GenericError> {
    let mut bit_buffer: BitVec = BitVec::new();
    for ch in contents.chars() {
        let ch_code = huff_table
            .get(&ch)
            .ok_or("Unexpected error while extracting code from huffman table")?;
        bit_buffer.extend(ch_code);
        while bit_buffer.len() >= 8 {
            let byte = bit_buffer.drain(..8).collect::<BitVec<u8>>();
            writer.write_all(byte.as_raw_slice())?;
        }
    }
    if !bit_buffer.is_empty() {
        let remaining_bits: BitVec = BitVec::repeat(false, 8 - bit_buffer.len());
        bit_buffer.extend(remaining_bits);
        writer.write_all(bit_buffer.drain(..8).collect::<BitVec<u8>>().as_raw_slice())?;
    }
    Ok(())
}

fn count_char_frequency(text: &str, freq: &mut HashMap<char, u32>) {
    for ch in text.chars() {
        *freq.entry(ch).or_insert(0) += 1;
    }
}
