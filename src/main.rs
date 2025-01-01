mod huffman;
mod utils;

type Operation = fn(&str, &str) -> Result<(), utils::GenericError>;

fn main() {
    let args = <Args as clap::Parser>::parse();
    abort_if_output_does_not_match_input(&args.files, &args.output);
    let (operation, show_compression_rate) = get_operation(args.compress);
    apply_operation(operation, args.files, args.output, show_compression_rate);
}

fn abort_if_output_does_not_match_input(files: &[String], output: &Option<Vec<String>>) {
    if let Some(output) = output {
        if files.len() != output.len() {
            <Args as clap::CommandFactory>::command()
                .error(
                    clap::error::ErrorKind::WrongNumberOfValues,
                    format!(
                        "Number of '--output' arguments ({}) should be the same as <FILES> ({})",
                        output.len(),
                        files.len(),
                    ),
                )
                .exit()
        }
    }
}

fn get_operation(compress: bool) -> (Operation, bool) {
    if compress {
        (huffman::compressor::huff_compress, true)
    } else {
        (huffman::decompressor::huff_decompress, false)
    }
}

fn apply_operation(
    operation: fn(&str, &str) -> Result<(), utils::GenericError>,
    files: Vec<String>,
    output_files: Option<Vec<String>>,
    show_compression_rate: bool,
) {
    for i in 0..files.len() {
        let file = files.get(i).unwrap();
        let output = match output_files {
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
    about = "Simple file compressor made in rust 🦀", 
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
