use crate::{gp_Ax2_ctor, workplane::Workplane, Error, TopoDS_Shape_to_owned};
use cxx::UniquePtr;
use glam::{dvec2, dvec3, DVec2, DVec3};
use opencascade_sys::ffi::{
    cast_compound_to_shape, cast_face_to_shape, cast_solid_to_shape, cast_wire_to_shape,
    gp_Ax1_ctor, gp_Ax2, gp_Circ_ctor, gp_Dir, gp_Dir_ctor, gp_Lin_ctor, gp_Pnt, gp_Vec,
    map_shapes, map_shapes_and_ancestors, new_HandleGeomCurve_from_HandleGeom_TrimmedCurve,
    new_indexed_data_map_of_shape_list_of_shape, new_indexed_map_of_shape, new_point,
    new_transform, new_vec, one_shape, outer_wire, read_step, shape_list_to_vector,
    triangulated_shape_normal, write_stl, BRepAdaptor_Curve_ctor, BRepAdaptor_Curve_value,
    BRepAlgoAPI_Cut_ctor, BRepAlgoAPI_Fuse_ctor, BRepBuilderAPI_MakeEdge_HandleGeomCurve,
    BRepBuilderAPI_MakeEdge_circle, BRepBuilderAPI_MakeEdge_gp_Pnt_gp_Pnt,
    BRepBuilderAPI_MakeFace_wire, BRepBuilderAPI_MakeVertex_gp_Pnt, BRepBuilderAPI_MakeWire_ctor,
    BRepBuilderAPI_Transform_ctor, BRepFeat_MakeDPrism_ctor, BRepFilletAPI_MakeChamfer_ctor,
    BRepFilletAPI_MakeFillet2d_add_chamfer, BRepFilletAPI_MakeFillet2d_add_fillet,
    BRepFilletAPI_MakeFillet2d_ctor, BRepFilletAPI_MakeFillet_ctor, BRepGProp_Face_ctor,
    BRepGProp_SurfaceProperties, BRepIntCurveSurface_Inter_ctor, BRepIntCurveSurface_Inter_face,
    BRepIntCurveSurface_Inter_point, BRepMesh_IncrementalMesh, BRepMesh_IncrementalMesh_ctor,
    BRepOffsetAPI_ThruSections_ctor, BRepPrimAPI_MakePrism_ctor, BRep_Tool_Surface,
    BRep_Tool_Triangulation, GCPnts_TangentialDeflection, GCPnts_TangentialDeflection_Value,
    GCPnts_TangentialDeflection_ctor, GC_MakeArcOfCircle_Value,
    GC_MakeArcOfCircle_point_point_point, GProp_GProps_CentreOfMass, GProp_GProps_ctor,
    GeomAPI_ProjectPointOnSurf_ctor, Handle_Poly_Triangulation_Get, Message_ProgressRange_ctor,
    Poly_Connect_ctor, Poly_Triangulation_Node, Poly_Triangulation_UV, STEPControl_Reader_ctor,
    ShapeUpgrade_UnifySameDomain_ctor, StlAPI_Writer_ctor, TColgp_Array1OfDir_Value,
    TColgp_Array1OfDir_ctor, TopAbs_Orientation, TopAbs_ShapeEnum, TopExp_Explorer,
    TopExp_Explorer_ctor, TopLoc_Location_Transformation, TopLoc_Location_ctor,
    TopLoc_Location_from_transform, TopoDS_Compound, TopoDS_Compound_to_owned, TopoDS_Edge,
    TopoDS_Edge_to_owned, TopoDS_Face, TopoDS_Face_ctor, TopoDS_Face_to_owned, TopoDS_Shape,
    TopoDS_Shell, TopoDS_Solid, TopoDS_Solid_to_owned, TopoDS_Vertex, TopoDS_Vertex_to_owned,
    TopoDS_Wire, TopoDS_Wire_to_owned, TopoDS_cast_to_compound, TopoDS_cast_to_edge,
    TopoDS_cast_to_face, TopoDS_cast_to_solid, TopoDS_cast_to_vertex, TopoDS_cast_to_wire,
};
use std::path::Path;

#[derive(Debug, Copy, Clone)]
pub enum Angle {
    Radians(f64),
    Degrees(f64),
}

impl Angle {
    pub fn radians(&self) -> f64 {
        match self {
            Self::Radians(r) => *r,
            Self::Degrees(d) => (d * std::f64::consts::PI) / 180.0,
        }
    }

    pub fn degrees(&self) -> f64 {
        match self {
            Self::Radians(r) => (r * 180.0) / std::f64::consts::PI,
            Self::Degrees(d) => *d,
        }
    }
}

pub trait ToAngle {
    fn degrees(&self) -> Angle;
    fn radians(&self) -> Angle;
}

impl ToAngle for f64 {
    fn degrees(&self) -> Angle {
        Angle::Degrees(*self)
    }

    fn radians(&self) -> Angle {
        Angle::Radians(*self)
    }
}

impl ToAngle for u64 {
    fn degrees(&self) -> Angle {
        Angle::Degrees(*self as f64)
    }

    fn radians(&self) -> Angle {
        Angle::Radians(*self as f64)
    }
}

pub fn make_point(p: DVec3) -> UniquePtr<gp_Pnt> {
    new_point(p.x, p.y, p.z)
}

pub fn make_dir(p: DVec3) -> UniquePtr<gp_Dir> {
    gp_Dir_ctor(p.x, p.y, p.z)
}

pub fn make_vec(vec: DVec3) -> UniquePtr<gp_Vec> {
    new_vec(vec.x, vec.y, vec.z)
}

pub fn make_axis_2(origin: DVec3, dir: DVec3) -> UniquePtr<gp_Ax2> {
    gp_Ax2_ctor(&make_point(origin), &make_dir(dir))
}

pub struct Vertex {
    _inner: UniquePtr<TopoDS_Vertex>,
}

impl Vertex {
    pub fn new(point: DVec3) -> Self {
        let mut make_vertex = BRepBuilderAPI_MakeVertex_gp_Pnt(&make_point(point));
        let vertex = make_vertex.pin_mut().Vertex();
        let inner = TopoDS_Vertex_to_owned(vertex);

        Self { _inner: inner }
    }
}

pub struct Edge {
    inner: UniquePtr<TopoDS_Edge>,
}

impl Edge {
    pub fn segment(p1: DVec3, p2: DVec3) -> Self {
        let mut make_edge = BRepBuilderAPI_MakeEdge_gp_Pnt_gp_Pnt(&make_point(p1), &make_point(p2));
        let edge = make_edge.pin_mut().Edge();
        let inner = TopoDS_Edge_to_owned(edge);

        Self { inner }
    }

    pub fn circle(center: DVec3, normal: DVec3, radius: f64) -> Self {
        let axis = make_axis_2(center, normal);

        let make_circle = gp_Circ_ctor(&axis, radius);

        let mut make_edge = BRepBuilderAPI_MakeEdge_circle(&make_circle);

        let edge = make_edge.pin_mut().Edge();
        let inner = TopoDS_Edge_to_owned(edge);

        Self { inner }
    }

    pub fn ellipse() {}

    pub fn spline() {}

    pub fn arc(p1: DVec3, p2: DVec3, p3: DVec3) -> Self {
        let make_arc =
            GC_MakeArcOfCircle_point_point_point(&make_point(p1), &make_point(p2), &make_point(p3));

        let mut make_edge = BRepBuilderAPI_MakeEdge_HandleGeomCurve(
            &new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(&GC_MakeArcOfCircle_Value(&make_arc)),
        );

        let edge = make_edge.pin_mut().Edge();
        let inner = TopoDS_Edge_to_owned(edge);

        Self { inner }
    }

    pub fn start_point(&self) -> DVec3 {
        let curve = BRepAdaptor_Curve_ctor(&self.inner);
        let start_param = curve.FirstParameter();
        let point = BRepAdaptor_Curve_value(&curve, start_param);

        dvec3(point.X(), point.Y(), point.Z())
    }

    pub fn end_point(&self) -> DVec3 {
        let curve = BRepAdaptor_Curve_ctor(&self.inner);
        let last_param = curve.LastParameter();
        let point = BRepAdaptor_Curve_value(&curve, last_param);

        dvec3(point.X(), point.Y(), point.Z())
    }

    pub fn approximation_segments(&self) -> ApproximationSegmentIterator {
        let adaptor_curve = BRepAdaptor_Curve_ctor(&self.inner);
        let approximator = GCPnts_TangentialDeflection_ctor(&adaptor_curve, 0.1, 0.1);

        ApproximationSegmentIterator { count: 1, approximator }
    }

    pub fn tangent_arc(_p1: DVec3, _tangent: DVec3, _p3: DVec3) {}
}

pub struct ApproximationSegmentIterator {
    count: usize,
    approximator: UniquePtr<GCPnts_TangentialDeflection>,
}

impl Iterator for ApproximationSegmentIterator {
    type Item = DVec3;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count <= self.approximator.NbPoints() as usize {
            let point = GCPnts_TangentialDeflection_Value(&self.approximator, self.count as i32);

            self.count += 1;
            Some(dvec3(point.X(), point.Y(), point.Z()))
        } else {
            None
        }
    }
}

pub struct Wire {
    inner: UniquePtr<TopoDS_Wire>,
}

impl Wire {
    pub fn from_edges<'a>(edges: impl IntoIterator<Item = &'a Edge>) -> Self {
        let mut make_wire = BRepBuilderAPI_MakeWire_ctor();

        for edge in edges.into_iter() {
            make_wire.pin_mut().add_edge(&edge.inner);
        }

        let wire = make_wire.pin_mut().Wire();
        let inner = TopoDS_Wire_to_owned(wire);

        Self { inner }
    }

    pub fn from_wires<'a>(wires: impl IntoIterator<Item = &'a Wire>) -> Self {
        let mut make_wire = BRepBuilderAPI_MakeWire_ctor();

        for wire in wires.into_iter() {
            make_wire.pin_mut().add_wire(&wire.inner);
        }

        let wire = make_wire.pin_mut().Wire();
        let inner = TopoDS_Wire_to_owned(wire);

        Self { inner }
    }

    pub fn mirror_along_axis(&self, axis_origin: DVec3, axis_dir: DVec3) -> Self {
        let axis_dir = make_dir(axis_dir);
        let axis = gp_Ax1_ctor(&make_point(axis_origin), &axis_dir);

        let mut transform = new_transform();

        transform.pin_mut().set_mirror_axis(&axis);

        let wire_shape = cast_wire_to_shape(&self.inner);

        let mut brep_transform = BRepBuilderAPI_Transform_ctor(wire_shape, &transform, false);

        let mirrored_shape = brep_transform.pin_mut().Shape();
        let mirrored_wire = TopoDS_cast_to_wire(mirrored_shape);
        let inner = TopoDS_Wire_to_owned(mirrored_wire);

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

        Self::from_edges([&top, &right, &bottom, &left].into_iter())
    }

    pub fn fillet(&mut self, radius: f64) {
        // Create a face from this wire
        let mut face = Face::from_wire(self);
        face.fillet(radius);
        let wire = outer_wire(&face.inner);

        self.inner = wire;
    }

    /// Chamfer the wire edges at each vertex by a given distance
    ///
    /// If distance2 is None, then the chamfer is symmetric
    pub fn chamfer(&mut self, distance_1: f64, distance_2: Option<f64>) {
        // Create a face from this wire
        let mut face = Face::from_wire(self);
        face.chamfer(distance_1, distance_2);

        let wire = outer_wire(&face.inner);

        self.inner = wire;
    }

    pub fn translate(&mut self, offset: DVec3) {
        self.transform(offset, dvec3(1.0, 0.0, 0.0), 0.0);
    }

    pub fn transform(&mut self, translation: DVec3, rotation_axis: DVec3, angle: f64) {
        let angle = angle * std::f64::consts::PI / 180.0;

        let mut transform = new_transform();
        let rotation_axis_vec = gp_Ax1_ctor(&make_point(DVec3::ZERO), &make_dir(rotation_axis));
        let translation_vec = make_vec(translation);

        transform.pin_mut().SetRotation(&rotation_axis_vec, angle);
        transform.pin_mut().set_translation_vec(&translation_vec);
        let location = TopLoc_Location_from_transform(&transform);

        let wire_shape = cast_wire_to_shape(&self.inner);
        let mut wire_shape = TopoDS_Shape_to_owned(wire_shape);

        let raise_exception = false;
        wire_shape.pin_mut().translate(&location, raise_exception);

        let translated_wire = TopoDS_cast_to_wire(&wire_shape);
        self.inner = TopoDS_Wire_to_owned(translated_wire);
    }

    pub fn to_face(self) -> Face {
        let only_plane = false;
        let make_face = BRepBuilderAPI_MakeFace_wire(&self.inner, only_plane);

        let face = make_face.Face();
        let inner = TopoDS_Face_to_owned(face);

        Face { inner }
    }

    pub fn to_shape(self) -> Shape {
        let inner_shape = cast_wire_to_shape(&self.inner);
        let inner = TopoDS_Shape_to_owned(inner_shape);

        Shape { inner }
    }

    // Create a closure-based API
    pub fn freeform() {}
}

pub struct Face {
    inner: UniquePtr<TopoDS_Face>,
}

impl Face {
    pub fn from_wire(wire: &Wire) -> Self {
        let only_plane = false;
        let make_face = BRepBuilderAPI_MakeFace_wire(&wire.inner, only_plane);

        let face = make_face.Face();
        let inner = TopoDS_Face_to_owned(face);

        Self { inner }
    }

    pub fn extrude(&self, dir: DVec3) -> Solid {
        let prism_vec = make_vec(dir);

        let copy = false;
        let canonize = true;

        let inner_shape = cast_face_to_shape(&self.inner);
        let mut make_solid = BRepPrimAPI_MakePrism_ctor(inner_shape, &prism_vec, copy, canonize);
        let extruded_shape = make_solid.pin_mut().Shape();
        let solid = TopoDS_cast_to_solid(extruded_shape);
        let inner = TopoDS_Solid_to_owned(solid);

        Solid { inner }
    }

    pub fn extrude_to_face(&self, shape_with_face: &Shape, face: &Face) -> Shape {
        let profile_base = &self.inner;
        let sketch_base = TopoDS_Face_ctor();
        let angle = 0.0;
        let fuse = 1; // 0 = subtractive, 1 = additive
        let modify = false;

        let mut make_prism = BRepFeat_MakeDPrism_ctor(
            &shape_with_face.inner,
            profile_base,
            &sketch_base,
            angle,
            fuse,
            modify,
        );

        let until_face = cast_face_to_shape(&face.inner);
        make_prism.pin_mut().perform_until_face(until_face);

        let extruded_shape = make_prism.pin_mut().Shape();
        let inner = TopoDS_Shape_to_owned(extruded_shape);

        Shape { inner }
    }

    pub fn subtractive_extrude(&self, shape_with_face: &Shape, height: f64) -> Shape {
        let profile_base = &self.inner;
        let sketch_base = TopoDS_Face_ctor();
        let angle = 0.0;
        let fuse = 1; // 0 = subtractive, 1 = additive
        let modify = false;

        let mut make_prism = BRepFeat_MakeDPrism_ctor(
            &shape_with_face.inner,
            profile_base,
            &sketch_base,
            angle,
            fuse,
            modify,
        );

        make_prism.pin_mut().perform_with_height(height);

        let extruded_shape = make_prism.pin_mut().Shape();
        let inner = TopoDS_Shape_to_owned(extruded_shape);

        Shape { inner }
    }

    /// Fillets the face edges by a given radius at each vertex
    pub fn fillet(&mut self, radius: f64) {
        // use BRepFilletAPI_MakeFillet2d
        let mut make_fillet = BRepFilletAPI_MakeFillet2d_ctor(&self.inner);

        // add all vertices from the face
        let face_shape = cast_face_to_shape(&self.inner);

        // We use a shape map here to avoid duplicates.
        let mut shape_map = new_indexed_map_of_shape();
        map_shapes(face_shape, TopAbs_ShapeEnum::TopAbs_VERTEX, shape_map.pin_mut());

        for i in 1..=shape_map.Extent() {
            let vertex = TopoDS_cast_to_vertex(shape_map.FindKey(i));
            BRepFilletAPI_MakeFillet2d_add_fillet(make_fillet.pin_mut(), vertex, radius);
        }

        make_fillet.pin_mut().Build(&Message_ProgressRange_ctor());

        let result_shape = make_fillet.pin_mut().Shape();
        // convert back to a wire with BRepTools::OuterWire
        let result_face = TopoDS_cast_to_face(result_shape);

        self.inner = TopoDS_Face_to_owned(result_face);
    }

    /// Chamfer the wire edges at each vertex by a given distance
    ///
    /// If distance2 is None, then the chamfer is symmetric
    pub fn chamfer(&mut self, distance_1: f64, distance_2: Option<f64>) {
        let distance_2 = distance_2.unwrap_or(distance_1);

        // add all vertices from the face
        let face_shape = cast_face_to_shape(&self.inner);

        // use BRepFilletAPI_MakeFillet2d for 2d face
        let mut make_fillet = BRepFilletAPI_MakeFillet2d_ctor(&self.inner);

        let mut vertex_map = new_indexed_map_of_shape();
        map_shapes(face_shape, TopAbs_ShapeEnum::TopAbs_VERTEX, vertex_map.pin_mut());

        // get map of vertices to edges so we can find the edges connected to each vertex
        let mut data_map = new_indexed_data_map_of_shape_list_of_shape();
        map_shapes_and_ancestors(
            face_shape,
            TopAbs_ShapeEnum::TopAbs_VERTEX,
            TopAbs_ShapeEnum::TopAbs_EDGE,
            data_map.pin_mut(),
        );

        // chamfer at vertex of all edges
        for i in 1..=vertex_map.Extent() {
            let edges = shape_list_to_vector(data_map.FindFromIndex(i));
            let edge_1 = edges.get(0).expect("Vertex has no edges");
            let edge_2 = edges.get(1).expect("Vertex has only one edge");
            BRepFilletAPI_MakeFillet2d_add_chamfer(
                make_fillet.pin_mut(),
                TopoDS_cast_to_edge(edge_1),
                TopoDS_cast_to_edge(edge_2),
                distance_1,
                distance_2,
            );
        }

        let filleted_shape = make_fillet.pin_mut().Shape();
        let result_face = TopoDS_cast_to_face(filleted_shape);

        self.inner = TopoDS_Face_to_owned(result_face);
    }

    pub fn edges(&self) -> EdgeIterator {
        let explorer =
            TopExp_Explorer_ctor(cast_face_to_shape(&self.inner), TopAbs_ShapeEnum::TopAbs_EDGE);

        EdgeIterator { explorer }
    }

    pub fn center_of_mass(&self) -> DVec3 {
        let mut props = GProp_GProps_ctor();

        let inner_shape = cast_face_to_shape(&self.inner);
        BRepGProp_SurfaceProperties(inner_shape, props.pin_mut());

        let center = GProp_GProps_CentreOfMass(&props);

        dvec3(center.X(), center.Y(), center.Z())
    }

    pub fn normal_at(&self, pos: DVec3) -> DVec3 {
        let surface = BRep_Tool_Surface(&self.inner);
        let projector = GeomAPI_ProjectPointOnSurf_ctor(&make_point(pos), &surface);
        let mut u: f64 = 0.0;
        let mut v: f64 = 0.0;

        projector.LowerDistanceParameters(&mut u, &mut v);

        let mut p = new_point(0.0, 0.0, 0.0);
        let mut normal = new_vec(0.0, 1.0, 0.0);

        let face = BRepGProp_Face_ctor(&self.inner);
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
        let inner_shape = cast_face_to_shape(&self.inner);
        let other_inner_shape = cast_face_to_shape(&other.inner);

        let mut fuse_operation = BRepAlgoAPI_Fuse_ctor(inner_shape, other_inner_shape);

        let fuse_shape = fuse_operation.pin_mut().Shape();

        let compound = TopoDS_cast_to_compound(fuse_shape);
        let inner = TopoDS_Compound_to_owned(compound);

        CompoundFace { inner }
    }

    pub fn orientation(&self) -> FaceOrientation {
        FaceOrientation::from(self.inner.Orientation())
    }

    pub fn from_shape(shape: &Shape) -> Self {
        let face = TopoDS_cast_to_face(&shape.inner);
        let inner = TopoDS_Face_to_owned(face);

        Self { inner }
    }
}

pub struct CompoundFace {
    inner: UniquePtr<TopoDS_Compound>,
}

impl CompoundFace {
    pub fn clean(&mut self) -> Self {
        let inner = cast_compound_to_shape(&self.inner);
        let inner = TopoDS_Shape_to_owned(inner);
        let mut shape = Shape { inner };

        shape.clean();

        let inner = TopoDS_cast_to_compound(&shape.inner);
        let inner = TopoDS_Compound_to_owned(inner);

        Self { inner }
    }

    pub fn extrude(&self, dir: DVec3) -> Shape {
        let prism_vec = make_vec(dir);

        let copy = false;
        let canonize = true;

        let inner_shape = cast_compound_to_shape(&self.inner);

        let mut make_solid = BRepPrimAPI_MakePrism_ctor(inner_shape, &prism_vec, copy, canonize);
        let extruded_shape = make_solid.pin_mut().Shape();
        let inner = TopoDS_Shape_to_owned(extruded_shape);

        Shape { inner }
    }

    pub fn set_global_translation(&mut self, translation: DVec3) {
        let inner = cast_compound_to_shape(&self.inner);
        let inner = TopoDS_Shape_to_owned(inner);
        let mut shape = Shape { inner };

        shape.set_global_translation(translation);

        let compound = TopoDS_cast_to_compound(&shape.inner);
        let compound = TopoDS_Compound_to_owned(compound);

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

impl From<TopAbs_Orientation> for FaceOrientation {
    fn from(orientation: TopAbs_Orientation) -> Self {
        match orientation {
            TopAbs_Orientation::TopAbs_FORWARD => Self::Forward,
            TopAbs_Orientation::TopAbs_REVERSED => Self::Reversed,
            TopAbs_Orientation::TopAbs_INTERNAL => Self::Internal,
            TopAbs_Orientation::TopAbs_EXTERNAL => Self::External,
            TopAbs_Orientation { repr } => {
                panic!("TopAbs_Orientation had an unrepresentable value: {repr}")
            },
        }
    }
}

pub struct Shell {
    _inner: UniquePtr<TopoDS_Shell>,
}

pub struct Solid {
    inner: UniquePtr<TopoDS_Solid>,
}

impl Solid {
    pub fn to_shape(self) -> Shape {
        let inner_shape = cast_solid_to_shape(&self.inner);
        let inner = TopoDS_Shape_to_owned(inner_shape);

        Shape { inner }
    }

    // TODO(bschwind) - Do some cool stuff from this link:
    // https://neweopencascade.wordpress.com/2018/10/17/lets-talk-about-fillets/
    // Key takeaway: Use the `SectionEdges` function to retrieve edges that were
    // the result of combining two shapes.
    pub fn fillet_edge(&self, radius: f64, edge: &Edge) -> Compound {
        let inner_shape = cast_solid_to_shape(&self.inner);

        let mut make_fillet = BRepFilletAPI_MakeFillet_ctor(inner_shape);
        make_fillet.pin_mut().add_edge(radius, &edge.inner);

        let filleted_shape = make_fillet.pin_mut().Shape();

        let compund = TopoDS_cast_to_compound(filleted_shape);
        let inner = TopoDS_Compound_to_owned(compund);

        Compound { inner }
    }

    // TODO(bschwind) - Accept IntoIter instead of Iterator
    pub fn loft<'a>(wires: impl Iterator<Item = &'a Wire>) -> Self {
        let is_solid = true;
        let mut make_loft = BRepOffsetAPI_ThruSections_ctor(is_solid);

        for wire in wires {
            make_loft.pin_mut().AddWire(&wire.inner);
        }

        // Set to CheckCompatibility to `true` to avoid twisted results.
        make_loft.pin_mut().CheckCompatibility(true);

        let shape = make_loft.pin_mut().Shape();
        let solid = TopoDS_cast_to_solid(shape);
        let inner = TopoDS_Solid_to_owned(solid);

        Self { inner }
    }

    pub fn write_stl<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let inner_shape = cast_solid_to_shape(&self.inner);

        let mut stl_writer = StlAPI_Writer_ctor();
        let triangulation = BRepMesh_IncrementalMesh_ctor(inner_shape, 0.001);
        let success = write_stl(
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

    pub fn subtract(&mut self, other: &Solid) -> (Shape, Vec<Edge>) {
        let inner_shape = cast_solid_to_shape(&self.inner);
        let other_inner_shape = cast_solid_to_shape(&other.inner);

        let mut cut_operation = BRepAlgoAPI_Cut_ctor(inner_shape, other_inner_shape);

        let edge_list = cut_operation.pin_mut().SectionEdges();
        let vec = shape_list_to_vector(edge_list);

        let mut edges = vec![];
        for shape in vec.iter() {
            let edge = TopoDS_cast_to_edge(shape);
            let inner = TopoDS_Edge_to_owned(edge);
            let edge = Edge { inner };
            edges.push(edge);
        }

        let cut_shape = cut_operation.pin_mut().Shape();
        let inner = TopoDS_Shape_to_owned(cut_shape);

        (Shape { inner }, edges)
    }

    pub fn union(&self, other: &Solid) -> (Shape, Vec<Edge>) {
        let inner_shape = cast_solid_to_shape(&self.inner);
        let other_inner_shape = cast_solid_to_shape(&other.inner);

        let mut fuse_operation = BRepAlgoAPI_Fuse_ctor(inner_shape, other_inner_shape);
        let edge_list = fuse_operation.pin_mut().SectionEdges();
        let vec = shape_list_to_vector(edge_list);

        let mut edges = vec![];
        for shape in vec.iter() {
            let edge = TopoDS_cast_to_edge(shape);
            let inner = TopoDS_Edge_to_owned(edge);
            let edge = Edge { inner };
            edges.push(edge);
        }

        let fuse_shape = fuse_operation.pin_mut().Shape();
        let inner = TopoDS_Shape_to_owned(fuse_shape);

        (Shape { inner }, edges)
    }
}

pub struct Compound {
    inner: UniquePtr<TopoDS_Compound>,
}

impl Compound {
    pub fn clean(&mut self) -> Shape {
        let inner = cast_compound_to_shape(&self.inner);
        let inner = TopoDS_Shape_to_owned(inner);
        let mut shape = Shape { inner };

        shape.clean();

        shape
    }

    pub fn to_shape(self) -> Shape {
        let inner_shape = cast_compound_to_shape(&self.inner);
        let inner = TopoDS_Shape_to_owned(inner_shape);

        Shape { inner }
    }
}

pub struct Shape {
    inner: UniquePtr<TopoDS_Shape>,
}

impl Shape {
    pub fn fillet_edge(&mut self, radius: f64, edge: &Edge) {
        let mut make_fillet = BRepFilletAPI_MakeFillet_ctor(&self.inner);
        make_fillet.pin_mut().add_edge(radius, &edge.inner);

        let filleted_shape = make_fillet.pin_mut().Shape();

        self.inner = TopoDS_Shape_to_owned(filleted_shape);
    }

    pub fn chamfer_edge(&mut self, distance: f64, edge: &Edge) {
        let mut make_chamfer = BRepFilletAPI_MakeChamfer_ctor(&self.inner);
        make_chamfer.pin_mut().add_edge(distance, &edge.inner);

        let chamfered_shape = make_chamfer.pin_mut().Shape();

        self.inner = TopoDS_Shape_to_owned(chamfered_shape);
    }

    pub fn fillet_edges<'a>(&mut self, radius: f64, edges: impl IntoIterator<Item = &'a Edge>) {
        let mut make_fillet = BRepFilletAPI_MakeFillet_ctor(&self.inner);

        for edge in edges.into_iter() {
            make_fillet.pin_mut().add_edge(radius, &edge.inner);
        }

        let filleted_shape = make_fillet.pin_mut().Shape();

        self.inner = TopoDS_Shape_to_owned(filleted_shape);
    }

    pub fn chamfer_edges<'a>(&mut self, distance: f64, edges: impl IntoIterator<Item = &'a Edge>) {
        let mut make_chamfer = BRepFilletAPI_MakeChamfer_ctor(&self.inner);

        for edge in edges.into_iter() {
            make_chamfer.pin_mut().add_edge(distance, &edge.inner);
        }

        let chamfered_shape = make_chamfer.pin_mut().Shape();

        self.inner = TopoDS_Shape_to_owned(chamfered_shape);
    }

    /// Performs fillet of `radius` on all edges of the shape
    pub fn fillet(&mut self, radius: f64) {
        let all_edges = self.edges().collect::<Vec<_>>();
        self.fillet_edges(radius, &all_edges);
    }

    /// Performs chamfer of `distance` on all edges of the shape
    pub fn chamfer(&mut self, distance: f64) {
        let all_edges = self.edges().collect::<Vec<_>>();
        self.chamfer_edges(distance, &all_edges);
    }

    pub fn subtract(&mut self, other: &Solid) -> (Shape, Vec<Edge>) {
        let other_inner_shape = cast_solid_to_shape(&other.inner);

        let mut cut_operation = BRepAlgoAPI_Cut_ctor(&self.inner, other_inner_shape);

        let edge_list = cut_operation.pin_mut().SectionEdges();
        let vec = shape_list_to_vector(edge_list);

        let mut edges = vec![];
        for shape in vec.iter() {
            let edge = TopoDS_cast_to_edge(shape);
            let inner = TopoDS_Edge_to_owned(edge);
            let edge = Edge { inner };
            edges.push(edge);
        }

        let cut_shape = cut_operation.pin_mut().Shape();
        let inner = TopoDS_Shape_to_owned(cut_shape);

        (Shape { inner }, edges)
    }

    // TODO(bschwind) - Deduplicate with the above function.
    pub fn subtract_shape(&mut self, other: &Shape) -> (Shape, Vec<Edge>) {
        let mut cut_operation = BRepAlgoAPI_Cut_ctor(&self.inner, &other.inner);

        let edge_list = cut_operation.pin_mut().SectionEdges();
        let vec = shape_list_to_vector(edge_list);

        let mut edges = vec![];
        for shape in vec.iter() {
            let edge = TopoDS_cast_to_edge(shape);
            let inner = TopoDS_Edge_to_owned(edge);
            let edge = Edge { inner };
            edges.push(edge);
        }

        let cut_shape = cut_operation.pin_mut().Shape();
        let inner = TopoDS_Shape_to_owned(cut_shape);

        (Shape { inner }, edges)
    }

    pub fn read_step<P: AsRef<Path>>(path: P) -> Self {
        let mut reader = STEPControl_Reader_ctor();
        let _return_status =
            read_step(reader.pin_mut(), path.as_ref().to_string_lossy().to_string());
        reader.pin_mut().TransferRoots(&Message_ProgressRange_ctor());

        let inner = one_shape(&reader);

        Self { inner }
    }

    pub fn union(&self, other: &Solid) -> (Shape, Vec<Edge>) {
        let other_inner_shape = cast_solid_to_shape(&other.inner);

        let mut fuse_operation = BRepAlgoAPI_Fuse_ctor(&self.inner, other_inner_shape);
        let edge_list = fuse_operation.pin_mut().SectionEdges();
        let vec = shape_list_to_vector(edge_list);

        let mut edges = vec![];
        for shape in vec.iter() {
            let edge = TopoDS_cast_to_edge(shape);
            let inner = TopoDS_Edge_to_owned(edge);
            let edge = Edge { inner };
            edges.push(edge);
        }

        let fuse_shape = fuse_operation.pin_mut().Shape();
        let inner = TopoDS_Shape_to_owned(fuse_shape);

        (Shape { inner }, edges)
    }

    // TODO(bschwind) - Unify this later
    pub fn union_shape(&self, other: &Shape) -> (Shape, Vec<Edge>) {
        let mut fuse_operation = BRepAlgoAPI_Fuse_ctor(&self.inner, &other.inner);
        let edge_list = fuse_operation.pin_mut().SectionEdges();
        let vec = shape_list_to_vector(edge_list);

        let mut edges = vec![];
        for shape in vec.iter() {
            let edge = TopoDS_cast_to_edge(shape);
            let inner = TopoDS_Edge_to_owned(edge);
            let edge = Edge { inner };
            edges.push(edge);
        }

        let fuse_shape = fuse_operation.pin_mut().Shape();
        let inner = TopoDS_Shape_to_owned(fuse_shape);

        (Shape { inner }, edges)
    }

    pub fn write_stl<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let mut stl_writer = StlAPI_Writer_ctor();
        let triangulation = BRepMesh_IncrementalMesh_ctor(&self.inner, 0.001);
        let success = write_stl(
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
        let mut upgrader = ShapeUpgrade_UnifySameDomain_ctor(&self.inner, true, true, true);
        upgrader.pin_mut().AllowInternalEdges(false);
        upgrader.pin_mut().Build();

        let upgraded_shape = upgrader.Shape();

        self.inner = TopoDS_Shape_to_owned(upgraded_shape);
    }

    pub fn set_global_translation(&mut self, translation: DVec3) {
        let mut transform = new_transform();
        let translation_vec = make_vec(translation);
        transform.pin_mut().set_translation_vec(&translation_vec);

        let location = TopLoc_Location_from_transform(&transform);

        self.inner.pin_mut().set_global_translation(&location, false);
    }

    pub fn mesh(&self) -> Mesh {
        let mesher = Mesher::new(self);
        mesher.mesh()
    }

    pub fn edges(&self) -> EdgeIterator {
        let explorer = TopExp_Explorer_ctor(&self.inner, TopAbs_ShapeEnum::TopAbs_EDGE);

        EdgeIterator { explorer }
    }

    pub fn faces(&self) -> FaceIterator {
        let explorer = TopExp_Explorer_ctor(&self.inner, TopAbs_ShapeEnum::TopAbs_FACE);

        FaceIterator { explorer }
    }

    pub fn faces_along_ray(&self, ray_start: DVec3, ray_dir: DVec3) -> Vec<(Face, DVec3)> {
        let mut intersector = BRepIntCurveSurface_Inter_ctor();
        let tolerance = 0.0001;
        intersector.pin_mut().Init(
            &self.inner,
            &gp_Lin_ctor(&make_point(ray_start), &make_dir(ray_dir)),
            tolerance,
        );

        let mut results = vec![];

        while intersector.More() {
            let face = BRepIntCurveSurface_Inter_face(&intersector);
            let point = BRepIntCurveSurface_Inter_point(&intersector);

            let face = Face { inner: TopoDS_Face_to_owned(&face) };

            results.push((face, dvec3(point.X(), point.Y(), point.Z())));

            intersector.pin_mut().Next();
        }

        results
    }
}

struct Mesher {
    inner: UniquePtr<BRepMesh_IncrementalMesh>,
}

impl Mesher {
    fn new(shape: &Shape) -> Self {
        let inner = BRepMesh_IncrementalMesh_ctor(&shape.inner, 0.01);

        if !inner.IsDone() {
            // TODO(bschwind) - Add proper Error type and return Result.
            panic!("Call to BRepMesh_IncrementalMesh_ctor failed");
        }

        Self { inner }
    }

    fn mesh(mut self) -> Mesh {
        let mut vertices = vec![];
        let mut uvs = vec![];
        let mut normals = vec![];
        let mut indices = vec![];

        let triangulated_shape = TopoDS_Shape_to_owned(self.inner.pin_mut().Shape());
        let triangulated_shape = Shape { inner: triangulated_shape };

        for face in triangulated_shape.faces() {
            let mut location = TopLoc_Location_ctor();

            let triangulation_handle = BRep_Tool_Triangulation(&face.inner, location.pin_mut());

            let Ok(triangulation) = Handle_Poly_Triangulation_Get(&triangulation_handle) else {
                // TODO(bschwind) - Do better error handling, use Results.
                println!("Encountered a face with no triangulation");
                continue;
            };

            let index_offset = vertices.len();
            let face_point_count = triangulation.NbNodes();

            for i in 1..=face_point_count {
                let mut point = Poly_Triangulation_Node(triangulation, i);
                point.pin_mut().Transform(&TopLoc_Location_Transformation(&location));
                vertices.push(dvec3(point.X(), point.Y(), point.Z()));
            }

            let mut u_min = f64::INFINITY;
            let mut v_min = f64::INFINITY;

            let mut u_max = f64::NEG_INFINITY;
            let mut v_max = f64::NEG_INFINITY;

            for i in 1..=(face_point_count) {
                let uv = Poly_Triangulation_UV(triangulation, i);
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
            let mut poly_connect = Poly_Connect_ctor(&triangulation_handle);
            let mut normal_array = TColgp_Array1OfDir_ctor(0, face_point_count);

            triangulated_shape_normal(&face.inner, poly_connect.pin_mut(), normal_array.pin_mut());

            // TODO(bschwind) - Why do we start at 1 here?
            for i in 1..(normal_array.Length() as usize) {
                let normal = TColgp_Array1OfDir_Value(&normal_array, i as i32);
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
    explorer: UniquePtr<TopExp_Explorer>,
}

impl Iterator for EdgeIterator {
    type Item = Edge;

    fn next(&mut self) -> Option<Self::Item> {
        if self.explorer.More() {
            let edge = TopoDS_cast_to_edge(self.explorer.Current());
            let inner = TopoDS_Edge_to_owned(edge);

            self.explorer.pin_mut().Next();

            Some(Edge { inner })
        } else {
            None
        }
    }
}

pub struct FaceIterator {
    explorer: UniquePtr<TopExp_Explorer>,
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
    pub fn farthest(self, direction: Direction) -> Option<Face> {
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
            let face = TopoDS_cast_to_face(self.explorer.Current());
            let inner = TopoDS_Face_to_owned(face);

            self.explorer.pin_mut().Next();

            Some(Face { inner })
        } else {
            None
        }
    }
}
