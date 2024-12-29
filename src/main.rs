use huffman::compressor::huff_compress;

mod huffman;
mod utils;

fn main() {
    match huff_compress("test-files/lorem.txt", "test-files/lorem.txt.rbh") {
        Ok(()) => (),
        Err(e) => println!("Error: {}", e),
    }
    // TODO: Decompress with huffman
}
