mod python_transforms;
mod transform;

use clap::{Parser};
use colored::Colorize;
use std::process::exit;
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
    let expected = args.expected;

    let mut results: Vec<(String, Value)> = try_all(&input)
        .into_iter()
        .map(|(name, v)| (name.to_string(), v))
        .collect();

    let py_results =
        python_transforms::python_transforms(args.decode.as_bytes()).unwrap_or_default();

    results.extend(
        py_results
            .into_iter()
            .map(|(name, bytes)| (name, Value::Bytes(bytes))),
    );

    if results.is_empty() {
        println!("no decoder output");
        exit(0);
    }
    println!("EXPECTED RESULTS:");
    if let Some(exp) = &expected {
        for (name, value) in &results {
            let text = value.to_string();
            if text.contains(exp) {
                let colored = text.replace(exp, &exp.green().to_string());
                println!("{:<10} {}", name, colored);
            }
        }
    }
    println!("ALL RESULTS:");
    for (name, value) in results {
        println!("{:<10} {}", name.cyan(), value);
    }
}
