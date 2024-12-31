use bitvec::vec::BitVec;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use super::{tree::HuffTree, HuffTable, InverseHuffTable};
use crate::utils::{min_heap::MinHeap, GenericError};

#[derive(Serialize, Deserialize, Debug)]
struct FrequencyEntry {
    byte: u8,
    freq: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FrequencyList {
    entries: Vec<FrequencyEntry>,
}

impl FrequencyList {
    pub fn from_hashmap(freq_map: &HashMap<u8, u32>) -> Self {
        let mut entries: Vec<FrequencyEntry> = freq_map
            .iter()
            .map(|(&byte, &freq)| FrequencyEntry { byte, freq })
            .collect();
        entries.sort_by(|a, b| a.byte.cmp(&b.byte)); // Sort by character
        FrequencyList { entries }
    }
}

pub fn build_table(tree: &HuffTree) -> HuffTable {
    tree.pre_order_code(BitVec::new())
}

pub fn build_inverse_table(tree: &HuffTree) -> InverseHuffTable {
    tree.pre_order_byte(BitVec::new())
}

pub fn build_tree(freq_list: &FrequencyList) -> Result<HuffTree, GenericError> {
    let temp_vec = Vec::with_capacity(freq_list.entries.len());
    let mut heap: MinHeap<HuffTree> = temp_vec.into();

    for FrequencyEntry { byte, freq } in &freq_list.entries {
        heap.push(HuffTree::leaf(*byte, *freq));
    }

    match merge(&mut heap) {
        Some(tree) => Ok(tree),
        None => Err(Box::from("Unexpected error while merging the Huffman tree")),
    }
}

fn merge(heap: &mut MinHeap<HuffTree>) -> Option<HuffTree> {
    while heap.len() > 1 {
        let left = heap.pop()?;
        let right = heap.pop()?;
        heap.push(HuffTree::combine(left, right));
    }
    heap.pop()
}
