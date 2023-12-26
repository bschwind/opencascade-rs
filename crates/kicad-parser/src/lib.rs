use sexp::{Atom, Sexp};
use thiserror::Error;

pub mod board;
pub mod graphics;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("S-Expression Parse Error: {0}")]
    SexpParseError(#[from] Box<sexp::Error>),
    #[error("Top level object is not a list")]
    TopLevelObjectNotList,
    #[error("First element in the top level list should be a string")]
    FirstElementInListNotString,
    #[error("The file is not a kicad_pcb file")]
    NotKicadPcbFile,
    #[error("Tried to extract a number which is not a float or an int")]
    NumberShouldBeFloatOrInt,
}

fn extract_number(num: &Sexp) -> Result<f64, Error> {
    match num {
        Sexp::Atom(Atom::F(float)) => Ok(*float),
        Sexp::Atom(Atom::I(int)) => Ok(*int as f64),
        _ => Err(Error::NumberShouldBeFloatOrInt),
    }
}

fn extract_coords(x: &Sexp, y: &Sexp) -> Result<(f64, f64), Error> {
    Ok((extract_number(x)?, extract_number(y)?))
}
