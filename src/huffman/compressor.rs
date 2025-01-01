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
use crate::utils::{
    file::{drain_byte, initialize_writer},
    GenericError,
};

pub fn huff_compress(filename: &str, result_filename: &str) -> Result<(), GenericError> {
    let (freq_list, contents) = read_file(filename)?;
    let table = build_compression_table(&freq_list)?;

    let mut writer = initialize_writer(result_filename)?;

    write_header(&freq_list, &mut writer)?;
    let padding = write_encoded_content(table, contents, &mut writer)?;

    write_padding_bits_into_header(padding, &mut writer)?;
    Ok(())
}

fn build_compression_table(freq_list: &FrequencyList) -> Result<HuffTable, GenericError> {
    Ok(build_table(&build_tree(freq_list)?))
}

fn read_file(filename: &str) -> Result<(FrequencyList, Vec<u8>), Error> {
    let buffer: Vec<u8> = fs::read(filename)?;
    let mut freq_map = HashMap::new();
    count_byte_frequency(&buffer, &mut freq_map);
    Ok((FrequencyList::from_hashmap(&freq_map), buffer))
}

fn write_header(
    freq_list: &FrequencyList,
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
    let mut buffer: BitVec = BitVec::new();
    write_complete_bytes(&mut buffer, &huff_table, &content, writer)?;
    write_last_byte(&mut buffer, writer)
}

fn write_complete_bytes(
    buffer: &mut BitVec,
    huff_table: &HuffTable,
    content: &Vec<u8>,
    writer: &mut BufWriter<File>,
) -> Result<(), GenericError> {
    for byte in content {
        let byte_code = huff_table
            .get(byte)
            .ok_or("Unexpected error while extracting code from huffman table")?;
        buffer.extend(byte_code);
        while buffer.len() >= 8 {
            let byte = drain_byte(buffer);
            writer.write_all(byte.as_raw_slice())?;
        }
    }
    Ok(())
}

fn write_last_byte(buffer: &mut BitVec, writer: &mut BufWriter<File>) -> Result<u8, GenericError> {
    let mut padding = 0;
    if !buffer.is_empty() {
        padding = 8 - buffer.len();
        for _ in 0..padding {
            buffer.push(false);
        }
        let byte = drain_byte(buffer);
        writer.write_all(byte.as_raw_slice())?;
    }
    writer.flush()?;
    Ok(padding as u8)
}

fn write_padding_bits_into_header(padding: u8, writer: &mut BufWriter<File>) -> Result<(), Error> {
    writer.seek(SeekFrom::Start(0))?;
    writer.write_all(&[padding])
}

fn count_byte_frequency(data: &Vec<u8>, freq: &mut HashMap<u8, u32>) {
    for byte in data {
        *freq.entry(*byte).or_insert(0) += 1;
    }
}
