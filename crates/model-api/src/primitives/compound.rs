use crate::{primitives::Edge, wasm, wasm::WasmCompound};

pub struct Compound {
    pub(crate) inner: WasmCompound,
}

impl AsRef<Compound> for Compound {
    fn as_ref(&self) -> &Compound {
        self
    }
}

impl Compound {}
