use crate::{primitives::Edge, wasm, wasm::WasmSolid};

pub struct Solid {
    pub(crate) inner: WasmSolid,
}

impl AsRef<Solid> for Solid {
    fn as_ref(&self) -> &Solid {
        self
    }
}

impl Solid {}
