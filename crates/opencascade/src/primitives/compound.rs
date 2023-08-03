use crate::primitives::Shape;
use cxx::UniquePtr;
use opencascade_sys::ffi::{self};

pub struct Compound {
    pub(crate) inner: UniquePtr<ffi::TopoDS_Compound>,
}

impl AsRef<Compound> for Compound {
    fn as_ref(&self) -> &Compound {
        self
    }
}

impl Compound {
    pub fn clean(&mut self) -> Shape {
        let inner = ffi::cast_compound_to_shape(&self.inner);
        let inner = ffi::TopoDS_Shape_to_owned(inner);
        let mut shape = Shape { inner };

        shape.clean();

        shape
    }

    pub fn to_shape(self) -> Shape {
        let inner_shape = ffi::cast_compound_to_shape(&self.inner);
        let inner = ffi::TopoDS_Shape_to_owned(inner_shape);

        Shape { inner }
    }
}
