// use crate::primitives::BooleanShape;
use crate::{
    primitives::{
        BooleanShape, Compound, Edge, EdgeIterator, Face, FaceIterator, Shell, Solid, Wire,
    },
    wasm,
};
use glam::{dvec3, DVec3};

pub struct Shape {
    pub(crate) inner: wasm::Shape,
}

impl AsRef<Shape> for Shape {
    fn as_ref(&self) -> &Shape {
        self
    }
}

impl From<Edge> for Shape {
    fn from(edge: Edge) -> Self {
        Shape::from_edge(&edge)
    }
}

impl From<&Edge> for Shape {
    fn from(edge: &Edge) -> Self {
        Shape::from_edge(edge)
    }
}

impl From<Wire> for Shape {
    fn from(wire: Wire) -> Self {
        Shape::from_wire(&wire)
    }
}

impl From<&Wire> for Shape {
    fn from(wire: &Wire) -> Self {
        Shape::from_wire(wire)
    }
}

impl From<Face> for Shape {
    fn from(face: Face) -> Self {
        Shape::from_face(&face)
    }
}

impl From<&Face> for Shape {
    fn from(face: &Face) -> Self {
        Shape::from_face(face)
    }
}

impl From<Shell> for Shape {
    fn from(shell: Shell) -> Self {
        Shape::from_shell(&shell)
    }
}

impl From<&Shell> for Shape {
    fn from(shell: &Shell) -> Self {
        Shape::from_shell(shell)
    }
}

impl From<Solid> for Shape {
    fn from(solid: Solid) -> Self {
        Shape::from_solid(&solid)
    }
}

impl From<&Solid> for Shape {
    fn from(solid: &Solid) -> Self {
        Shape::from_solid(solid)
    }
}

impl From<Compound> for Shape {
    fn from(compound: Compound) -> Self {
        Shape::from_compound(&compound)
    }
}

impl From<&Compound> for Shape {
    fn from(compound: &Compound) -> Self {
        Shape::from_compound(compound)
    }
}

impl Shape {
    pub fn from_edge(edge: &Edge) -> Self {
        let shape = wasm::Shape::from_edge(&edge.inner);

        Self { inner: shape }
    }

    pub fn from_wire(wire: &Wire) -> Self {
        let shape = wasm::Shape::from_wire(&wire.inner);

        Self { inner: shape }
    }

    pub fn from_face(face: &Face) -> Self {
        let shape = wasm::Shape::from_face(&face.inner);

        Self { inner: shape }
    }

    pub fn from_shell(shell: &Shell) -> Self {
        let shape = wasm::Shape::from_shell(&shell.inner);

        Self { inner: shape }
    }

    pub fn from_solid(solid: &Solid) -> Self {
        let shape = wasm::Shape::from_solid(&solid.inner);

        Self { inner: shape }
    }

    pub fn from_compound(compound: &Compound) -> Self {
        let shape = wasm::Shape::from_compound(&compound.inner);

        Self { inner: shape }
    }

    /// Make a box with one corner at corner_1, and the opposite corner
    /// at corner_2.
    pub fn box_from_corners(corner_1: DVec3, corner_2: DVec3) -> Self {
        let shape = wasm::Shape::box_from_corners(corner_1.into(), corner_2.into());

        Self { inner: shape }
    }

    /// Make a box with `width` (x), `depth` (y), and `height` (z)
    /// extending into the positive axes
    pub fn box_with_dimensions(width: f64, depth: f64, height: f64) -> Self {
        let corner_1 = DVec3::ZERO;
        let corner_2 = dvec3(width, depth, height);
        Self::box_from_corners(corner_1, corner_2)
    }

    #[must_use]
    pub fn fillet_edges<T: AsRef<Edge>>(
        &self,
        radius: f64,
        edges: impl IntoIterator<Item = T>,
    ) -> Self {
        let make_fillet = wasm::FilletMaker::new(&self.inner);

        for edge in edges.into_iter() {
            make_fillet.add_edge(radius, &edge.as_ref().inner);
        }

        Self { inner: make_fillet.build() }
    }

    /// Performs chamfer of `distance` on all edges of the shape
    #[must_use]
    pub fn chamfer(&self, distance: f64) -> Self {
        self.chamfer_edges(distance, self.edges())
    }

    #[must_use]
    pub fn chamfer_edges<T: AsRef<Edge>>(
        &self,
        distance: f64,
        edges: impl IntoIterator<Item = T>,
    ) -> Self {
        let make_chamfer = wasm::ChamferMaker::new(&self.inner);

        for edge in edges.into_iter() {
            make_chamfer.add_edge(distance, &edge.as_ref().inner);
        }

        Self { inner: make_chamfer.build() }
    }

    pub fn edges(&self) -> EdgeIterator {
        EdgeIterator::new_from_shape(self)
    }

    pub fn faces(&self) -> FaceIterator {
        FaceIterator::new(self)
    }

    #[must_use]
    pub fn subtract(&self, other: &Shape) -> BooleanShape {
        let (shape, new_edges) = self.inner.subtract(&other.inner);

        let shape = Shape { inner: shape };

        let new_edges = new_edges.into_iter().map(|e| Edge { inner: e }).collect();

        BooleanShape { shape, new_edges }
    }
}
