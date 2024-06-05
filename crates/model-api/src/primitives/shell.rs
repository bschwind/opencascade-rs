use crate::{primitives::Edge, wasm, wasm::WasmShell};

pub struct Shell {
    pub(crate) inner: WasmShell,
}

impl AsRef<Shell> for Shell {
    fn as_ref(&self) -> &Shell {
        self
    }
}

impl Shell {}
