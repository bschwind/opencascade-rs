use crate::primitives::Shape;
use cxx::UniquePtr;
use opencascade_sys as ffi;

pub struct Compound {
    pub(crate) inner: UniquePtr<ffi::topo_ds::TopoDS_Compound>,
}

impl AsRef<Compound> for Compound {
    fn as_ref(&self) -> &Compound {
        self
    }
}

impl Compound {
    pub(crate) fn from_compound(compound: &ffi::topo_ds::TopoDS_Compound) -> Self {
        let inner = ffi::topo_ds::TopoDS_Compound_to_owned(compound);

        Self { inner }
    }

    #[must_use]
    pub fn clean(&self) -> Shape {
        let shape = ffi::topo_ds::cast_compound_to_shape(&self.inner);

        Shape::from_shape(shape).clean()
    }

    pub fn from_shapes<T: AsRef<Shape>>(shapes: impl IntoIterator<Item = T>) -> Self {
        let mut compound = ffi::topo_ds::TopoDS_Compound_new();
        let builder = ffi::b_rep::BRep_Builder_new();
        let builder = ffi::b_rep::BRep_Builder_upcast_to_topods_builder(&builder);
        builder.MakeCompound(compound.pin_mut());
        let mut compound_shape = ffi::topo_ds::TopoDS_Compound_as_shape(compound);

        for shape in shapes.into_iter() {
            builder.Add(compound_shape.pin_mut(), &shape.as_ref().inner);
        }

        let compound = ffi::topo_ds::TopoDS::Compound(&compound_shape);
        Self::from_compound(compound)
    }
}
