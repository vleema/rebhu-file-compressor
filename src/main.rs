use huffman::{compressor::huff_compress, decompressor::huff_decompress};

mod huffman;
mod utils;

fn main() {
    match huff_compress("test-files/lorem.txt", "test-files/lorem.txt.rbh") {
        Ok(()) => {
            if let Err(e) = huff_decompress("test-files/lorem.txt.rbh", "test-files/lorem1.txt") {
                println!("Error: {}", e)
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
