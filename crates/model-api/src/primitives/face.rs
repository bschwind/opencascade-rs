use crate::wasm::WasmFace;

pub struct Face {
    pub(crate) inner: WasmFace,
}

impl AsRef<Face> for Face {
    fn as_ref(&self) -> &Face {
        self
    }
}

impl Face {}
