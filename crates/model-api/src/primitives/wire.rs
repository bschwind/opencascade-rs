use crate::{
    primitives::{Edge, Face},
    wasm,
    wasm::WasmWire,
};

pub struct Wire {
    pub(crate) inner: WasmWire,
}

impl AsRef<Wire> for Wire {
    fn as_ref(&self) -> &Wire {
        self
    }
}

impl Wire {
    pub fn from_edges<'a>(edges: impl IntoIterator<Item = &'a Edge>) -> Self {
        let wire_builder = wasm::WasmWireBuilder::new();

        for edge in edges.into_iter() {
            wire_builder.add_edge(&edge.inner);
        }

        Self { inner: wire_builder.build() }
    }

    pub fn fillet(&self, radius: f64) -> Self {
        let face = Face::from_wire(self).fillet(radius);
        face.outer_wire()
    }
}
