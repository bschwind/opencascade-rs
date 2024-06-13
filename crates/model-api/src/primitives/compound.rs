use crate::wasm;

pub struct Compound {
    pub(crate) inner: wasm::Compound,
}

impl AsRef<Compound> for Compound {
    fn as_ref(&self) -> &Compound {
        self
    }
}

impl Compound {}
