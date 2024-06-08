use crate::wasm;

pub struct Solid {
    pub(crate) _inner: wasm::Solid,
}

impl AsRef<Solid> for Solid {
    fn as_ref(&self) -> &Solid {
        self
    }
}

impl Solid {}
