use thiserror::Error;

pub mod adhoc;
pub mod angle;
pub mod primitives;
pub mod workplane;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to write STL file")]
    StlWriteFailed,
}
