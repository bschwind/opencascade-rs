use std::iter::once;

use crate::{
    angle::{Angle, ToAngle},
    law_function::law_function_from_graph,
    make_pipe_shell::make_pipe_shell_with_law_function,
    primitives::{make_dir, make_point, make_vec, Edge, Face, JoinType, Shape, Shell},
    Error,
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

    pub fn from_ordered_points(points: impl IntoIterator<Item = DVec3>) -> Result<Self, Error> {
        let points: Vec<_> = points.into_iter().collect();
        if points.len() < 2 {
            return Err(Error::NotEnoughPoints);
        }

        let (first, last) = (points.first().unwrap(), points.last().unwrap());
        let mut make_wire = ffi::BRepBuilderAPI_MakeWire_ctor();

        if points.len() == 2 {
            make_wire.pin_mut().add_edge(&Edge::segment(*first, *last).inner);
        } else {
            for window in points.windows(2).chain(once([*last, *first].as_slice())) {
                let edge = Edge::segment(window[0], window[1]);
                make_wire.pin_mut().add_edge(&edge.inner);
            }
        }

        Ok(Self::from_make_wire(make_wire))
    }

    pub fn from_edges<'a>(edges: impl IntoIterator<Item = &'a Edge>) -> Self {
        let mut make_wire = ffi::BRepBuilderAPI_MakeWire_ctor();

        for edge in edges.into_iter() {
            make_wire.pin_mut().add_edge(&edge.inner);
        }

        Self::from_make_wire(make_wire)
    }

    pub fn from_unordered_edges<T: AsRef<Edge>>(
        unordered_edges: impl IntoIterator<Item = T>,
        edge_connection: EdgeConnection,
    ) -> Self {
        let mut edges = ffi::new_Handle_TopTools_HSequenceOfShape();

        for edge in unordered_edges {
            let edge_shape = ffi::cast_edge_to_shape(&edge.as_ref().inner);
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

    /// Offset the wire by a given distance and join settings
    #[must_use]
    pub fn offset(&self, distance: f64, join_type: JoinType) -> Self {
        let mut make_offset =
            ffi::BRepOffsetAPI_MakeOffset_wire_ctor(&self.inner, join_type.into());
        make_offset.pin_mut().Perform(distance, 0.0);

        let offset_shape = make_offset.pin_mut().Shape();
        let result_wire = ffi::TopoDS_cast_to_wire(offset_shape);

        Self::from_wire(result_wire)
    }

    /// Sweep the wire along a path to produce a shell
    #[must_use]
    pub fn sweep_along(&self, path: &Wire) -> Shell {
        let profile_shape = ffi::cast_wire_to_shape(&self.inner);
        let mut make_pipe = ffi::BRepOffsetAPI_MakePipe_ctor(&path.inner, profile_shape);

        let pipe_shape = make_pipe.pin_mut().Shape();
        let result_shell = ffi::TopoDS_cast_to_shell(pipe_shape);

        Shell::from_shell(result_shell)
    }

    /// Sweep the wire along a path, modulated by a function, to produce a shell
    #[must_use]
    pub fn sweep_along_with_radius_values(
        &self,
        path: &Wire,
        radius_values: impl IntoIterator<Item = (f64, f64)>,
    ) -> Shell {
        let law_function = law_function_from_graph(radius_values);
        let law_handle = ffi::Law_Function_to_handle(law_function);

        let mut make_pipe_shell =
            make_pipe_shell_with_law_function(&self.inner, &path.inner, &law_handle);
        let pipe_shape = make_pipe_shell.pin_mut().Shape();
        let result_shell = ffi::TopoDS_cast_to_shell(pipe_shape);

        Shell::from_shell(result_shell)
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

pub struct WireBuilder {
    inner: UniquePtr<ffi::BRepBuilderAPI_MakeWire>,
}

impl Default for WireBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl WireBuilder {
    pub fn new() -> Self {
        let make_wire = ffi::BRepBuilderAPI_MakeWire_ctor();

        Self { inner: make_wire }
    }

    pub fn add_edge(&mut self, edge: &Edge) {
        self.inner.pin_mut().add_edge(&edge.inner);
    }

    pub fn build(self) -> Wire {
        Wire::from_make_wire(self.inner)
    }
}
