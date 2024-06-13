use crate::wasm;
use glam::DVec3;

mod compound;
mod edge;
mod face;
mod shape;
mod shell;
mod solid;
mod wire;

pub use compound::*;
pub use edge::*;
pub use face::*;
pub use shape::*;
pub use shell::*;
pub use solid::*;
pub use wire::*;

pub trait IntoShape {
    fn into_shape(self) -> Shape;
}

impl<T: Into<Shape>> IntoShape for T {
    fn into_shape(self) -> Shape {
        self.into()
    }
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

pub struct EdgeIterator {
    iterator: wasm::EdgeIterator,
}

impl EdgeIterator {
    pub fn new(face: &Face) -> Self {
        Self { iterator: wasm::EdgeIterator::new(&face.inner) }
    }
}

impl Iterator for EdgeIterator {
    type Item = Edge;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next().map(|inner_edge| Edge { inner: inner_edge })
    }
}

pub struct FaceIterator {
    iterator: wasm::FaceIterator,
}

impl FaceIterator {
    pub fn new(shape: &Shape) -> Self {
        Self { iterator: wasm::FaceIterator::new(&shape.inner) }
    }

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
        self.iterator.next().map(|inner_face| Face { inner: inner_face })
    }
}
