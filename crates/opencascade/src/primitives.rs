use cxx::UniquePtr;
use glam::DVec3;
use opencascade_sys::ffi;

mod boolean_shape;
mod compound;
mod edge;
mod face;
mod shape;
mod shell;
mod solid;
mod vertex;
mod wire;

pub use boolean_shape::*;
pub use compound::*;
pub use edge::*;
pub use face::*;
pub use shape::*;
pub use shell::*;
pub use solid::*;
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

fn make_dir(p: DVec3) -> UniquePtr<ffi::gp_Dir> {
    ffi::gp_Dir_ctor(p.x, p.y, p.z)
}

fn make_vec(vec: DVec3) -> UniquePtr<ffi::gp_Vec> {
    ffi::new_vec(vec.x, vec.y, vec.z)
}

fn make_axis_1(origin: DVec3, dir: DVec3) -> UniquePtr<ffi::gp_Ax1> {
    ffi::gp_Ax1_ctor(&make_point(origin), &make_dir(dir))
}

fn make_axis_2(origin: DVec3, dir: DVec3) -> UniquePtr<ffi::gp_Ax2> {
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
