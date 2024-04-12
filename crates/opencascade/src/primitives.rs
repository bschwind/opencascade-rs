use cxx::UniquePtr;
use glam::{DVec2, DVec3};
use opencascade_sys::ffi;

mod boolean_shape;
mod compound;
mod edge;
mod face;
mod shape;
mod shell;
mod solid;
mod surface;
mod vertex;
mod wire;

pub use boolean_shape::*;
pub use compound::*;
pub use edge::*;
pub use face::*;
pub use shape::*;
pub use shell::*;
pub use solid::*;
pub use surface::*;
pub use vertex::*;
pub use wire::*;

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

pub fn make_point2d(p: DVec2) -> UniquePtr<ffi::gp_Pnt2d> {
    ffi::new_point_2d(p.x, p.y)
}

fn make_dir(p: DVec3) -> UniquePtr<ffi::gp_Dir> {
    ffi::gp_Dir_ctor(p.x, p.y, p.z)
}

fn make_vec(vec: DVec3) -> UniquePtr<ffi::gp_Vec> {
    ffi::new_vec(vec.x, vec.y, vec.z)
}

fn make_axis_1(origin: DVec3, dir: DVec3) -> UniquePtr<ffi::gp_Ax1> {
    ffi::gp_Ax1_ctor(&make_point(origin), &make_dir(dir))
}

pub fn make_axis_2(origin: DVec3, dir: DVec3) -> UniquePtr<ffi::gp_Ax2> {
    ffi::gp_Ax2_ctor(&make_point(origin), &make_dir(dir))
}

pub struct EdgeIterator {
    explorer: UniquePtr<ffi::TopExp_Explorer>,
}

impl Iterator for EdgeIterator {
    type Item = Edge;

    fn next(&mut self) -> Option<Self::Item> {
        if self.explorer.More() {
            let edge = ffi::TopoDS_cast_to_edge(self.explorer.Current());
            let edge = Edge::from_edge(edge);

            self.explorer.pin_mut().Next();

            Some(edge)
        } else {
            None
        }
    }
}

impl EdgeIterator {
    pub fn parallel_to(
        self,
        direction: Direction,
    ) -> impl Iterator<Item = <Self as Iterator>::Item> {
        let normalized_dir = direction.normalized_vec();

        self.filter(move |edge| {
            edge.edge_type() == EdgeType::Line
                && 1.0
                    - (edge.end_point() - edge.start_point()).normalize().dot(normalized_dir).abs()
                    < 0.0001
        })
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

        self.max_by(|face_1, face_2| {
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
            let face = Face::from_face(face);

            self.explorer.pin_mut().Next();

            Some(face)
        } else {
            None
        }
    }
}

/// Given n and func, returns an iterator of (t, f(t)) values
/// where t is in the range [0, 1].
/// Note that n + 1 values are returned.
pub fn approximate_function<F: FnMut(f64) -> f64>(
    n: usize,
    mut func: F,
) -> impl Iterator<Item = (f64, f64)> {
    let mut count = 0;

    std::iter::from_fn(move || {
        if count > n {
            return None;
        }

        let t = count as f64 / n as f64;
        count += 1;

        let val = func(t);

        Some((t, val))
    })
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum JoinType {
    Arc,
    // TODO(mkovaxx): Figure out how to make tangent joints work without segfaults
    //Tangent,
    Intersection,
}

impl From<ffi::GeomAbs_JoinType> for JoinType {
    fn from(value: ffi::GeomAbs_JoinType) -> Self {
        match value {
            ffi::GeomAbs_JoinType::GeomAbs_Arc => Self::Arc,
            //ffi::GeomAbs_JoinType::GeomAbs_Tangent => Self::Tangent,
            ffi::GeomAbs_JoinType::GeomAbs_Intersection => Self::Intersection,
            ffi::GeomAbs_JoinType { repr } => panic!("Unexpected join type: {repr}"),
        }
    }
}

impl From<JoinType> for ffi::GeomAbs_JoinType {
    fn from(value: JoinType) -> Self {
        match value {
            JoinType::Arc => ffi::GeomAbs_JoinType::GeomAbs_Arc,
            //JoinType::Tangent => ffi::GeomAbs_JoinType::GeomAbs_Tangent,
            JoinType::Intersection => ffi::GeomAbs_JoinType::GeomAbs_Intersection,
        }
    }
}
