use crate::{
    adhoc::AdHocShape,
    angle::{Angle, ToAngle},
    workplane::Workplane,
    Error,
};
use cxx::UniquePtr;
use glam::{dvec2, dvec3, DVec2, DVec3};
use opencascade_sys::ffi::{self, IFSelect_ReturnStatus};
use std::{
    ops::{Deref, DerefMut},
    path::Path,
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ShapeType {
    /// Abstract topological data structure describes a basic entity.
    Shape,

    /// A zero-dimensional shape corresponding to a point in geometry.
    Vertex,

    /// A single dimensional shape correspondingto a curve, and bound
    /// by a vertex at each extremity.
    Edge,

    /// A sequence of edges connected by their vertices. It can be open
    /// or closed depending on whether the edges are linked or not.
    Wire,

    /// Part of a plane (in 2D geometry) or a surface(in 3D geometry)
    /// bounded by a closed wire. Its geometry is constrained (trimmed)
    /// by contours.
    Face,

    /// A set of faces connected by some of the
    /// edges of their wire boundaries. A shell can be open or closed.
    Shell,

    /// A part of 3D space bounded by shells.
    Solid,

    /// A set of solids connected by their faces. This expands
    /// the notions of Wire and Shell to solids.
    CompoundSolid,

    /// A group of any of the shapes below.
    Compound,
}

impl From<ffi::TopAbs_ShapeEnum> for ShapeType {
    fn from(shape_enum: ffi::TopAbs_ShapeEnum) -> Self {
        match shape_enum {
            ffi::TopAbs_ShapeEnum::TopAbs_SHAPE => ShapeType::Shape,
            ffi::TopAbs_ShapeEnum::TopAbs_VERTEX => ShapeType::Vertex,
            ffi::TopAbs_ShapeEnum::TopAbs_EDGE => ShapeType::Edge,
            ffi::TopAbs_ShapeEnum::TopAbs_WIRE => ShapeType::Wire,
            ffi::TopAbs_ShapeEnum::TopAbs_FACE => ShapeType::Face,
            ffi::TopAbs_ShapeEnum::TopAbs_SHELL => ShapeType::Shell,
            ffi::TopAbs_ShapeEnum::TopAbs_SOLID => ShapeType::Solid,
            ffi::TopAbs_ShapeEnum::TopAbs_COMPSOLID => ShapeType::CompoundSolid,
            ffi::TopAbs_ShapeEnum::TopAbs_COMPOUND => ShapeType::Compound,
            ffi::TopAbs_ShapeEnum { repr } => panic!("Unexpected shape type: {repr}"),
        }
    }
}

pub trait IntoShape {
    fn into_shape(self) -> Shape;
}

impl<T: Into<Shape>> IntoShape for T {
    fn into_shape(self) -> Shape {
        self.into()
    }
}

pub fn make_point(p: DVec3) -> UniquePtr<ffi::gp_Pnt> {
    ffi::new_point(p.x, p.y, p.z)
}

pub fn make_dir(p: DVec3) -> UniquePtr<ffi::gp_Dir> {
    ffi::gp_Dir_ctor(p.x, p.y, p.z)
}

pub fn make_vec(vec: DVec3) -> UniquePtr<ffi::gp_Vec> {
    ffi::new_vec(vec.x, vec.y, vec.z)
}

pub fn make_axis_1(origin: DVec3, dir: DVec3) -> UniquePtr<ffi::gp_Ax1> {
    ffi::gp_Ax1_ctor(&make_point(origin), &make_dir(dir))
}

pub fn make_axis_2(origin: DVec3, dir: DVec3) -> UniquePtr<ffi::gp_Ax2> {
    ffi::gp_Ax2_ctor(&make_point(origin), &make_dir(dir))
}

pub struct Vertex {
    inner: UniquePtr<ffi::TopoDS_Vertex>,
}

// You'll see several of these `impl AsRef` blocks for the various primitive
// geometry types. This is for functions which take an Iterator of primitives
// which are either owned or borrowed values. The general pattern looks like this:
//
//     pub fn do_something_with_edges<T: AsRef<Edge>>(edges: impl IntoIterator<Item = T>) {
//         for edge in edges.into_iter() {
//             let edge_ref = edge.as_ref();
//             // Do something with edge_ref
//         }
//     }
impl AsRef<Vertex> for Vertex {
    fn as_ref(&self) -> &Vertex {
        self
    }
}

impl Vertex {
    pub fn new(point: DVec3) -> Self {
        let mut make_vertex = ffi::BRepBuilderAPI_MakeVertex_gp_Pnt(&make_point(point));
        let vertex = make_vertex.pin_mut().Vertex();
        let inner = ffi::TopoDS_Vertex_to_owned(vertex);

        Self { inner }
    }
}

pub struct Edge {
    inner: UniquePtr<ffi::TopoDS_Edge>,
}

impl AsRef<Edge> for Edge {
    fn as_ref(&self) -> &Edge {
        self
    }
}

impl Edge {
    pub fn segment(p1: DVec3, p2: DVec3) -> Self {
        let mut make_edge =
            ffi::BRepBuilderAPI_MakeEdge_gp_Pnt_gp_Pnt(&make_point(p1), &make_point(p2));
        let edge = make_edge.pin_mut().Edge();
        let inner = ffi::TopoDS_Edge_to_owned(edge);

        Self { inner }
    }

    pub fn circle(center: DVec3, normal: DVec3, radius: f64) -> Self {
        let axis = make_axis_2(center, normal);

        let make_circle = ffi::gp_Circ_ctor(&axis, radius);

        let mut make_edge = ffi::BRepBuilderAPI_MakeEdge_circle(&make_circle);

        let edge = make_edge.pin_mut().Edge();
        let inner = ffi::TopoDS_Edge_to_owned(edge);

        Self { inner }
    }

    pub fn ellipse() {}

    pub fn spline() {}

    pub fn arc(p1: DVec3, p2: DVec3, p3: DVec3) -> Self {
        let make_arc = ffi::GC_MakeArcOfCircle_point_point_point(
            &make_point(p1),
            &make_point(p2),
            &make_point(p3),
        );

        let mut make_edge = ffi::BRepBuilderAPI_MakeEdge_HandleGeomCurve(
            &ffi::new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(&ffi::GC_MakeArcOfCircle_Value(
                &make_arc,
            )),
        );

        let edge = make_edge.pin_mut().Edge();
        let inner = ffi::TopoDS_Edge_to_owned(edge);

        Self { inner }
    }

    pub fn start_point(&self) -> DVec3 {
        let curve = ffi::BRepAdaptor_Curve_ctor(&self.inner);
        let start_param = curve.FirstParameter();
        let point = ffi::BRepAdaptor_Curve_value(&curve, start_param);

        dvec3(point.X(), point.Y(), point.Z())
    }

    pub fn end_point(&self) -> DVec3 {
        let curve = ffi::BRepAdaptor_Curve_ctor(&self.inner);
        let last_param = curve.LastParameter();
        let point = ffi::BRepAdaptor_Curve_value(&curve, last_param);

        dvec3(point.X(), point.Y(), point.Z())
    }

    pub fn approximation_segments(&self) -> ApproximationSegmentIterator {
        let adaptor_curve = ffi::BRepAdaptor_Curve_ctor(&self.inner);
        let approximator = ffi::GCPnts_TangentialDeflection_ctor(&adaptor_curve, 0.1, 0.1);

        ApproximationSegmentIterator { count: 1, approximator }
    }

    pub fn tangent_arc(_p1: DVec3, _tangent: DVec3, _p3: DVec3) {}
}

pub struct ApproximationSegmentIterator {
    count: usize,
    approximator: UniquePtr<ffi::GCPnts_TangentialDeflection>,
}

impl Iterator for ApproximationSegmentIterator {
    type Item = DVec3;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count <= self.approximator.NbPoints() as usize {
            let point =
                ffi::GCPnts_TangentialDeflection_Value(&self.approximator, self.count as i32);

            self.count += 1;
            Some(dvec3(point.X(), point.Y(), point.Z()))
        } else {
            None
        }
    }
}

pub struct Wire {
    inner: UniquePtr<ffi::TopoDS_Wire>,
}

impl AsRef<Wire> for Wire {
    fn as_ref(&self) -> &Wire {
        self
    }
}

impl Wire {
    pub fn from_edges<'a>(edges: impl IntoIterator<Item = &'a Edge>) -> Self {
        let mut make_wire = ffi::BRepBuilderAPI_MakeWire_ctor();

        for edge in edges.into_iter() {
            make_wire.pin_mut().add_edge(&edge.inner);
        }

        let wire = make_wire.pin_mut().Wire();
        let inner = ffi::TopoDS_Wire_to_owned(wire);

        Self { inner }
    }

    pub fn from_wires<'a>(wires: impl IntoIterator<Item = &'a Wire>) -> Self {
        let mut make_wire = ffi::BRepBuilderAPI_MakeWire_ctor();

        for wire in wires.into_iter() {
            make_wire.pin_mut().add_wire(&wire.inner);
        }

        let wire = make_wire.pin_mut().Wire();
        let inner = ffi::TopoDS_Wire_to_owned(wire);

        Self { inner }
    }

    pub fn mirror_along_axis(&self, axis_origin: DVec3, axis_dir: DVec3) -> Self {
        let axis_dir = make_dir(axis_dir);
        let axis = ffi::gp_Ax1_ctor(&make_point(axis_origin), &axis_dir);

        let mut transform = ffi::new_transform();

        transform.pin_mut().set_mirror_axis(&axis);

        let wire_shape = ffi::cast_wire_to_shape(&self.inner);

        let mut brep_transform = ffi::BRepBuilderAPI_Transform_ctor(wire_shape, &transform, false);

        let mirrored_shape = brep_transform.pin_mut().Shape();
        let mirrored_wire = ffi::TopoDS_cast_to_wire(mirrored_shape);
        let inner = ffi::TopoDS_Wire_to_owned(mirrored_wire);

        Self { inner }
    }

    pub fn rect(width: f64, height: f64) -> Self {
        let half_width = width / 2.0;
        let half_height = height / 2.0;

        let p1 = dvec3(-half_width, half_height, 0.0);
        let p2 = dvec3(half_width, half_height, 0.0);
        let p3 = dvec3(half_width, -half_height, 0.0);
        let p4 = dvec3(-half_width, -half_height, 0.0);

        let top = Edge::segment(p1, p2);
        let right = Edge::segment(p2, p3);
        let bottom = Edge::segment(p3, p4);
        let left = Edge::segment(p4, p1);

        Self::from_edges([&top, &right, &bottom, &left])
    }

    pub fn fillet(&mut self, radius: f64) {
        // Create a face from this wire
        let mut face: Face = Face::from_wire(self);
        face.fillet(radius);
        let wire = ffi::outer_wire(&face.inner);

        self.inner = wire;
    }

    /// Chamfer the wire edges at each vertex by a given distance.
    pub fn chamfer(&mut self, distance_1: f64) {
        let mut face = Face::from_wire(self);
        face.chamfer(distance_1);

        let wire = ffi::outer_wire(&face.inner);

        self.inner = wire;
    }

    pub fn translate(&mut self, offset: DVec3) {
        self.transform(offset, dvec3(1.0, 0.0, 0.0), 0.degrees());
    }

    pub fn transform(&mut self, translation: DVec3, rotation_axis: DVec3, angle: Angle) {
        let mut transform = ffi::new_transform();
        let rotation_axis_vec =
            ffi::gp_Ax1_ctor(&make_point(DVec3::ZERO), &make_dir(rotation_axis));
        let translation_vec = make_vec(translation);

        transform.pin_mut().SetRotation(&rotation_axis_vec, angle.radians());
        transform.pin_mut().set_translation_vec(&translation_vec);
        let location = ffi::TopLoc_Location_from_transform(&transform);

        let wire_shape = ffi::cast_wire_to_shape(&self.inner);
        let mut wire_shape = ffi::TopoDS_Shape_to_owned(wire_shape);

        let raise_exception = false;
        wire_shape.pin_mut().translate(&location, raise_exception);

        let translated_wire = ffi::TopoDS_cast_to_wire(&wire_shape);
        self.inner = ffi::TopoDS_Wire_to_owned(translated_wire);
    }

    pub fn to_face(self) -> Face {
        let only_plane = false;
        let make_face = ffi::BRepBuilderAPI_MakeFace_wire(&self.inner, only_plane);

        let face = make_face.Face();
        let inner = ffi::TopoDS_Face_to_owned(face);

        Face { inner }
    }

    // Create a closure-based API
    pub fn freeform() {}
}

pub struct Face {
    inner: UniquePtr<ffi::TopoDS_Face>,
}

impl AsRef<Face> for Face {
    fn as_ref(&self) -> &Face {
        self
    }
}

impl Face {
    pub fn from_wire(wire: &Wire) -> Self {
        let only_plane = false;
        let make_face = ffi::BRepBuilderAPI_MakeFace_wire(&wire.inner, only_plane);

        let face = make_face.Face();
        let inner = ffi::TopoDS_Face_to_owned(face);

        Self { inner }
    }

    pub fn extrude(&self, dir: DVec3) -> Solid {
        let prism_vec = make_vec(dir);

        let copy = false;
        let canonize = true;

        let inner_shape = ffi::cast_face_to_shape(&self.inner);
        let mut make_solid =
            ffi::BRepPrimAPI_MakePrism_ctor(inner_shape, &prism_vec, copy, canonize);
        let extruded_shape = make_solid.pin_mut().Shape();
        let solid = ffi::TopoDS_cast_to_solid(extruded_shape);
        let inner = ffi::TopoDS_Solid_to_owned(solid);

        Solid { inner }
    }

    pub fn extrude_to_face(&self, shape_with_face: &Shape, face: &Face) -> Shape {
        let profile_base = &self.inner;
        let sketch_base = ffi::TopoDS_Face_ctor();
        let angle = 0.0;
        let fuse = 1; // 0 = subtractive, 1 = additive
        let modify = false;

        let mut make_prism = ffi::BRepFeat_MakeDPrism_ctor(
            &shape_with_face.inner,
            profile_base,
            &sketch_base,
            angle,
            fuse,
            modify,
        );

        let until_face = ffi::cast_face_to_shape(&face.inner);
        make_prism.pin_mut().perform_until_face(until_face);

        let extruded_shape = make_prism.pin_mut().Shape();
        let inner = ffi::TopoDS_Shape_to_owned(extruded_shape);

        Shape { inner }
    }

    pub fn subtractive_extrude(&self, shape_with_face: &Shape, height: f64) -> Shape {
        let profile_base = &self.inner;
        let sketch_base = ffi::TopoDS_Face_ctor();
        let angle = 0.0;
        let fuse = 1; // 0 = subtractive, 1 = additive
        let modify = false;

        let mut make_prism = ffi::BRepFeat_MakeDPrism_ctor(
            &shape_with_face.inner,
            profile_base,
            &sketch_base,
            angle,
            fuse,
            modify,
        );

        make_prism.pin_mut().perform_with_height(height);

        let extruded_shape = make_prism.pin_mut().Shape();
        let inner = ffi::TopoDS_Shape_to_owned(extruded_shape);

        Shape { inner }
    }

    pub fn revolve(&self, origin: DVec3, axis: DVec3, angle: Option<Angle>) -> Solid {
        let revol_vec = make_axis_1(origin, axis);

        let angle = angle.map(Angle::radians).unwrap_or(std::f64::consts::PI * 2.0);
        let copy = false;

        let inner_shape = ffi::cast_face_to_shape(&self.inner);
        let mut make_solid = ffi::BRepPrimAPI_MakeRevol_ctor(inner_shape, &revol_vec, angle, copy);
        let revolved_shape = make_solid.pin_mut().Shape();
        let solid = ffi::TopoDS_cast_to_solid(revolved_shape);
        let inner = ffi::TopoDS_Solid_to_owned(solid);

        Solid { inner }
    }

    /// Fillets the face edges by a given radius at each vertex
    pub fn fillet(&mut self, radius: f64) {
        let mut make_fillet = ffi::BRepFilletAPI_MakeFillet2d_ctor(&self.inner);

        let face_shape = ffi::cast_face_to_shape(&self.inner);

        // We use a shape map here to avoid duplicates.
        let mut shape_map = ffi::new_indexed_map_of_shape();
        ffi::map_shapes(face_shape, ffi::TopAbs_ShapeEnum::TopAbs_VERTEX, shape_map.pin_mut());

        for i in 1..=shape_map.Extent() {
            let vertex = ffi::TopoDS_cast_to_vertex(shape_map.FindKey(i));
            ffi::BRepFilletAPI_MakeFillet2d_add_fillet(make_fillet.pin_mut(), vertex, radius);
        }

        make_fillet.pin_mut().Build(&ffi::Message_ProgressRange_ctor());

        let result_shape = make_fillet.pin_mut().Shape();
        let result_face = ffi::TopoDS_cast_to_face(result_shape);

        self.inner = ffi::TopoDS_Face_to_owned(result_face);
    }

    /// Chamfer the wire edges at each vertex by a given distance
    pub fn chamfer(&mut self, distance_1: f64) {
        // TODO - Support asymmetric chamfers.
        let distance_2 = distance_1;

        let face_shape = ffi::cast_face_to_shape(&self.inner);

        let mut make_fillet = ffi::BRepFilletAPI_MakeFillet2d_ctor(&self.inner);

        let mut vertex_map = ffi::new_indexed_map_of_shape();
        ffi::map_shapes(face_shape, ffi::TopAbs_ShapeEnum::TopAbs_VERTEX, vertex_map.pin_mut());

        // Get map of vertices to edges so we can find the edges connected to each vertex.
        let mut data_map = ffi::new_indexed_data_map_of_shape_list_of_shape();
        ffi::map_shapes_and_ancestors(
            face_shape,
            ffi::TopAbs_ShapeEnum::TopAbs_VERTEX,
            ffi::TopAbs_ShapeEnum::TopAbs_EDGE,
            data_map.pin_mut(),
        );

        // Chamfer at vertex of all edges.
        for i in 1..=vertex_map.Extent() {
            let edges = ffi::shape_list_to_vector(data_map.FindFromIndex(i));
            let edge_1 = edges.get(0).expect("Vertex has no edges");
            let edge_2 = edges.get(1).expect("Vertex has only one edge");
            ffi::BRepFilletAPI_MakeFillet2d_add_chamfer(
                make_fillet.pin_mut(),
                ffi::TopoDS_cast_to_edge(edge_1),
                ffi::TopoDS_cast_to_edge(edge_2),
                distance_1,
                distance_2,
            );
        }

        let filleted_shape = make_fillet.pin_mut().Shape();
        let result_face = ffi::TopoDS_cast_to_face(filleted_shape);

        self.inner = ffi::TopoDS_Face_to_owned(result_face);
    }

    pub fn edges(&self) -> EdgeIterator {
        let explorer = ffi::TopExp_Explorer_ctor(
            ffi::cast_face_to_shape(&self.inner),
            ffi::TopAbs_ShapeEnum::TopAbs_EDGE,
        );

        EdgeIterator { explorer }
    }

    pub fn center_of_mass(&self) -> DVec3 {
        let mut props = ffi::GProp_GProps_ctor();

        let inner_shape = ffi::cast_face_to_shape(&self.inner);
        ffi::BRepGProp_SurfaceProperties(inner_shape, props.pin_mut());

        let center = ffi::GProp_GProps_CentreOfMass(&props);

        dvec3(center.X(), center.Y(), center.Z())
    }

    pub fn normal_at(&self, pos: DVec3) -> DVec3 {
        let surface = ffi::BRep_Tool_Surface(&self.inner);
        let projector = ffi::GeomAPI_ProjectPointOnSurf_ctor(&make_point(pos), &surface);
        let mut u: f64 = 0.0;
        let mut v: f64 = 0.0;

        projector.LowerDistanceParameters(&mut u, &mut v);

        let mut p = ffi::new_point(0.0, 0.0, 0.0);
        let mut normal = ffi::new_vec(0.0, 1.0, 0.0);

        let face = ffi::BRepGProp_Face_ctor(&self.inner);
        face.Normal(u, v, p.pin_mut(), normal.pin_mut());

        dvec3(normal.X(), normal.Y(), normal.Z())
    }

    pub fn normal_at_center(&self) -> DVec3 {
        let center = self.center_of_mass();
        self.normal_at(center)
    }

    pub fn workplane(&self) -> Workplane {
        const NORMAL_DIFF_TOLERANCE: f64 = 0.0001;

        let center = self.center_of_mass();
        let normal = self.normal_at(center);
        let mut x_dir = dvec3(0.0, 0.0, 1.0).cross(normal);

        if x_dir.length() < NORMAL_DIFF_TOLERANCE {
            // The normal of this face is too close to the same direction
            // as the global Z axis. Use the global X axis for X instead.
            x_dir = dvec3(1.0, 0.0, 0.0);
        }

        let mut workplane = Workplane::new(x_dir, normal);
        workplane.set_translation(center);
        workplane
    }

    pub fn union(&self, other: &Face) -> CompoundFace {
        let inner_shape = ffi::cast_face_to_shape(&self.inner);
        let other_inner_shape = ffi::cast_face_to_shape(&other.inner);

        let mut fuse_operation = ffi::BRepAlgoAPI_Fuse_ctor(inner_shape, other_inner_shape);

        let fuse_shape = fuse_operation.pin_mut().Shape();

        let compound = ffi::TopoDS_cast_to_compound(fuse_shape);
        let inner = ffi::TopoDS_Compound_to_owned(compound);

        CompoundFace { inner }
    }

    pub fn orientation(&self) -> FaceOrientation {
        FaceOrientation::from(self.inner.Orientation())
    }

    pub fn from_shape(shape: &Shape) -> Self {
        let face = ffi::TopoDS_cast_to_face(&shape.inner);
        let inner = ffi::TopoDS_Face_to_owned(face);

        Self { inner }
    }
}

pub struct CompoundFace {
    inner: UniquePtr<ffi::TopoDS_Compound>,
}

impl AsRef<CompoundFace> for CompoundFace {
    fn as_ref(&self) -> &CompoundFace {
        self
    }
}

impl CompoundFace {
    pub fn clean(&mut self) -> Self {
        let inner = ffi::cast_compound_to_shape(&self.inner);
        let inner = ffi::TopoDS_Shape_to_owned(inner);
        let mut shape = Shape { inner };

        shape.clean();

        let inner = ffi::TopoDS_cast_to_compound(&shape.inner);
        let inner = ffi::TopoDS_Compound_to_owned(inner);

        Self { inner }
    }

    pub fn extrude(&self, dir: DVec3) -> Shape {
        let prism_vec = make_vec(dir);

        let copy = false;
        let canonize = true;

        let inner_shape = ffi::cast_compound_to_shape(&self.inner);

        let mut make_solid =
            ffi::BRepPrimAPI_MakePrism_ctor(inner_shape, &prism_vec, copy, canonize);
        let extruded_shape = make_solid.pin_mut().Shape();
        let inner = ffi::TopoDS_Shape_to_owned(extruded_shape);

        Shape { inner }
    }

    pub fn revolve(&self, origin: DVec3, axis: DVec3, angle: Option<Angle>) -> Shape {
        let revol_axis = make_axis_1(origin, axis);

        let angle = angle.map(Angle::radians).unwrap_or(std::f64::consts::PI * 2.0);
        let copy = false;

        let inner_shape = ffi::cast_compound_to_shape(&self.inner);

        let mut make_solid = ffi::BRepPrimAPI_MakeRevol_ctor(inner_shape, &revol_axis, angle, copy);
        let revolved_shape = make_solid.pin_mut().Shape();
        let inner = ffi::TopoDS_Shape_to_owned(revolved_shape);

        Shape { inner }
    }

    pub fn set_global_translation(&mut self, translation: DVec3) {
        let inner = ffi::cast_compound_to_shape(&self.inner);
        let inner = ffi::TopoDS_Shape_to_owned(inner);
        let mut shape = Shape { inner };

        shape.set_global_translation(translation);

        let compound = ffi::TopoDS_cast_to_compound(&shape.inner);
        let compound = ffi::TopoDS_Compound_to_owned(compound);

        self.inner = compound;
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FaceOrientation {
    Forward,
    Reversed,
    Internal,
    External,
}

impl From<ffi::TopAbs_Orientation> for FaceOrientation {
    fn from(orientation: ffi::TopAbs_Orientation) -> Self {
        match orientation {
            ffi::TopAbs_Orientation::TopAbs_FORWARD => Self::Forward,
            ffi::TopAbs_Orientation::TopAbs_REVERSED => Self::Reversed,
            ffi::TopAbs_Orientation::TopAbs_INTERNAL => Self::Internal,
            ffi::TopAbs_Orientation::TopAbs_EXTERNAL => Self::External,
            ffi::TopAbs_Orientation { repr } => {
                panic!("TopAbs_Orientation had an unrepresentable value: {repr}")
            },
        }
    }
}

pub struct Shell {
    _inner: UniquePtr<ffi::TopoDS_Shell>,
}

impl AsRef<Shell> for Shell {
    fn as_ref(&self) -> &Shell {
        self
    }
}

pub struct Solid {
    inner: UniquePtr<ffi::TopoDS_Solid>,
}

impl AsRef<Solid> for Solid {
    fn as_ref(&self) -> &Solid {
        self
    }
}

impl Solid {
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

pub struct Compound {
    inner: UniquePtr<ffi::TopoDS_Compound>,
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
}

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
}

/// The result of running a boolean operation (union, subtraction, intersection)
/// on two shapes.
pub struct BooleanShape {
    pub shape: Shape,
    pub new_edges: Vec<Edge>,
}

impl Deref for BooleanShape {
    type Target = Shape;

    fn deref(&self) -> &Self::Target {
        &self.shape
    }
}

impl DerefMut for BooleanShape {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.shape
    }
}

impl BooleanShape {
    pub fn new_edges(&self) -> impl Iterator<Item = &Edge> {
        self.new_edges.iter()
    }

    pub fn fillet_new_edges(&mut self, radius: f64) {
        self.shape.fillet_edges(radius, &self.new_edges);
    }

    pub fn chamfer_new_edges(&mut self, distance: f64) {
        self.shape.chamfer_edges(distance, &self.new_edges);
    }
}

struct Mesher {
    inner: UniquePtr<ffi::BRepMesh_IncrementalMesh>,
}

impl Mesher {
    fn new(shape: &Shape) -> Self {
        let inner = ffi::BRepMesh_IncrementalMesh_ctor(&shape.inner, 0.01);

        if !inner.IsDone() {
            // TODO(bschwind) - Add proper Error type and return Result.
            panic!("Call to ffi::BRepMesh_IncrementalMesh_ctor failed");
        }

        Self { inner }
    }

    fn mesh(mut self) -> Mesh {
        let mut vertices = vec![];
        let mut uvs = vec![];
        let mut normals = vec![];
        let mut indices = vec![];

        let triangulated_shape = ffi::TopoDS_Shape_to_owned(self.inner.pin_mut().Shape());
        let triangulated_shape = Shape { inner: triangulated_shape };

        for face in triangulated_shape.faces() {
            let mut location = ffi::TopLoc_Location_ctor();

            let triangulation_handle =
                ffi::BRep_Tool_Triangulation(&face.inner, location.pin_mut());

            let Ok(triangulation) = ffi::Handle_Poly_Triangulation_Get(&triangulation_handle)
            else {
                // TODO(bschwind) - Do better error handling, use Results.
                println!("Encountered a face with no triangulation");
                continue;
            };

            let index_offset = vertices.len();
            let face_point_count = triangulation.NbNodes();

            for i in 1..=face_point_count {
                let mut point = ffi::Poly_Triangulation_Node(triangulation, i);
                point.pin_mut().Transform(&ffi::TopLoc_Location_Transformation(&location));
                vertices.push(dvec3(point.X(), point.Y(), point.Z()));
            }

            let mut u_min = f64::INFINITY;
            let mut v_min = f64::INFINITY;

            let mut u_max = f64::NEG_INFINITY;
            let mut v_max = f64::NEG_INFINITY;

            for i in 1..=(face_point_count) {
                let uv = ffi::Poly_Triangulation_UV(triangulation, i);
                let (u, v) = (uv.X(), uv.Y());

                u_min = u_min.min(u);
                v_min = v_min.min(v);

                u_max = u_max.max(u);
                v_max = v_max.max(v);

                uvs.push(dvec2(u, v));
            }

            // Normalize the newly added UV coordinates.
            for uv in &mut uvs[index_offset..(index_offset + face_point_count as usize)] {
                uv.x = (uv.x - u_min) / (u_max - u_min);
                uv.y = (uv.y - v_min) / (v_max - v_min);

                if face.orientation() != FaceOrientation::Forward {
                    uv.x = 1.0 - uv.x;
                }
            }

            // Add in the normals.
            // TODO(bschwind) - Use `location` to transform the normals.
            let normal_array = ffi::TColgp_Array1OfDir_ctor(0, face_point_count);

            ffi::compute_normals(&face.inner, &triangulation_handle);

            // TODO(bschwind) - Why do we start at 1 here?
            for i in 1..(normal_array.Length() as usize) {
                let normal = ffi::Poly_Triangulation_Normal(triangulation, i as i32);
                normals.push(dvec3(normal.X(), normal.Y(), normal.Z()));
            }

            for i in 1..=triangulation.NbTriangles() {
                let triangle = triangulation.Triangle(i);

                if face.orientation() == FaceOrientation::Forward {
                    indices.push(index_offset + triangle.Value(1) as usize - 1);
                    indices.push(index_offset + triangle.Value(2) as usize - 1);
                    indices.push(index_offset + triangle.Value(3) as usize - 1);
                } else {
                    indices.push(index_offset + triangle.Value(3) as usize - 1);
                    indices.push(index_offset + triangle.Value(2) as usize - 1);
                    indices.push(index_offset + triangle.Value(1) as usize - 1);
                }
            }
        }

        Mesh { vertices, uvs, normals, indices }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<DVec3>,
    pub uvs: Vec<DVec2>,
    pub normals: Vec<DVec3>,
    pub indices: Vec<usize>,
}

pub struct EdgeIterator {
    explorer: UniquePtr<ffi::TopExp_Explorer>,
}

impl Iterator for EdgeIterator {
    type Item = Edge;

    fn next(&mut self) -> Option<Self::Item> {
        if self.explorer.More() {
            let edge = ffi::TopoDS_cast_to_edge(self.explorer.Current());
            let inner = ffi::TopoDS_Edge_to_owned(edge);

            self.explorer.pin_mut().Next();

            Some(Edge { inner })
        } else {
            None
        }
    }
}

pub struct FaceIterator {
    explorer: UniquePtr<ffi::TopExp_Explorer>,
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    PosX,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
    Custom(DVec3),
}

impl Direction {
    pub fn normalized_vec(&self) -> DVec3 {
        match self {
            Self::PosX => DVec3::X,
            Self::NegX => DVec3::NEG_X,
            Self::PosY => DVec3::Y,
            Self::NegY => DVec3::NEG_Y,
            Self::PosZ => DVec3::Z,
            Self::NegZ => DVec3::NEG_Z,
            Self::Custom(dir) => dir.normalize(),
        }
    }
}

impl FaceIterator {
    pub fn farthest(self, direction: Direction) -> Face {
        self.try_farthest(direction).unwrap()
    }

    pub fn try_farthest(self, direction: Direction) -> Option<Face> {
        let normalized_dir = direction.normalized_vec();

        Iterator::max_by(self, |face_1, face_2| {
            let dist_1 = face_1.center_of_mass().dot(normalized_dir);
            let dist_2 = face_2.center_of_mass().dot(normalized_dir);

            PartialOrd::partial_cmp(&dist_1, &dist_2)
                .expect("Face center of masses should contain no NaNs")
        })
    }
}

impl Iterator for FaceIterator {
    type Item = Face;

    fn next(&mut self) -> Option<Self::Item> {
        if self.explorer.More() {
            let face = ffi::TopoDS_cast_to_face(self.explorer.Current());
            let inner = ffi::TopoDS_Face_to_owned(face);

            self.explorer.pin_mut().Next();

            Some(Face { inner })
        } else {
            None
        }
    }
}
