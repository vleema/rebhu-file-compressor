use std::collections::HashMap;

use bitvec::vec::BitVec;

pub mod compressor;
pub mod decompressor;
mod huff;
mod tree;

type HuffTable = HashMap<u8, BitVec<u8>>;
type InverseHuffTable = HashMap<BitVec, u8>;
