use crate::{
    primitives::{BooleanShape, Compound, Edge, Shape, Wire},
    Error,
};
use cxx::UniquePtr;
use opencascade_sys::ffi::{self};
use std::path::Path;

pub struct Solid {
    pub(crate) inner: UniquePtr<ffi::TopoDS_Solid>,
}

impl AsRef<Solid> for Solid {
    fn as_ref(&self) -> &Solid {
        self
    }
}

impl Solid {
    pub fn to_shape(self) -> Shape {
        let inner_shape = ffi::cast_solid_to_shape(&self.inner);
        let inner = ffi::TopoDS_Shape_to_owned(inner_shape);

        Shape { inner }
    }

    // TODO(bschwind) - Do some cool stuff from this link:
    // https://neweopencascade.wordpress.com/2018/10/17/lets-talk-about-fillets/
    // Key takeaway: Use the `SectionEdges` function to retrieve edges that were
    // the result of combining two shapes.
    pub fn fillet_edge(&self, radius: f64, edge: &Edge) -> Compound {
        let inner_shape = ffi::cast_solid_to_shape(&self.inner);

        let mut make_fillet = ffi::BRepFilletAPI_MakeFillet_ctor(inner_shape);
        make_fillet.pin_mut().add_edge(radius, &edge.inner);

        let filleted_shape = make_fillet.pin_mut().Shape();

        let compund = ffi::TopoDS_cast_to_compound(filleted_shape);
        let inner = ffi::TopoDS_Compound_to_owned(compund);

        Compound { inner }
    }

    pub fn loft<T: AsRef<Wire>>(wires: impl IntoIterator<Item = T>) -> Self {
        let is_solid = true;
        let mut make_loft = ffi::BRepOffsetAPI_ThruSections_ctor(is_solid);

        for wire in wires.into_iter() {
            make_loft.pin_mut().AddWire(&wire.as_ref().inner);
        }

        // Set to CheckCompatibility to `true` to avoid twisted results.
        make_loft.pin_mut().CheckCompatibility(true);

        let shape = make_loft.pin_mut().Shape();
        let solid = ffi::TopoDS_cast_to_solid(shape);
        let inner = ffi::TopoDS_Solid_to_owned(solid);

        Self { inner }
    }

    pub fn write_stl<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let inner_shape = ffi::cast_solid_to_shape(&self.inner);

        let mut stl_writer = ffi::StlAPI_Writer_ctor();
        let triangulation = ffi::BRepMesh_IncrementalMesh_ctor(inner_shape, 0.001);
        let success = ffi::write_stl(
            stl_writer.pin_mut(),
            triangulation.Shape(),
            path.as_ref().to_string_lossy().to_string(),
        );

        if success {
            Ok(())
        } else {
            Err(Error::StlWriteFailed)
        }
    }

    pub fn subtract(&self, other: &Solid) -> BooleanShape {
        let inner_shape = ffi::cast_solid_to_shape(&self.inner);
        let other_inner_shape = ffi::cast_solid_to_shape(&other.inner);

        let mut cut_operation = ffi::BRepAlgoAPI_Cut_ctor(inner_shape, other_inner_shape);

        let edge_list = cut_operation.pin_mut().SectionEdges();
        let vec = ffi::shape_list_to_vector(edge_list);

        let mut new_edges = vec![];
        for shape in vec.iter() {
            let edge = ffi::TopoDS_cast_to_edge(shape);
            let inner = ffi::TopoDS_Edge_to_owned(edge);
            let edge = Edge { inner };
            new_edges.push(edge);
        }

        let cut_shape = cut_operation.pin_mut().Shape();
        let inner = ffi::TopoDS_Shape_to_owned(cut_shape);

        BooleanShape { shape: Shape { inner }, new_edges }
    }

    pub fn union(&self, other: &Solid) -> BooleanShape {
        let inner_shape = ffi::cast_solid_to_shape(&self.inner);
        let other_inner_shape = ffi::cast_solid_to_shape(&other.inner);

        let mut fuse_operation = ffi::BRepAlgoAPI_Fuse_ctor(inner_shape, other_inner_shape);
        let edge_list = fuse_operation.pin_mut().SectionEdges();
        let vec = ffi::shape_list_to_vector(edge_list);

        let mut new_edges = vec![];
        for shape in vec.iter() {
            let edge = ffi::TopoDS_cast_to_edge(shape);
            let inner = ffi::TopoDS_Edge_to_owned(edge);
            let edge = Edge { inner };
            new_edges.push(edge);
        }

        let fuse_shape = fuse_operation.pin_mut().Shape();
        let inner = ffi::TopoDS_Shape_to_owned(fuse_shape);

        BooleanShape { shape: Shape { inner }, new_edges }
    }
}
