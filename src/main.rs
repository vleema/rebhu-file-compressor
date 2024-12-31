mod huffman;
mod utils;

fn main() {
    let args = <Args as clap::Parser>::parse();

    if let Some(output) = &args.output {
        if args.files.len() != output.len() {
            <Args as clap::CommandFactory>::command()
                .error(
                    clap::error::ErrorKind::WrongNumberOfValues,
                    format!(
                        "Number of '--output' arguments ({}) should be the same as <FILES> ({})",
                        output.len(),
                        args.files.len(),
                    ),
                )
                .exit()
        }
    }
    let operation: fn(&str, &str) -> Result<(), utils::GenericError>;
    let mut show_compression_rate = false;
    if args.compress {
        operation = huffman::compressor::huff_compress;
        show_compression_rate = args.verbose;
    } else if args.decompress {
        operation = huffman::decompressor::huff_decompress;
    } else {
        eprintln!("Unexpected error, while selecting operation");
        std::process::exit(1)
    }
    for i in 0..args.files.len() {
        let file = args.files.get(i).unwrap();
        let output = match args.output {
            Some(ref outputs) => outputs.get(i).unwrap(),
            None => &format!("{}.rbh", file),
        };
        if let Err(e) = operation(file, output) {
            eprintln!("ERROR: {}", e);
            std::process::exit(1)
        }
        if show_compression_rate {
            match utils::file::compression_rate(file, output) {
                Ok(rate) => {
                    println!("{} compression rate: {:.2}%", file, rate);
                }
                Err(e) => {
                    eprintln!("ERROR: {}", e);
                    std::process::exit(1)
                }
            }
        }
    }
}

#[derive(clap::Parser)]
#[command(name ="rebhu",
    author = "Vinicius de Lima", 
    version,
    about = "Simple file compressor made in rust", 
    long_about = None)]
#[command(group = clap::ArgGroup::new("operation").args(&["compress", "decompress"]).required(true))]
pub struct Args {
    #[arg(help = "Files to be compressed/decompressed", required = true)]
    files: Vec<String>,
    #[arg(
        short,
        long,
        help = "Files will be compressed (should not be run with --decompress)"
    )]
    compress: bool,
    #[arg(
        short,
        long,
        help = "Files will be decompressed (should not be run with --compress and must run with --output)",
        requires = "output"
    )]
    decompress: bool,
    #[arg(
        short,
        long,
        help = "The resulting name of the compressed/decompressed files",
        num_args = 1..,
        value_delimiter = ' ',
    )]
    output: Option<Vec<String>>,
    #[arg(
        short,
        long,
        help = "Shows the compression rate (only when compressing)"
    )]
    verbose: bool,
}
