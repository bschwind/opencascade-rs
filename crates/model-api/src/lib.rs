use crate::primitives::Shape;

pub mod angle;
pub mod primitives;
pub mod wasm;
pub mod workplane;

pub trait Model: Send + Sync {
    fn new() -> Self
    where
        Self: Sized;

    fn create_model(&mut self) -> Shape;
}
