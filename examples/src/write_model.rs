use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use examples::Example;

/// Save an example model to a file
#[derive(Debug, Clone, Parser)]
struct Args {
    /// Example to save
    example: Example,

    /// Output file path, WITHOUT the extension
    #[clap(short, long, default_value = "output")]
    output: PathBuf,

    /// Output format
    #[clap(short, long, default_value = "step")]
    format: Format,
}

#[derive(Debug, Clone, ValueEnum)]
enum Format {
    Step,
    Stl,
}

fn main() {
    let args = Args::parse();
    let model = args.example.shape();
    match args.format {
        Format::Step => model.write_step(args.output.with_extension("step")).unwrap(),
        Format::Stl => model.write_stl(args.output.with_extension("stl")).unwrap(),
    }
}
