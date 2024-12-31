# Rebhu file compressor

Simple file compressor that combines the Huffman algorithm, Burrows Wheeler Transformation (BWT) and Run-Length encoding (RLE) to compress and decompress files.

## Dependencies

- Cargo (Rust package manager)

## Usage

```
$ rebhu -h
Simple file compressor made in rust 🦀

Usage: rebhu [OPTIONS] <--compress|--decompress> <FILES>...

Arguments:
  <FILES>...  Files to be compressed/decompressed

Options:
  -c, --compress            Files will be compressed (should not be run with --decompress)
  -d, --decompress          Files will be decompressed (should not be run with --compress and must run with --output)
  -o, --output <OUTPUT>...  The resulting name of the compressed/decompressed files
  -v, --verbose             Shows the compression rate (only when compressing)
  -h, --help                Print help
  -V, --version             Print version
```

- **Installation**:

  ```
  cargo install --path .
  ```

- **Run**:

  ```
  cargo r --release -- <args...>
  ```

## Known Problems

- Conflict with PSEUDO EOF character in regular binary files
