use crate::{
    mesh::{Mesh, Mesher},
    primitives::{
        make_dir, make_point, make_point2d, make_vec, BooleanShape, Compound, Edge, EdgeIterator,
        Face, FaceIterator, ShapeType, Solid, Vertex, Wire,
    },
    Error,
};
use cxx::UniquePtr;
use glam::{dvec2, dvec3, DVec3};
use opencascade_sys::ffi;
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

        Self::from_shape(shape)
    }
}

impl From<Edge> for Shape {
    fn from(edge: Edge) -> Self {
        let shape = ffi::cast_edge_to_shape(&edge.inner);

        Self::from_shape(shape)
    }
}

impl From<Wire> for Shape {
    fn from(wire: Wire) -> Self {
        let shape = ffi::cast_wire_to_shape(&wire.inner);

        Self::from_shape(shape)
    }
}

impl From<Face> for Shape {
    fn from(face: Face) -> Self {
        let shape = ffi::cast_face_to_shape(&face.inner);

        Self::from_shape(shape)
    }
}

impl From<Solid> for Shape {
    fn from(solid: Solid) -> Self {
        let shape = ffi::cast_solid_to_shape(&solid.inner);

        Self::from_shape(shape)
    }
}

impl From<Compound> for Shape {
    fn from(compound: Compound) -> Self {
        let shape = ffi::cast_compound_to_shape(&compound.inner);

        Self::from_shape(shape)
    }
}

impl From<BooleanShape> for Shape {
    fn from(boolean_shape: BooleanShape) -> Self {
        boolean_shape.shape
    }
}

impl Shape {
    pub(crate) fn from_shape(shape: &ffi::TopoDS_Shape) -> Self {
        let inner = ffi::TopoDS_Shape_to_owned(shape);

        Self { inner }
    }

    pub fn shape_type(&self) -> ShapeType {
        self.inner.ShapeType().into()
    }

    #[must_use]
    pub fn fillet_edge(&self, radius: f64, edge: &Edge) -> Self {
        self.fillet_edges(radius, [edge])
    }

    #[must_use]
    pub fn variable_fillet_edge(
        &self,
        radius_values: impl IntoIterator<Item = (f64, f64)>,
        edge: &Edge,
    ) -> Self {
        self.variable_fillet_edges(radius_values, [edge])
    }

    #[must_use]
    pub fn chamfer_edge(&self, distance: f64, edge: &Edge) -> Self {
        self.chamfer_edges(distance, [edge])
    }

    #[must_use]
    pub fn fillet_edges<T: AsRef<Edge>>(
        &self,
        radius: f64,
        edges: impl IntoIterator<Item = T>,
    ) -> Self {
        let mut make_fillet = ffi::BRepFilletAPI_MakeFillet_ctor(&self.inner);

        for edge in edges.into_iter() {
            make_fillet.pin_mut().add_edge(radius, &edge.as_ref().inner);
        }

        Self::from_shape(make_fillet.pin_mut().Shape())
    }

    #[must_use]
    pub fn variable_fillet_edges<T: AsRef<Edge>>(
        &self,
        radius_values: impl IntoIterator<Item = (f64, f64)>,
        edges: impl IntoIterator<Item = T>,
    ) -> Self {
        let radius_values: Vec<_> = radius_values.into_iter().collect();
        let mut array = ffi::TColgp_Array1OfPnt2d_ctor(1, radius_values.len() as i32);

        for (index, (t, radius)) in radius_values.into_iter().enumerate() {
            array.pin_mut().SetValue(index as i32 + 1, &make_point2d(dvec2(t, radius)));
        }

        let mut make_fillet = ffi::BRepFilletAPI_MakeFillet_ctor(&self.inner);

        for edge in edges.into_iter() {
            make_fillet.pin_mut().variable_add_edge(&array, &edge.as_ref().inner);
        }

        Self::from_shape(make_fillet.pin_mut().Shape())
    }

    #[must_use]
    pub fn chamfer_edges<T: AsRef<Edge>>(
        &self,
        distance: f64,
        edges: impl IntoIterator<Item = T>,
    ) -> Self {
        let mut make_chamfer = ffi::BRepFilletAPI_MakeChamfer_ctor(&self.inner);

        for edge in edges.into_iter() {
            make_chamfer.pin_mut().add_edge(distance, &edge.as_ref().inner);
        }

        Self::from_shape(make_chamfer.pin_mut().Shape())
    }

    /// Performs fillet of `radius` on all edges of the shape
    #[must_use]
    pub fn fillet(&self, radius: f64) -> Self {
        self.fillet_edges(radius, self.edges())
    }

    /// Performs chamfer of `distance` on all edges of the shape
    #[must_use]
    pub fn chamfer(&self, distance: f64) -> Self {
        self.chamfer_edges(distance, self.edges())
    }

    #[must_use]
    pub fn subtract(&self, other: &Shape) -> BooleanShape {
        let mut cut_operation = ffi::BRepAlgoAPI_Cut_ctor(&self.inner, &other.inner);

        let edge_list = cut_operation.pin_mut().SectionEdges();
        let vec = ffi::shape_list_to_vector(edge_list);

        let mut new_edges = vec![];
        for shape in vec.iter() {
            let edge = ffi::TopoDS_cast_to_edge(shape);
            new_edges.push(Edge::from_edge(edge));
        }

        let shape = Self::from_shape(cut_operation.pin_mut().Shape());

        BooleanShape { shape, new_edges }
    }

    pub fn read_step(path: impl AsRef<Path>) -> Result<Self, Error> {
        let mut reader = ffi::STEPControl_Reader_ctor();

        let status = ffi::read_step(reader.pin_mut(), path.as_ref().to_string_lossy().to_string());

        if status != ffi::IFSelect_ReturnStatus::IFSelect_RetDone {
            return Err(Error::StepReadFailed);
        }

        reader.pin_mut().TransferRoots(&ffi::Message_ProgressRange_ctor());

        let inner = ffi::one_shape(&reader);

        Ok(Self { inner })
    }

    pub fn write_step(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        let mut writer = ffi::STEPControl_Writer_ctor();

        let status = ffi::transfer_shape(writer.pin_mut(), &self.inner);

        if status != ffi::IFSelect_ReturnStatus::IFSelect_RetDone {
            return Err(Error::StepWriteFailed);
        }

        let status = ffi::write_step(writer.pin_mut(), path.as_ref().to_string_lossy().to_string());

        if status != ffi::IFSelect_ReturnStatus::IFSelect_RetDone {
            return Err(Error::StepWriteFailed);
        }

        Ok(())
    }

    #[must_use]
    pub fn union(&self, other: &Shape) -> BooleanShape {
        let mut fuse_operation = ffi::BRepAlgoAPI_Fuse_ctor(&self.inner, &other.inner);
        let edge_list = fuse_operation.pin_mut().SectionEdges();
        let vec = ffi::shape_list_to_vector(edge_list);

        let mut new_edges = vec![];
        for shape in vec.iter() {
            let edge = ffi::TopoDS_cast_to_edge(shape);
            new_edges.push(Edge::from_edge(edge));
        }

        let shape = Self::from_shape(fuse_operation.pin_mut().Shape());

        BooleanShape { shape, new_edges }
    }

    pub fn write_stl<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        self.write_stl_with_tolerance(path, 0.001)
    }

    pub fn write_stl_with_tolerance<P: AsRef<Path>>(
        &self,
        path: P,
        triangulation_tolerance: f64,
    ) -> Result<(), Error> {
        let mut stl_writer = ffi::StlAPI_Writer_ctor();
        let mesher = Mesher::try_new(self, triangulation_tolerance)?;
        let success = ffi::write_stl(
            stl_writer.pin_mut(),
            mesher.inner.Shape(),
            path.as_ref().to_string_lossy().to_string(),
        );

        if success {
            Ok(())
        } else {
            Err(Error::StlWriteFailed)
        }
    }

    #[must_use]
    pub fn clean(&self) -> Self {
        let mut upgrader = ffi::ShapeUpgrade_UnifySameDomain_ctor(&self.inner, true, true, true);
        upgrader.pin_mut().AllowInternalEdges(false);
        upgrader.pin_mut().Build();

        Self::from_shape(upgrader.Shape())
    }

    pub fn set_global_translation(&mut self, translation: DVec3) {
        let mut transform = ffi::new_transform();
        let translation_vec = make_vec(translation);
        transform.pin_mut().set_translation_vec(&translation_vec);

        let location = ffi::TopLoc_Location_from_transform(&transform);

        self.inner.pin_mut().set_global_translation(&location, false);
    }

    pub fn mesh(&self) -> Result<Mesh, Error> {
        self.mesh_with_tolerance(0.01)
    }

    pub fn mesh_with_tolerance(&self, triangulation_tolerance: f64) -> Result<Mesh, Error> {
        let mesher = Mesher::try_new(self, triangulation_tolerance)?;
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
            let face = Face::from_face(&face);
            let point = ffi::BRepIntCurveSurface_Inter_point(&intersector);

            results.push((face, dvec3(point.X(), point.Y(), point.Z())));

            intersector.pin_mut().Next();
        }

        results
    }

    #[must_use]
    pub fn hollow<T: AsRef<Face>>(
        &self,
        offset: f64,
        faces_to_remove: impl IntoIterator<Item = T>,
    ) -> Self {
        let mut faces_list = ffi::new_list_of_shape();

        for face in faces_to_remove.into_iter() {
            ffi::shape_list_append_face(faces_list.pin_mut(), &face.as_ref().inner);
        }

        let mut solid_maker = ffi::BRepOffsetAPI_MakeThickSolid_ctor();
        ffi::MakeThickSolidByJoin(solid_maker.pin_mut(), &self.inner, &faces_list, offset, 0.001);

        Self::from_shape(solid_maker.pin_mut().Shape())
    }

    #[must_use]
    pub fn offset_surface(&self, offset: f64) -> Self {
        let faces_to_remove: [Face; 0] = [];
        self.hollow(offset, faces_to_remove)
    }
}
