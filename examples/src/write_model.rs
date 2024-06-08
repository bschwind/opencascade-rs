use std::{ffi::OsStr, os::unix::ffi::OsStrExt, path::PathBuf};

use clap::{Parser, ValueEnum};
use examples::Example;

/// Save an example model to a file
#[derive(Debug, Clone, Parser)]
struct Args {
    /// Example to save
    example: Example,

    /// Output file path
    #[clap(short, long, default_value = "output.step")]
    output: PathBuf,

    /// Output format
    #[clap(short, long)]
    format: Option<Format>,
}

#[derive(Debug, Clone, ValueEnum)]
enum Format {
    Step,
    Stl,
}

fn main() {
    let args = Args::parse();
    let model = args.example.shape();

    let format = args.format.unwrap_or_else(|| {
        let extension = args.output.extension().unwrap_or_else(|| {
            panic!("Cannot guess format because the output file name has no extension. Use the '-f' or '--format' flag to specify a format.")
        });

        determine_format(extension)
            .unwrap_or_else(|| panic!("Cannot guess format from extension {:?}.  Use the '-f' or '--format' flag to specify a format.", extension))
    });

    match format {
        Format::Step => model.write_step(args.output).unwrap(),
        Format::Stl => model.write_stl(args.output).unwrap(),
    }
}

fn determine_format(extension: &OsStr) -> Option<Format> {
    match extension.to_ascii_lowercase().as_bytes() {
        b"step" | b"stp" => Some(Format::Step),
        b"stl" => Some(Format::Stl),
        _ => None,
    }
}
