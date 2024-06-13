use crate::{
    mesh::{Mesh, Mesher},
    primitives::{
        make_axis_1, make_axis_2, make_dir, make_point, make_point2d, make_vec, BooleanShape,
        Compound, Edge, EdgeIterator, Face, FaceIterator, ShapeType, Shell, Solid, Vertex, Wire,
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

impl From<&Vertex> for Shape {
    fn from(vertex: &Vertex) -> Self {
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

impl From<&Edge> for Shape {
    fn from(edge: &Edge) -> Self {
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

impl From<&Wire> for Shape {
    fn from(wire: &Wire) -> Self {
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

impl From<&Face> for Shape {
    fn from(face: &Face) -> Self {
        let shape = ffi::cast_face_to_shape(&face.inner);

        Self::from_shape(shape)
    }
}

impl From<Shell> for Shape {
    fn from(shell: Shell) -> Self {
        let shape = ffi::cast_shell_to_shape(&shell.inner);

        Self::from_shape(shape)
    }
}

impl From<&Shell> for Shape {
    fn from(shell: &Shell) -> Self {
        let shape = ffi::cast_shell_to_shape(&shell.inner);

        Self::from_shape(shape)
    }
}

impl From<Solid> for Shape {
    fn from(solid: Solid) -> Self {
        let shape = ffi::cast_solid_to_shape(&solid.inner);

        Self::from_shape(shape)
    }
}

impl From<&Solid> for Shape {
    fn from(solid: &Solid) -> Self {
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

impl From<&Compound> for Shape {
    fn from(compound: &Compound) -> Self {
        let shape = ffi::cast_compound_to_shape(&compound.inner);

        Self::from_shape(shape)
    }
}

impl From<BooleanShape> for Shape {
    fn from(boolean_shape: BooleanShape) -> Self {
        boolean_shape.shape
    }
}

pub struct SphereBuilder {
    center: DVec3,
    radius: f64,
    z_angle: f64,
}

impl SphereBuilder {
    pub fn build(self) -> Shape {
        let axis = make_axis_2(self.center, DVec3::Z);
        let mut make_shere = ffi::BRepPrimAPI_MakeSphere_ctor(&axis, self.radius, self.z_angle);

        Shape::from_shape(make_shere.pin_mut().Shape())
    }

    pub fn at(mut self, center: DVec3) -> Self {
        self.center = center;
        self
    }

    pub fn z_angle(mut self, z_angle: f64) -> Self {
        self.z_angle = z_angle;
        self
    }
}

pub struct ConeBuilder {
    pos: DVec3,
    height: f64,
    bottom_radius: f64,
    top_radius: f64,
    z_angle: f64,
}

impl ConeBuilder {
    pub fn build(self) -> Shape {
        let axis = make_axis_2(self.pos, DVec3::Z);
        let mut make_cone = ffi::BRepPrimAPI_MakeCone_ctor(
            &axis,
            self.bottom_radius,
            self.top_radius,
            self.height,
            self.z_angle,
        );

        Shape::from_shape(make_cone.pin_mut().Shape())
    }

    pub fn at(mut self, pos: DVec3) -> Self {
        self.pos = pos;
        self
    }

    pub fn bottom_radius(mut self, bottom_radius: f64) -> Self {
        self.bottom_radius = bottom_radius;
        self
    }

    pub fn top_radius(mut self, top_radius: f64) -> Self {
        self.top_radius = top_radius;
        self
    }

    pub fn height(mut self, height: f64) -> Self {
        self.height = height;
        self
    }

    pub fn z_angle(mut self, z_angle: f64) -> Self {
        self.z_angle = z_angle;
        self
    }
}

pub struct TorusBuilder {
    pos: DVec3,
    z_axis: DVec3,
    radius_1: f64,
    radius_2: f64,
    angle_1: f64,
    angle_2: f64,
    z_angle: f64,
}

impl TorusBuilder {
    pub fn build(self) -> Shape {
        let axis = make_axis_2(self.pos, self.z_axis);
        let mut make_torus = ffi::BRepPrimAPI_MakeTorus_ctor(
            &axis,
            self.radius_1,
            self.radius_2,
            self.angle_1,
            self.angle_2,
            self.z_angle,
        );

        Shape::from_shape(make_torus.pin_mut().Shape())
    }

    pub fn at(mut self, pos: DVec3) -> Self {
        self.pos = pos;
        self
    }

    pub fn z_axis(mut self, z_axis: DVec3) -> Self {
        self.z_axis = z_axis;
        self
    }

    pub fn radius_1(mut self, radius_1: f64) -> Self {
        self.radius_1 = radius_1;
        self
    }

    pub fn radius_2(mut self, radius_2: f64) -> Self {
        self.radius_2 = radius_2;
        self
    }

    pub fn angle_1(mut self, angle_1: f64) -> Self {
        self.angle_1 = angle_1;
        self
    }

    pub fn angle_2(mut self, angle_2: f64) -> Self {
        self.angle_2 = angle_2;
        self
    }

    pub fn z_angle(mut self, z_angle: f64) -> Self {
        self.z_angle = z_angle;
        self
    }
}

impl Shape {
    pub(crate) fn from_shape(shape: &ffi::TopoDS_Shape) -> Self {
        let inner = ffi::TopoDS_Shape_to_owned(shape);

        Self { inner }
    }

    /// Make a box with one corner at corner_1, and the opposite corner
    /// at corner_2.
    pub fn box_from_corners(corner_1: DVec3, corner_2: DVec3) -> Self {
        let min_corner = corner_1.min(corner_2);
        let max_corner = corner_1.max(corner_2);

        let point = ffi::new_point(min_corner.x, min_corner.y, min_corner.z);
        let diff = max_corner - min_corner;
        let mut my_box = ffi::BRepPrimAPI_MakeBox_ctor(&point, diff.x, diff.y, diff.z);

        Self::from_shape(my_box.pin_mut().Shape())
    }

    /// Make a box with `width` (x), `depth` (y), and `height` (z)
    /// centered around the origin.
    pub fn box_centered(width: f64, depth: f64, height: f64) -> Self {
        let half_width = width / 2.0;
        let half_depth = depth / 2.0;
        let half_height = height / 2.0;

        let corner_1 = dvec3(-half_width, -half_depth, -half_height);
        let corner_2 = dvec3(half_width, half_depth, half_height);
        Self::box_from_corners(corner_1, corner_2)
    }

    /// Make a box with `width` (x), `depth` (y), and `height` (z)
    /// extending into the positive axes
    pub fn box_with_dimensions(width: f64, depth: f64, height: f64) -> Self {
        let corner_1 = DVec3::ZERO;
        let corner_2 = dvec3(width, depth, height);
        Self::box_from_corners(corner_1, corner_2)
    }

    /// Make a cube with side length of `size`
    /// extending into the positive axes
    pub fn cube(size: f64) -> Self {
        Self::box_with_dimensions(size, size, size)
    }

    /// Make a centered cube with side length of `size`
    pub fn cube_centered(size: f64) -> Self {
        Self::box_centered(size, size, size)
    }

    /// Make a cylinder with base at point `p`, radius `r`, and height `h`.
    /// Extends from `p` along axis `dir`.
    pub fn cylinder(p: DVec3, r: f64, dir: DVec3, h: f64) -> Self {
        let cylinder_coord_system = make_axis_2(p, dir);
        let mut cylinder = ffi::BRepPrimAPI_MakeCylinder_ctor(&cylinder_coord_system, r, h);

        Self::from_shape(cylinder.pin_mut().Shape())
    }

    /// Make a "default" cylinder with radius `r` and height `h`.
    /// The base is at the coordinate origin, and extends along the Z axis.
    pub fn cylinder_radius_height(r: f64, h: f64) -> Self {
        Self::cylinder(DVec3::ZERO, r, DVec3::Z, h)
    }

    /// Make a cylinder from start point `p1` and end point `p2`,
    /// with radius `r`.
    pub fn cylinder_from_points(p1: DVec3, p2: DVec3, r: f64) -> Self {
        let dir = p2 - p1;
        Self::cylinder(p1, r, dir, dir.length())
    }

    /// Make a cylinder centered at point `p`, with radius `r`, and height `h`.
    /// Extends along axis `dir`.
    pub fn cylinder_centered(p: DVec3, r: f64, dir: DVec3, h: f64) -> Self {
        let p = p - (dir.normalize() * (h / 2.0));
        Self::cylinder(p, r, dir, h)
    }

    pub fn sphere(radius: f64) -> SphereBuilder {
        SphereBuilder { center: DVec3::ZERO, radius, z_angle: std::f64::consts::TAU }
    }

    pub fn cone() -> ConeBuilder {
        ConeBuilder {
            pos: DVec3::ZERO,
            height: 1.0,
            bottom_radius: 1.0,
            top_radius: 0.0,
            z_angle: std::f64::consts::TAU,
        }
    }

    pub fn torus() -> TorusBuilder {
        TorusBuilder {
            pos: DVec3::ZERO,
            z_axis: DVec3::Z,
            radius_1: 20.0,
            radius_2: 10.0,
            angle_1: -std::f64::consts::PI,
            angle_2: std::f64::consts::PI,
            z_angle: std::f64::consts::TAU,
        }
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

    #[must_use]
    pub fn intersect(&self, other: &Shape) -> BooleanShape {
        let mut fuse_operation = ffi::BRepAlgoAPI_Common_ctor(&self.inner, &other.inner);
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
    pub fn faces_along_line(&self, line_origin: DVec3, line_dir: DVec3) -> Vec<LineFaceHitPoint> {
        let mut intersector = ffi::BRepIntCurveSurface_Inter_ctor();
        let tolerance = 0.0001;
        intersector.pin_mut().Init(
            &self.inner,
            &ffi::gp_Lin_ctor(&make_point(line_origin), &make_dir(line_dir)),
            tolerance,
        );

        let mut results = vec![];

        while intersector.More() {
            let face = ffi::BRepIntCurveSurface_Inter_face(&intersector);
            let face = Face::from_face(&face);
            let point = ffi::BRepIntCurveSurface_Inter_point(&intersector);

            results.push(LineFaceHitPoint {
                face,
                t: intersector.W(),
                u: intersector.U(),
                v: intersector.V(),
                point: dvec3(point.X(), point.Y(), point.Z()),
            });

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

    /// Drill a cylindrical hole along the line defined by point `p`
    /// and direction `dir`, with `radius`.
    #[must_use]
    pub fn drill_hole(&self, p: DVec3, dir: DVec3, radius: f64) -> Self {
        let hole_axis = make_axis_1(p, dir);

        let mut make_hole = ffi::BRepFeat_MakeCylindricalHole_ctor();
        make_hole.pin_mut().Init(&self.inner, &hole_axis);

        make_hole.pin_mut().Perform(radius);
        make_hole.pin_mut().Build();

        Self::from_shape(make_hole.pin_mut().Shape())
    }
}

/// Information about a point where a line hits (i.e. intersects) a face
pub struct LineFaceHitPoint {
    /// The face that is hit
    pub face: Face,
    /// The T parameter along the line
    pub t: f64,
    /// The U parameter on the face
    pub u: f64,
    /// The V parameter on the face
    pub v: f64,
    /// The intersection point
    pub point: DVec3,
}

pub struct ChamferMaker {
    inner: UniquePtr<ffi::BRepFilletAPI_MakeChamfer>,
}

impl ChamferMaker {
    pub fn new(shape: &Shape) -> Self {
        let make_chamfer = ffi::BRepFilletAPI_MakeChamfer_ctor(&shape.inner);

        Self { inner: make_chamfer }
    }

    pub fn add_edge(&mut self, distance: f64, edge: &Edge) {
        self.inner.pin_mut().add_edge(distance, &edge.inner);
    }

    pub fn build(mut self) -> Shape {
        Shape::from_shape(self.inner.pin_mut().Shape())
    }
}
