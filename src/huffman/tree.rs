use std::{cmp::Ordering, collections::HashMap};

use bitvec::vec::BitVec;

use super::{HuffTable, InverseHuffTable};

#[derive(Debug)]
pub enum HuffTree {
    Leaf {
        byte: u8,
        freq: u32,
    },
    Node {
        freq: u32,
        left: Box<HuffTree>,
        right: Box<HuffTree>,
    },
}

impl PartialEq for HuffTree {
    fn eq(&self, other: &Self) -> bool {
        self.freq() == other.freq()
    }
}

impl Eq for HuffTree {}

impl PartialOrd for HuffTree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.freq().cmp(&other.freq()))
    }
}

impl Ord for HuffTree {
    fn cmp(&self, other: &Self) -> Ordering {
        self.freq().cmp(&other.freq())
    }
}

impl HuffTree {
    pub fn pre_order_code(&self, code: BitVec<u8>) -> HuffTable {
        let mut traversal_leafs: HashMap<u8, BitVec<u8>> = HashMap::new();
        match self {
            Self::Leaf { byte, .. } => {
                traversal_leafs.insert(*byte, code);
            }
            Self::Node { left, right, .. } => {
                let mut left_code = code.clone();
                left_code.push(false); // false represents 0
                traversal_leafs.extend(left.pre_order_code(left_code));

                let mut right_code = code.clone();
                right_code.push(true); // true represents 1
                traversal_leafs.extend(right.pre_order_code(right_code));
            }
        }
        traversal_leafs
    }

    pub fn pre_order_byte(&self, code: BitVec) -> InverseHuffTable {
        let mut traversal_leafs: HashMap<BitVec, u8> = HashMap::new();
        match self {
            Self::Leaf { byte, .. } => {
                traversal_leafs.insert(code, *byte);
            }
            Self::Node { left, right, .. } => {
                let mut left_code = code.clone();
                left_code.push(false); // false represents 0
                traversal_leafs.extend(left.pre_order_byte(left_code));

                let mut right_code = code.clone();
                right_code.push(true); // true represents 1
                traversal_leafs.extend(right.pre_order_byte(right_code));
            }
        }
        traversal_leafs
    }

    pub fn combine(left: HuffTree, right: HuffTree) -> HuffTree {
        Self::Node {
            freq: left.freq() + right.freq(),
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn leaf(byte: u8, freq: u32) -> HuffTree {
        Self::Leaf { byte, freq }
    }

    fn freq(&self) -> u32 {
        match self {
            Self::Leaf { freq, .. } | Self::Node { freq, .. } => *freq,
        }
    }
}
