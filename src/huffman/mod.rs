use std::collections::HashMap;

use bitvec::vec::BitVec;

pub mod compressor;
pub mod decompressor;
mod huff;
mod tree;

const PSEUDO_EOF: u8 = b'\0';
type HuffTable = HashMap<u8, BitVec>;
type InverseHuffTable = HashMap<BitVec, u8>;
