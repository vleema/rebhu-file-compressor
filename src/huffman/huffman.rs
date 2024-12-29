use serde::{Deserialize, Serialize};

use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
};

use super::{tree::HuffTree, PSEUDO_EOF};
use crate::utils::min_heap::MinHeap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    pub frequency_list: HashMap<char, u32>,
}

pub fn build_tree(freq: &HashMap<char, u32>) -> Result<HuffTree, Error> {
    let temp_vec = Vec::with_capacity(freq.len() + 1); // +1 to account for EOF char
    let mut heap: MinHeap<HuffTree> = temp_vec.into();

    heap.push(HuffTree::leaf(PSEUDO_EOF, 1));
    for (ch, freq) in freq {
        heap.push(HuffTree::leaf(*ch, *freq));
    }

    match merge(&mut heap) {
        Some(tree) => Ok(tree),
        None => Err(Error::new(
            ErrorKind::Other,
            "Unexpected error while merging the Huffman tree",
        )),
    }
}

fn merge(heap: &mut MinHeap<HuffTree>) -> Option<HuffTree> {
    while heap.len() > 1 {
        let right = heap.pop()?;
        let left = heap.pop()?;
        heap.push(HuffTree::combine(left, right));
    }
    heap.pop()
}
