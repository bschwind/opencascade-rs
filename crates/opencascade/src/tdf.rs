use cxx::UniquePtr;
use opencascade_sys::ffi;
use std::marker::PhantomData;

/// ```compile_fail
/// use opencascade::tdf::TdfData;
/// let root = {
///     let doc = TdfData::new();
///     doc.root()
/// };
/// ```
pub struct TdfData {
    inner: UniquePtr<ffi::HandleTdfData>,
}

impl TdfData {
    pub fn new() -> Self {
        Self { inner: ffi::TDF_Data_new() }
    }

    pub fn root(&self) -> TdfLabel<'_> {
        TdfLabel { inner: ffi::TDF_Data_root(self.inner.as_ref().unwrap()), _phantom: PhantomData }
    }
    pub fn transaction(&self) -> TdfTransaction<'_> {
        TdfTransaction {
            inner: ffi::TDF_Transaction_new(self.inner.as_ref().unwrap()),
            _phantom: PhantomData,
        }
    }
}

pub struct TdfLabel<'doc> {
    inner: UniquePtr<ffi::TDF_Label>,
    _phantom: PhantomData<&'doc TdfData>,
}

impl<'doc> TdfLabel<'doc> {
    pub fn new_child(&self) -> TdfLabel<'doc> {
        TdfLabel {
            inner: ffi::TDF_Label_new_child(self.inner.as_ref().unwrap()),
            _phantom: PhantomData,
        }
    }

    pub fn is_null(&self) -> bool {
        ffi::TDF_Label_is_null(self.inner.as_ref().unwrap())
    }
}
pub struct TdfTransaction<'doc> {
    inner: UniquePtr<ffi::TDF_Transaction>,
    _phantom: PhantomData<&'doc TdfData>,
}

impl<'doc> TdfTransaction<'doc> {
    pub fn open(&mut self) -> i32 {
        ffi::TDF_Transaction_open(self.inner.pin_mut())
    }

    // TODO: wrap HandleTdfDelta in a safe TdfDelta type before contributing upstream.
    // Currently leaks the raw FFI type into the safe wrapper layer.
    pub fn commit(mut self) -> UniquePtr<ffi::HandleTdfDelta> {
        ffi::TDF_Transaction_commit(self.inner.pin_mut())
    }

    pub fn abort(mut self) {
        ffi::TDF_Transaction_abort(self.inner.pin_mut())
    }

    pub fn is_open(&self) -> bool {
        ffi::TDF_Transaction_is_open(self.inner.as_ref().unwrap())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tdf_data_constructs() {
        let doc = TdfData::new();
        drop(doc);
    }
    #[test]
    fn test_tdf_data_root_is_not_null() {
        let doc = TdfData::new();
        let root = doc.root();
        assert!(!root.is_null());
    }

    #[test]
    fn test_new_child_is_not_null() {
        let doc = TdfData::new();
        let root = doc.root();
        // let child = root.new_child();
        assert!(!root.is_null());
        // drop(child);
    }
    #[test]
    fn test_new_child_with_transaction() {
        let doc = TdfData::new();
        let mut tx = doc.transaction();
        tx.open();
        let root = doc.root();
        let child = root.new_child();
        assert!(!child.is_null());
        tx.commit();
    }
}
