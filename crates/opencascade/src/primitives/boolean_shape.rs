use crate::primitives::{Edge, Shape};
use std::ops::{Deref, DerefMut};

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
