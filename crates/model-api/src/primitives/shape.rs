use crate::{
    primitives::{Compound, Edge, Face, FaceIterator, Shell, Solid, Wire},
    wasm,
};

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

    pub fn faces(&self) -> FaceIterator {
        FaceIterator::new(self)
    }
}
