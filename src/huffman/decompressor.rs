use std::{
    fs::File,
    io::{self, BufReader, Read, Write},
};

use bitvec::vec::BitVec;

use crate::utils::{
    file::{initialize_reader, initialize_writer},
    GenericError,
};

use super::{
    huff::{build_inverse_table, build_tree, FrequencyList},
    InverseHuffTable,
};

pub fn huff_decompress(filename: &str, result_filename: &str) -> Result<(), GenericError> {
    let mut reader = initialize_reader(filename)?;

    let (freq_list, padding_size) = read_header(&mut reader)?;
    let inverse_table = build_inverse_compression_table(&freq_list)?;

    decode_into_file(padding_size, &inverse_table, &mut reader, result_filename)?;
    Ok(())
}

fn read_header(reader: &mut BufReader<File>) -> io::Result<(FrequencyList, u8)> {
    let padding_size: u8 = read_padding_size(reader)?;
    let header_size: u32 = read_header_size(reader)?;
    let mut buffer: Vec<u8> = vec![0u8; header_size as usize];
    reader.read_exact(&mut buffer)?;
    let header: FrequencyList =
        bincode::deserialize(&buffer).expect("Failed to deserialize huffman frequency list");
    Ok((header, padding_size))
}

fn decode_into_file(
    padding_size: u8,
    huff_table: &InverseHuffTable,
    reader: &mut BufReader<File>,
    result_filename: &str,
) -> io::Result<()> {
    let mut acc: BitVec = BitVec::new();
    let mut writer = initialize_writer(result_filename)?;

    let mut current_byte: [u8; 1];
    let mut next_byte: [u8; 1] = [0u8; 1];
    let mut has_next: bool = reader.read(&mut next_byte)? > 0;

    while has_next {
        current_byte = next_byte;
        has_next = reader.read(&mut next_byte)? > 0;
        let mut bits_to_read = 8;
        if !has_next {
            bits_to_read = 8 - padding_size as usize;
        }
        for bit_index in 0..bits_to_read {
            let bit = extract_bit(current_byte[0], bit_index);
            acc.push(bit);
            if let Some(byte) = huff_table.get(&acc) {
                writer.write_all(&[*byte])?;
                acc.clear();
            }
        }
    }
    writer.flush()?;
    Ok(())
}

fn read_header_size(reader: &mut BufReader<File>) -> io::Result<u32> {
    let mut buffer = [0u8; 4];
    reader.read_exact(&mut buffer)?;
    Ok(u32::from_le_bytes(buffer))
}

fn read_padding_size(reader: &mut BufReader<File>) -> io::Result<u8> {
    let mut buffer = [0u8];
    reader.read_exact(&mut buffer)?;
    Ok(u8::from_le_bytes(buffer))
}

fn build_inverse_compression_table(
    freq_list: &FrequencyList,
) -> Result<InverseHuffTable, GenericError> {
    Ok(build_inverse_table(&build_tree(freq_list)?))
}

fn extract_bit(byte: u8, bit_index: usize) -> bool {
    (byte >> (7 - bit_index)) & 1 == 1
}
