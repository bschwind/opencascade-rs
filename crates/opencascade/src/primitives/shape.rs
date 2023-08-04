use crate::{
    adhoc::AdHocShape,
    mesh::{Mesh, Mesher},
    primitives::{
        make_dir, make_point, make_vec, BooleanShape, Compound, Edge, EdgeIterator, Face,
        FaceIterator, ShapeType, Solid, Vertex, Wire,
    },
    Error,
};
use cxx::UniquePtr;
use glam::{dvec3, DVec3};
use opencascade_sys::ffi::{self, IFSelect_ReturnStatus};
use std::path::Path;

pub struct Shape {
    pub(crate) inner: UniquePtr<ffi::TopoDS_Shape>,
}

impl AsRef<Shape> for Shape {
    fn as_ref(&self) -> &Shape {
        self
    }
}

impl From<Vertex> for Shape {
    fn from(vertex: Vertex) -> Self {
        let shape = ffi::cast_vertex_to_shape(&vertex.inner);
        let inner = ffi::TopoDS_Shape_to_owned(shape);

        Shape { inner }
    }
}

impl From<Edge> for Shape {
    fn from(edge: Edge) -> Self {
        let shape = ffi::cast_edge_to_shape(&edge.inner);
        let inner = ffi::TopoDS_Shape_to_owned(shape);

        Shape { inner }
    }
}

impl From<Wire> for Shape {
    fn from(wire: Wire) -> Self {
        let shape = ffi::cast_wire_to_shape(&wire.inner);
        let inner = ffi::TopoDS_Shape_to_owned(shape);

        Shape { inner }
    }
}

impl From<Face> for Shape {
    fn from(face: Face) -> Self {
        let shape = ffi::cast_face_to_shape(&face.inner);
        let inner = ffi::TopoDS_Shape_to_owned(shape);

        Shape { inner }
    }
}

impl From<Solid> for Shape {
    fn from(solid: Solid) -> Self {
        let shape = ffi::cast_solid_to_shape(&solid.inner);
        let inner = ffi::TopoDS_Shape_to_owned(shape);

        Shape { inner }
    }
}

impl From<Compound> for Shape {
    fn from(compound: Compound) -> Self {
        let shape = ffi::cast_compound_to_shape(&compound.inner);
        let inner = ffi::TopoDS_Shape_to_owned(shape);

        Shape { inner }
    }
}

impl From<BooleanShape> for Shape {
    fn from(boolean_shape: BooleanShape) -> Self {
        boolean_shape.shape
    }
}

impl From<AdHocShape> for Shape {
    fn from(adhoc_shape: AdHocShape) -> Self {
        adhoc_shape.0
    }
}

impl Shape {
    pub fn shape_type(&self) -> ShapeType {
        self.inner.ShapeType().into()
    }

    pub fn fillet_edge(&mut self, radius: f64, edge: &Edge) {
        let mut make_fillet = ffi::BRepFilletAPI_MakeFillet_ctor(&self.inner);
        make_fillet.pin_mut().add_edge(radius, &edge.inner);

        let filleted_shape = make_fillet.pin_mut().Shape();

        self.inner = ffi::TopoDS_Shape_to_owned(filleted_shape);
    }

    pub fn chamfer_edge(&mut self, distance: f64, edge: &Edge) {
        let mut make_chamfer = ffi::BRepFilletAPI_MakeChamfer_ctor(&self.inner);
        make_chamfer.pin_mut().add_edge(distance, &edge.inner);

        let chamfered_shape = make_chamfer.pin_mut().Shape();

        self.inner = ffi::TopoDS_Shape_to_owned(chamfered_shape);
    }

    pub fn fillet_edges<T: AsRef<Edge>>(
        &mut self,
        radius: f64,
        edges: impl IntoIterator<Item = T>,
    ) {
        let mut make_fillet = ffi::BRepFilletAPI_MakeFillet_ctor(&self.inner);

        for edge in edges.into_iter() {
            make_fillet.pin_mut().add_edge(radius, &edge.as_ref().inner);
        }

        let filleted_shape = make_fillet.pin_mut().Shape();

        self.inner = ffi::TopoDS_Shape_to_owned(filleted_shape);
    }

    pub fn chamfer_edges<T: AsRef<Edge>>(
        &mut self,
        distance: f64,
        edges: impl IntoIterator<Item = T>,
    ) {
        let mut make_chamfer = ffi::BRepFilletAPI_MakeChamfer_ctor(&self.inner);

        for edge in edges.into_iter() {
            make_chamfer.pin_mut().add_edge(distance, &edge.as_ref().inner);
        }

        let chamfered_shape = make_chamfer.pin_mut().Shape();

        self.inner = ffi::TopoDS_Shape_to_owned(chamfered_shape);
    }

    /// Performs fillet of `radius` on all edges of the shape
    pub fn fillet(&mut self, radius: f64) {
        self.fillet_edges(radius, self.edges());
    }

    /// Performs chamfer of `distance` on all edges of the shape
    pub fn chamfer(&mut self, distance: f64) {
        self.chamfer_edges(distance, self.edges());
    }

    pub fn subtract(&self, other: &Shape) -> BooleanShape {
        let mut cut_operation = ffi::BRepAlgoAPI_Cut_ctor(&self.inner, &other.inner);

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

    pub fn read_step(path: impl AsRef<Path>) -> Result<Self, Error> {
        let mut reader = ffi::STEPControl_Reader_ctor();

        let status = ffi::read_step(reader.pin_mut(), path.as_ref().to_string_lossy().to_string());

        if status != IFSelect_ReturnStatus::IFSelect_RetDone {
            return Err(Error::StepReadFailed);
        }

        reader.pin_mut().TransferRoots(&ffi::Message_ProgressRange_ctor());

        let inner = ffi::one_shape(&reader);

        Ok(Self { inner })
    }

    pub fn write_step(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        let mut writer = ffi::STEPControl_Writer_ctor();

        let status = ffi::transfer_shape(writer.pin_mut(), &self.inner);

        if status != IFSelect_ReturnStatus::IFSelect_RetDone {
            return Err(Error::StepWriteFailed);
        }

        let status = ffi::write_step(writer.pin_mut(), path.as_ref().to_string_lossy().to_string());

        if status != IFSelect_ReturnStatus::IFSelect_RetDone {
            return Err(Error::StepWriteFailed);
        }

        Ok(())
    }

    pub fn union(&self, other: &Shape) -> BooleanShape {
        let mut fuse_operation = ffi::BRepAlgoAPI_Fuse_ctor(&self.inner, &other.inner);
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

    pub fn write_stl<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let mut stl_writer = ffi::StlAPI_Writer_ctor();
        let triangulation = ffi::BRepMesh_IncrementalMesh_ctor(&self.inner, 0.001);
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

    pub fn clean(&mut self) {
        let mut upgrader = ffi::ShapeUpgrade_UnifySameDomain_ctor(&self.inner, true, true, true);
        upgrader.pin_mut().AllowInternalEdges(false);
        upgrader.pin_mut().Build();

        let upgraded_shape = upgrader.Shape();

        self.inner = ffi::TopoDS_Shape_to_owned(upgraded_shape);
    }

    pub fn set_global_translation(&mut self, translation: DVec3) {
        let mut transform = ffi::new_transform();
        let translation_vec = make_vec(translation);
        transform.pin_mut().set_translation_vec(&translation_vec);

        let location = ffi::TopLoc_Location_from_transform(&transform);

        self.inner.pin_mut().set_global_translation(&location, false);
    }

    pub fn mesh(&self) -> Mesh {
        let mesher = Mesher::new(self);
        mesher.mesh()
    }

    pub fn edges(&self) -> EdgeIterator {
        let explorer = ffi::TopExp_Explorer_ctor(&self.inner, ffi::TopAbs_ShapeEnum::TopAbs_EDGE);

        EdgeIterator { explorer }
    }

    pub fn faces(&self) -> FaceIterator {
        let explorer = ffi::TopExp_Explorer_ctor(&self.inner, ffi::TopAbs_ShapeEnum::TopAbs_FACE);

        FaceIterator { explorer }
    }

    // TODO(bschwind) - Convert the return type to an iterator.
    pub fn faces_along_ray(&self, ray_start: DVec3, ray_dir: DVec3) -> Vec<(Face, DVec3)> {
        let mut intersector = ffi::BRepIntCurveSurface_Inter_ctor();
        let tolerance = 0.0001;
        intersector.pin_mut().Init(
            &self.inner,
            &ffi::gp_Lin_ctor(&make_point(ray_start), &make_dir(ray_dir)),
            tolerance,
        );

        let mut results = vec![];

        while intersector.More() {
            let face = ffi::BRepIntCurveSurface_Inter_face(&intersector);
            let point = ffi::BRepIntCurveSurface_Inter_point(&intersector);

            let face = Face { inner: ffi::TopoDS_Face_to_owned(&face) };

            results.push((face, dvec3(point.X(), point.Y(), point.Z())));

            intersector.pin_mut().Next();
        }

        results
    }

    pub fn hollow<T: AsRef<Face>>(
        self,
        offset: f64,
        faces_to_remove: impl IntoIterator<Item = T>,
    ) -> Self {
        let mut faces_list = ffi::new_list_of_shape();

        for face in faces_to_remove.into_iter() {
            ffi::shape_list_append_face(faces_list.pin_mut(), &face.as_ref().inner);
        }

        let mut solid_maker = ffi::BRepOffsetAPI_MakeThickSolid_ctor();
        ffi::MakeThickSolidByJoin(solid_maker.pin_mut(), &self.inner, &faces_list, offset, 0.001);

        let hollowed_shape = solid_maker.pin_mut().Shape();
        let inner = ffi::TopoDS_Shape_to_owned(hollowed_shape);

        Self { inner }
    }
}
