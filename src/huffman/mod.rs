use std::collections::HashMap;

use bitvec::vec::BitVec;

pub mod compressor;
pub mod decompressor;
mod huff;
mod tree;

const PSEUDO_EOF: char = '\0';
type HuffTable = HashMap<char, BitVec>;
type InverseHuffTable = HashMap<BitVec, char>;
