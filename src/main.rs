use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};

fn count_char_frequency(text: String, freq: &mut HashMap<char, u32>) {
    for ch in text.chars() {
        *freq.entry(ch).or_insert(0) += 1;
    }
}

fn main() {
    let file = File::open("lorem.txt").unwrap();
    let reader = BufReader::new(file);
    let mut ch_frequency: HashMap<char, u32> = HashMap::new();
    for line in reader.lines() {
        match line {
            Err(error) => {
                println!("Failed to read a line: {:?}", error);
                exit(1)
            }
            Ok(content) => count_char_frequency(content, &mut ch_frequency),
        }
    }
    println!("{:?}", ch_frequency)
}
