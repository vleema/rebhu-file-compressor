use bitvec::vec::BitVec;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufWriter, Error, Seek, SeekFrom, Write},
};

use super::{
    huff::{build_table, build_tree, FrequencyList},
    HuffTable,
};
use crate::utils::GenericError;

pub fn huff_compress(filename: &str, result_filename: &str) -> Result<(), GenericError> {
    let (freq_list, contents) = read_file(filename)?;
    let tree = build_tree(&freq_list)?;
    let huff_table = build_table(&tree);
    let file = File::create(result_filename)?;
    let mut writer = BufWriter::new(file);
    write_header(freq_list, &mut writer)?;
    let padding_bits = write_encoded_content(huff_table, contents, &mut writer)?;
    write_padding_bits_into_header(padding_bits, &mut writer)?;
    Ok(())
}

fn read_file(filename: &str) -> Result<(FrequencyList, Vec<u8>), Error> {
    let buffer: Vec<u8> = fs::read(filename)?;
    let mut freq_map = HashMap::new();
    count_byte_frequency(&buffer, &mut freq_map);
    Ok((FrequencyList::from_hashmap(&freq_map), buffer))
}

fn write_header(
    freq_list: FrequencyList,
    writer: &mut BufWriter<File>,
) -> Result<(), GenericError> {
    let serialized_header = bincode::serialize(&freq_list)?;
    writer.write_all(&[0])?; // First byte reserved to padding length
    writer.write_all(&(serialized_header.len() as u32).to_le_bytes())?;
    writer.write_all(serialized_header.as_slice())?;
    Ok(())
}

fn write_encoded_content(
    huff_table: HuffTable,
    content: Vec<u8>,
    writer: &mut BufWriter<File>,
) -> Result<u8, GenericError> {
    let mut bit_buffer: BitVec = BitVec::new();
    let mut padding_bits = 0;
    for byte in content {
        let byte_code = huff_table
            .get(&byte)
            .ok_or("Unexpected error while extracting code from huffman table")?;
        bit_buffer.extend(byte_code);
        while bit_buffer.len() >= 8 {
            let byte = bit_buffer.drain(..8).rev().collect::<BitVec<u8>>();
            writer.write_all(byte.as_raw_slice())?;
        }
    }
    if !bit_buffer.is_empty() {
        padding_bits = 8 - bit_buffer.len();
        for _ in 0..padding_bits {
            bit_buffer.push(false);
        }
        let byte = bit_buffer.drain(..8).rev().collect::<BitVec<u8>>();
        writer.write_all(byte.as_raw_slice())?;
    }
    writer.flush()?;
    Ok(padding_bits as u8)
}

fn write_padding_bits_into_header(padding: u8, writer: &mut BufWriter<File>) -> Result<(), Error> {
    writer.get_ref().seek(SeekFrom::Start(0))?;
    writer.write_all(&[padding])?;
    Ok(())
}

fn count_byte_frequency(data: &Vec<u8>, freq: &mut HashMap<u8, u32>) {
    for byte in data {
        *freq.entry(*byte).or_insert(0) += 1;
    }
}
