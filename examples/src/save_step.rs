use std::path::PathBuf;

use clap::Parser;
use examples::Example;

/// Save an example model to a STEP file
#[derive(Debug, Clone, Parser)]
struct Args {
    /// Example to save
    example: Example,

    /// Output file path
    #[clap(short, long, default_value = "output.step")]
    out_path: PathBuf,
}

fn main() {
    let args = Args::parse();
    let model = args.example.shape();
    model.write_step(&args.out_path).unwrap();
}
