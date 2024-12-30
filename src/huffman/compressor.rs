use bitvec::vec::BitVec;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufWriter, Error, Write},
};

use super::{
    huff::{build_table, build_tree, FrequencyList},
    GenericError, HuffTable, PSEUDO_EOF,
};
use crate::utils::file::open_file;

pub fn huff_compress(filename: &str, result_filename: &str) -> Result<(), GenericError> {
    let (freq_list, contents) = read_file(filename)?;
    let tree = build_tree(&freq_list)?;
    let huff_table = build_table(&tree);
    let file = File::create(result_filename)?;
    let mut writer = BufWriter::new(file);
    write_header(freq_list, &mut writer)?;
    write_encoded_content(huff_table, contents, &mut writer)
}

fn read_file(filename: &str) -> Result<(FrequencyList, String), Error> {
    let mut file = open_file(filename)?;
    let mut freq_map = HashMap::new();
    let mut contents = String::new();
    let mut buffer = String::new();
    while let Ok(n) = file.read_line(&mut buffer) {
        if n == 0 {
            break;
        }
        count_char_frequency(&buffer, &mut freq_map);
        contents.push_str(&buffer);
        buffer.clear();
    }
    freq_map.insert(PSEUDO_EOF, 1);
    Ok((FrequencyList::from_hashmap(&freq_map), contents))
}

fn write_header(
    freq_list: FrequencyList,
    writer: &mut BufWriter<File>,
) -> Result<(), GenericError> {
    let serialized_header = bincode::serialize(&freq_list)?;
    writer.write_all(&(serialized_header.len() as u32).to_le_bytes())?;
    writer.write_all(serialized_header.as_slice())?;
    Ok(())
}

fn write_encoded_content(
    huff_table: HuffTable,
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
        let eof_code = huff_table
            .get(&PSEUDO_EOF)
            .ok_or("EOF character was not in huffman table")?;
        bit_buffer.extend(eof_code);
        let remaining_bits: BitVec = if bit_buffer.len() < 8 {
            BitVec::repeat(false, 8 - bit_buffer.len())
        } else {
            BitVec::new()
        };
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
