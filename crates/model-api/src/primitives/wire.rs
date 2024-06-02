use crate::{primitives::Edge, wasm};

pub struct WireId(pub(crate) u64);

pub struct Wire {
    pub(crate) inner: WireId,
}

impl AsRef<Wire> for Wire {
    fn as_ref(&self) -> &Wire {
        self
    }
}

impl Wire {
    pub fn from_edges<'a>(edges: impl IntoIterator<Item = &'a Edge>) -> Self {
        let wire_builder = wasm::new_wire_builder();

        for edge in edges.into_iter() {
            wasm::wire_builder_add_edge(wire_builder, edge.inner.0);
        }

        Self { inner: WireId(wasm::wire_builder_build(wire_builder)) }
    }
}
