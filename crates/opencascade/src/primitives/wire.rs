use crate::{
    angle::{Angle, ToAngle},
    primitives::{make_dir, make_point, make_vec, Edge, Face, Shape},
};
use cxx::UniquePtr;
use glam::{dvec3, DVec3};
use opencascade_sys::ffi;

pub struct Wire {
    pub(crate) inner: UniquePtr<ffi::TopoDS_Wire>,
}

impl AsRef<Wire> for Wire {
    fn as_ref(&self) -> &Wire {
        self
    }
}

impl Wire {
    fn from_make_wire(mut make_wire: UniquePtr<ffi::BRepBuilderAPI_MakeWire>) -> Self {
        let wire = make_wire.pin_mut().Wire();
        let inner = ffi::TopoDS_Wire_to_owned(wire);

        Self { inner }
    }

    pub fn from_edges<'a>(edges: impl IntoIterator<Item = &'a Edge>) -> Self {
        let mut make_wire = ffi::BRepBuilderAPI_MakeWire_ctor();

        for edge in edges.into_iter() {
            make_wire.pin_mut().add_edge(&edge.inner);
        }

        Self::from_make_wire(make_wire)
    }

    pub fn from_wires<'a>(wires: impl IntoIterator<Item = &'a Wire>) -> Self {
        let mut make_wire = ffi::BRepBuilderAPI_MakeWire_ctor();

        for wire in wires.into_iter() {
            make_wire.pin_mut().add_wire(&wire.inner);
        }

        Self::from_make_wire(make_wire)
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
        let mut face = Face::from_wire(self);
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

    pub fn to_shape(self) -> Shape {
        let inner_shape = ffi::cast_wire_to_shape(&self.inner);
        let inner = ffi::TopoDS_Shape_to_owned(inner_shape);

        Shape { inner }
    }

    // Create a closure-based API
    pub fn freeform() {}
}
