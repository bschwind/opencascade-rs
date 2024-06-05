use crate::{primitives::Wire, wasm, wasm::WasmShape};

pub struct Shape {
    pub(crate) inner: WasmShape,
}

impl AsRef<Shape> for Shape {
    fn as_ref(&self) -> &Shape {
        self
    }
}

impl Shape {
    pub fn from_wire(wire: &Wire) -> Self {
        let shape = wasm::WasmShape::from_wire(&wire.inner);

        Self { inner: shape }
    }
}
