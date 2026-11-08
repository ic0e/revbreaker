mod transform;

use clap::{Parser, builder::StringValueParser};
use std::process::Command;
use transform::{Value, try_all};

#[derive(Parser)]
#[command(name = "revbreaker", about = "Decoding and RE helper")]
struct Args {
    /// Input to decode
    #[arg(short = 'd', long)]
    decode: String,

    /// Expected output
    #[arg(short = 'e', long)]
    expected: Option<String>,

    /// Max results to show
    #[arg(short = 'n', long, default_value_t = 50)]
    top: usize,
}

fn main() {
    let args = Args::parse();
    let input = Value::Bytes(args.decode.as_bytes().to_vec());

    let results = try_all(&input);

    if results.is_empty() {
        println!("no decoder matched");
    }
    for (name, value) in results {
        println!("{:<10} {}", name, value);
    }
}
