use thiserror::Error;

pub mod board;
pub mod graphics;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("IO Error: {0}")]
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
