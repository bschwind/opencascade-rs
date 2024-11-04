use crate::primitives::Shape;
use cxx::UniquePtr;
use opencascade_sys::ffi;

pub struct Compound {
    pub(crate) inner: UniquePtr<ffi::TopoDSCompound>,
}

impl AsRef<Compound> for Compound {
    fn as_ref(&self) -> &Compound {
        self
    }
}

impl Compound {
    pub(crate) fn from_compound(compound: &ffi::TopoDSCompound) -> Self {
        let inner = ffi::TopoDSCompound_to_owned(compound);

        Self { inner }
    }

    #[must_use]
    pub fn clean(&self) -> Shape {
        let shape = ffi::cast_compound_to_shape(&self.inner);

        Shape::from_shape(shape).clean()
    }

    pub fn from_shapes<T: AsRef<Shape>>(shapes: impl IntoIterator<Item = T>) -> Self {
        let mut compound = ffi::TopoDSCompound_ctor();
        let builder = ffi::BRepBuilder_ctor();
        let builder = ffi::BRepBuilder_upcast_to_topodsbuilder(&builder);
        builder.make_compound(compound.pin_mut());
        let mut compound_shape = ffi::TopoDSCompound_as_shape(compound);

        for shape in shapes.into_iter() {
            builder.add(compound_shape.pin_mut(), &shape.as_ref().inner);
        }

        let compound = ffi::TopoDS_cast_to_compound(&compound_shape);
        Self::from_compound(compound)
    }
}
