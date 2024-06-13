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

    #[must_use]
    pub fn fillet_new_edges(&self, radius: f64) -> Shape {
        self.shape.fillet_edges(radius, &self.new_edges)
    }

    // #[must_use]
    // pub fn variable_fillet_new_edges(
    //     &self,
    //     radius_values: impl IntoIterator<Item = (f64, f64)>,
    // ) -> Shape {
    //     self.shape.variable_fillet_edges(radius_values, &self.new_edges)
    // }

    #[must_use]
    pub fn chamfer_new_edges(&self, distance: f64) -> Shape {
        self.shape.chamfer_edges(distance, &self.new_edges)
    }
}
