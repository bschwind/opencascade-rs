use crate::wasm;
use glam::DVec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EdgeType {
    Line,
    Circle,
    Ellipse,
    Hyperbola,
    Parabola,
    BezierCurve,
    BSplineCurve,
    OffsetCurve,
    OtherCurve,
}

impl From<wasm::EdgeType> for EdgeType {
    fn from(edge_type: wasm::EdgeType) -> Self {
        match edge_type {
            wasm::EdgeType::Line => EdgeType::Line,
            wasm::EdgeType::Circle => EdgeType::Circle,
            wasm::EdgeType::Ellipse => EdgeType::Ellipse,
            wasm::EdgeType::Hyperbola => EdgeType::Hyperbola,
            wasm::EdgeType::Parabola => EdgeType::Parabola,
            wasm::EdgeType::BezierCurve => EdgeType::BezierCurve,
            wasm::EdgeType::BSplineCurve => EdgeType::BSplineCurve,
            wasm::EdgeType::OffsetCurve => EdgeType::OffsetCurve,
            wasm::EdgeType::OtherCurve => EdgeType::OtherCurve,
        }
    }
}

pub struct Edge {
    pub(crate) inner: wasm::Edge,
}

impl AsRef<Edge> for Edge {
    fn as_ref(&self) -> &Edge {
        self
    }
}

impl Edge {
    pub fn segment(p1: DVec3, p2: DVec3) -> Self {
        let inner = wasm::Edge::segment(p1.into(), p2.into());

        Edge { inner }
    }

    pub fn circle(center: DVec3, normal: DVec3, radius: f64) -> Self {
        let inner = wasm::Edge::circle(center.into(), normal.into(), radius);

        Edge { inner }
    }

    pub fn ellipse() {}

    pub fn splite() {}

    pub fn arc(p1: DVec3, p2: DVec3, p3: DVec3) -> Self {
        let inner = wasm::Edge::arc(p1.into(), p2.into(), p3.into());

        Edge { inner }
    }

    pub fn start_point(&self) -> DVec3 {
        self.inner.start_point().into()
    }

    pub fn end_point(&self) -> DVec3 {
        self.inner.end_point().into()
    }

    pub fn approximation_segments(&self) -> ApproximationSegmentIterator {
        ApproximationSegmentIterator::new(self)
    }

    pub fn tangent_arc(_p1: DVec3, _tangent: DVec3, _p3: DVec3) {}

    pub fn edge_type(&self) -> EdgeType {
        self.inner.edge_type().into()
    }
}

pub struct ApproximationSegmentIterator {
    iterator: wasm::ApproximationSegmentIterator,
}

impl ApproximationSegmentIterator {
    pub fn new(edge: &Edge) -> Self {
        Self { iterator: wasm::ApproximationSegmentIterator::new(&edge.inner) }
    }
}

impl Iterator for ApproximationSegmentIterator {
    type Item = DVec3;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next().map(|point| point.into())
    }
}
