use crate::wasm::new_line_segment;
use glam::DVec3;

pub struct EdgeId(pub(crate) u64);

pub struct Edge {
    pub(crate) inner: EdgeId,
}

impl AsRef<Edge> for Edge {
    fn as_ref(&self) -> &Edge {
        self
    }
}

impl Edge {
    pub fn segment(p1: DVec3, p2: DVec3) -> Self {
        let id = new_line_segment(p1.into(), p2.into());

        Self { inner: EdgeId(id) }
    }
}
