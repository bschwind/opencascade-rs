use crate::wasm;

pub struct Shell {
    pub(crate) _inner: wasm::Shell,
}

impl AsRef<Shell> for Shell {
    fn as_ref(&self) -> &Shell {
        self
    }
}

impl Shell {}
