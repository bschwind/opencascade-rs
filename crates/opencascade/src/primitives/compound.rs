use crate::primitives::Shape;
use cxx::UniquePtr;
use opencascade_sys::ffi;

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

    pub fn from_shapes<T: AsRef<Shape>>(shapes: impl IntoIterator<Item = T>) -> Self {
        let mut compound = ffi::TopoDS_Compound_ctor();
        let builder = ffi::BRep_Builder_ctor();
        let builder = ffi::BRep_Builder_upcast_to_topods_builder(&builder);
        builder.MakeCompound(compound.pin_mut());
        let mut compound_shape = ffi::TopoDS_Compound_as_shape(compound);

        for shape in shapes.into_iter() {
            builder.Add(compound_shape.pin_mut(), &shape.as_ref().inner);
        }

        let inner = ffi::TopoDS_cast_to_compound(&compound_shape);
        let inner = ffi::TopoDS_Compound_to_owned(inner);

        Self { inner }
    }
}
