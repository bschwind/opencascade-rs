use crate::wasm::WasmEdge;
use glam::DVec3;

pub struct Edge {
    pub(crate) inner: WasmEdge,
}

impl AsRef<Edge> for Edge {
    fn as_ref(&self) -> &Edge {
        self
    }
}

impl Edge {
    pub fn segment(p1: DVec3, p2: DVec3) -> Self {
        let inner = WasmEdge::segment(p1.into(), p2.into());

        Edge { inner }
    }
}
