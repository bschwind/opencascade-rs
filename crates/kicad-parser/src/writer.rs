use crate::board::KicadBoard;

pub struct Writer<'a, W: std::io::Write> {
    writer: &'a mut W,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
}

impl<'a, W: std::io::Write> Writer<'a, W> {
    pub fn write_board(board: &KicadBoard) -> Result<(), Error> {
        todo!()
    }
}
