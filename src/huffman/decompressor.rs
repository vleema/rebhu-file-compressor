use std::{
    fs::File,
    io::{self, BufReader, BufWriter, Read, Write},
};

use bitvec::{slice::BitSlice, vec::BitVec};

use super::{
    huff::{build_inverse_table, build_tree, FrequencyList},
    tree::HuffTree,
    GenericError, InverseHuffTable, PSEUDO_EOF,
};

pub fn huff_decompress(filename: &str, result_filename: &str) -> Result<HuffTree, GenericError> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let freq_list = read_header(&mut reader)?;
    let tree = build_tree(&freq_list)?;
    let inverse_table = build_inverse_table(&tree);
    decode_into_file(&inverse_table, &mut reader, result_filename)?;
    Ok(tree)
}

fn read_header(reader: &mut BufReader<File>) -> io::Result<FrequencyList> {
    let header_size = read_header_size(reader)?;
    let mut buffer = vec![0u8; header_size as usize];
    reader.read_exact(&mut buffer)?;
    let header: FrequencyList =
        bincode::deserialize(&buffer).expect("Failed to deserialize huffman frequency list");
    Ok(header)
}

fn decode_into_file(
    huff_table: &InverseHuffTable,
    reader: &mut BufReader<File>,
    result_filename: &str,
) -> io::Result<()> {
    let mut acc: BitVec = BitVec::new();
    let new_file = File::create(result_filename)?;
    let mut writer = BufWriter::new(new_file);
    for rbyte in reader.bytes() {
        let byte = rbyte?;
        let bits: &BitSlice<u8> = BitSlice::from_element(&byte);
        for bit in bits {
            acc.push(*bit);
            if let Some(ch) = huff_table.get(&acc) {
                if ch == &PSEUDO_EOF {
                    break;
                }
                write!(writer, "{}", ch)?;
                acc.clear();
            }
        }
    }
    Ok(())
}

fn read_header_size(reader: &mut BufReader<File>) -> io::Result<u32> {
    let mut buffer = [0u8; 4];
    reader.read_exact(&mut buffer)?;
    Ok(u32::from_le_bytes(buffer))
}
