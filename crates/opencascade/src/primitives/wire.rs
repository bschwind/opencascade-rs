use std::iter::once;

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

/// Provides control over how an edge is considered "connected" to another edge.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EdgeConnection {
    /// The edges must share the same exact vertices to be considered connected.
    Exact,

    /// The endpoints of two edges must be with `tolerance` distance to be considered connected.
    Fuzzy { tolerance: f64 },
}

impl Default for EdgeConnection {
    fn default() -> Self {
        Self::Fuzzy { tolerance: 0.001 }
    }
}

impl Wire {
    pub(crate) fn from_wire(wire: &ffi::TopoDS_Wire) -> Self {
        let inner = ffi::TopoDS_Wire_to_owned(wire);

        Self { inner }
    }

    fn from_make_wire(mut make_wire: UniquePtr<ffi::BRepBuilderAPI_MakeWire>) -> Self {
        Self::from_wire(make_wire.pin_mut().Wire())
    }

    pub fn from_ordered_points(points: &[DVec3]) -> Self {
        let mut make_wire = ffi::BRepBuilderAPI_MakeWire_ctor();

        if let (Some(first), Some(last)) = (points.first(), points.last()) {
            for window in points.windows(2).chain(once([*last, *first].as_slice())) {
                let edge = Edge::segment(window[0], window[1]);
                make_wire.pin_mut().add_edge(&edge.inner);
            }
        }

        Self::from_make_wire(make_wire)
    }

    pub fn from_edges<'a>(edges: impl IntoIterator<Item = &'a Edge>) -> Self {
        let mut make_wire = ffi::BRepBuilderAPI_MakeWire_ctor();

        for edge in edges.into_iter() {
            make_wire.pin_mut().add_edge(&edge.inner);
        }

        Self::from_make_wire(make_wire)
    }

    pub fn from_unordered_edges<'a>(
        unordered_edges: impl IntoIterator<Item = &'a Edge>,
        edge_connection: EdgeConnection,
    ) -> Self {
        let mut edges = ffi::new_Handle_TopTools_HSequenceOfShape();

        for edge in unordered_edges {
            let edge_shape = ffi::cast_edge_to_shape(&edge.inner);
            ffi::TopTools_HSequenceOfShape_append(edges.pin_mut(), edge_shape);
        }

        let mut wires = ffi::new_Handle_TopTools_HSequenceOfShape();

        let (tolerance, shared) = match edge_connection {
            EdgeConnection::Exact => (0.0, true),
            EdgeConnection::Fuzzy { tolerance } => (tolerance, false),
        };

        ffi::connect_edges_to_wires(edges.pin_mut(), tolerance, shared, wires.pin_mut());

        let mut make_wire = ffi::BRepBuilderAPI_MakeWire_ctor();

        let wire_len = ffi::TopTools_HSequenceOfShape_length(&wires);

        for index in 1..=wire_len {
            let wire_shape = ffi::TopTools_HSequenceOfShape_value(&wires, index);
            let wire = ffi::TopoDS_cast_to_wire(wire_shape);

            make_wire.pin_mut().add_wire(wire);
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

    #[must_use]
    pub fn mirror_along_axis(&self, axis_origin: DVec3, axis_dir: DVec3) -> Self {
        let axis_dir = make_dir(axis_dir);
        let axis = ffi::gp_Ax1_ctor(&make_point(axis_origin), &axis_dir);

        let mut transform = ffi::new_transform();

        transform.pin_mut().set_mirror_axis(&axis);

        let wire_shape = ffi::cast_wire_to_shape(&self.inner);

        let mut brep_transform = ffi::BRepBuilderAPI_Transform_ctor(wire_shape, &transform, false);

        let mirrored_shape = brep_transform.pin_mut().Shape();
        let mirrored_wire = ffi::TopoDS_cast_to_wire(mirrored_shape);

        Self::from_wire(mirrored_wire)
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

    #[must_use]
    pub fn fillet(&self, radius: f64) -> Wire {
        // Create a face from this wire
        let face = Face::from_wire(self).fillet(radius);
        let inner = ffi::outer_wire(&face.inner);

        Self { inner }
    }

    /// Chamfer the wire edges at each vertex by a given distance.
    #[must_use]
    pub fn chamfer(&self, distance_1: f64) -> Wire {
        let face = Face::from_wire(self).chamfer(distance_1);
        let inner = ffi::outer_wire(&face.inner);

        Self { inner }
    }

    #[must_use]
    pub fn translate(&self, offset: DVec3) -> Self {
        self.transform(offset, dvec3(1.0, 0.0, 0.0), 0.degrees())
    }

    #[must_use]
    pub fn transform(&self, translation: DVec3, rotation_axis: DVec3, angle: Angle) -> Self {
        let mut transform = ffi::new_transform();
        let rotation_axis_vec =
            ffi::gp_Ax1_ctor(&make_point(DVec3::ZERO), &make_dir(rotation_axis));
        let translation_vec = make_vec(translation);

        transform.pin_mut().SetRotation(&rotation_axis_vec, angle.radians());
        transform.pin_mut().set_translation_vec(&translation_vec);
        let location = ffi::TopLoc_Location_from_transform(&transform);

        let wire_shape = ffi::cast_wire_to_shape(&self.inner);
        let mut wire_shape = Shape::from_shape(wire_shape).inner;

        let raise_exception = false;
        wire_shape.pin_mut().translate(&location, raise_exception);

        let translated_wire = ffi::TopoDS_cast_to_wire(&wire_shape);

        Self::from_wire(translated_wire)
    }

    pub fn to_face(self) -> Face {
        let only_plane = false;
        let make_face = ffi::BRepBuilderAPI_MakeFace_wire(&self.inner, only_plane);

        Face::from_face(make_face.Face())
    }

    // Create a closure-based API
    pub fn freeform() {}
}
