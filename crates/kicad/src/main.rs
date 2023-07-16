use std::path::PathBuf;

use std::io::Error;

use clap::Parser;

mod kicad;
mod kicad_sexp;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The .kicad_pcb containing the board to create a case for.
    #[arg(value_name = "FILE")]
    input: PathBuf,
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    println!("Reading file: {}", args.input.display());

    let board = kicad_sexp::read_kicad_pcb(&args.input);

    println!("Board: {:?}", board);

    Ok(())
}
