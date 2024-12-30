use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct MinHeap<T> {
    heap: BinaryHeap<Reverse<T>>,
}

#[allow(dead_code)]
impl<T: Ord> MinHeap<T> {
    pub fn new() -> Self {
        MinHeap {
            heap: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, value: T) {
        self.heap.push(Reverse(value));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|Reverse(value)| value)
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.peek().map(|Reverse(value)| value)
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }
}

impl<T: Ord> From<Vec<T>> for MinHeap<T> {
    fn from(vec: Vec<T>) -> Self {
        MinHeap {
            heap: vec.into_iter().map(Reverse).collect(),
        }
    }
}
