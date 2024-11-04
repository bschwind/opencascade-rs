use crate::primitives::{make_axis_2, make_point};
use cxx::UniquePtr;
use glam::{dvec3, DVec3};
use opencascade_sys::ffi;

use super::make_vec;

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

impl From<ffi::GeomAbsCurveType> for EdgeType {
    fn from(curve_type: ffi::GeomAbsCurveType) -> Self {
        match curve_type {
            ffi::GeomAbsCurveType::GeomAbs_Line => Self::Line,
            ffi::GeomAbsCurveType::GeomAbs_Circle => Self::Circle,
            ffi::GeomAbsCurveType::GeomAbs_Ellipse => Self::Ellipse,
            ffi::GeomAbsCurveType::GeomAbs_Hyperbola => Self::Hyperbola,
            ffi::GeomAbsCurveType::GeomAbs_Parabola => Self::Parabola,
            ffi::GeomAbsCurveType::GeomAbs_BezierCurve => Self::BezierCurve,
            ffi::GeomAbsCurveType::GeomAbs_BSplineCurve => Self::BSplineCurve,
            ffi::GeomAbsCurveType::GeomAbs_OffsetCurve => Self::OffsetCurve,
            ffi::GeomAbsCurveType::GeomAbs_OtherCurve => Self::OtherCurve,
            ffi::GeomAbsCurveType { repr } => panic!("Unexpected curve type: {repr}"),
        }
    }
}

pub struct Edge {
    pub(crate) inner: UniquePtr<ffi::TopoDSEdge>,
}

impl AsRef<Edge> for Edge {
    fn as_ref(&self) -> &Edge {
        self
    }
}

impl Edge {
    pub(crate) fn from_edge(edge: &ffi::TopoDSEdge) -> Self {
        let inner = ffi::TopoDSEdge_to_owned(edge);

        Self { inner }
    }

    fn from_make_edge(mut make_edge: UniquePtr<ffi::BRepBuilderAPI_MakeEdge>) -> Self {
        Self::from_edge(make_edge.pin_mut().edge())
    }

    pub fn segment(p1: DVec3, p2: DVec3) -> Self {
        let make_edge =
            ffi::BRepBuilderAPI_MakeEdge_GpPoint_GpPoint(&make_point(p1), &make_point(p2));

        Self::from_make_edge(make_edge)
    }

    pub fn circle(center: DVec3, normal: DVec3, radius: f64) -> Self {
        let axis = make_axis_2(center, normal);

        let make_circle = ffi::GpCircle_ctor(&axis, radius);
        let make_edge = ffi::BRepBuilderAPI_MakeEdge_circle(&make_circle);

        Self::from_make_edge(make_edge)
    }

    pub fn ellipse() {}

    pub fn spline_from_points(
        points: impl IntoIterator<Item = DVec3>,
        tangents: Option<(DVec3, DVec3)>,
    ) -> Self {
        let points: Vec<_> = points.into_iter().collect();
        let mut array = ffi::TColgpHArray1OfPnt_ctor(1, points.len() as i32);
        for (index, point) in points.into_iter().enumerate() {
            array.pin_mut().set_value(index as i32 + 1, &make_point(point));
        }
        let array_handle = ffi::new_HandleTColgpHArray1OfPnt_from_TColgpHArray1OfPnt(array);

        let periodic = false;
        let tolerance = 1.0e-7;
        let mut interpolate = ffi::GeomAPIInterpolate_ctor(&array_handle, periodic, tolerance);
        if let Some((t_start, t_end)) = tangents {
            interpolate.pin_mut().load(&make_vec(t_start), &make_vec(t_end), true);
        }

        interpolate.pin_mut().perform();
        let bspline_handle = ffi::GeomAPIInterpolate_Curve(&interpolate);
        let curve_handle = ffi::new_HandleGeomCurve_from_HandleGeom_BSplineCurve(&bspline_handle);

        let mut make_edge = ffi::BRepBuilderAPI_MakeEdge_HandleGeomCurve(&curve_handle);
        let edge = make_edge.pin_mut().edge();
        Self::from_edge(edge)
    }

    pub fn arc(p1: DVec3, p2: DVec3, p3: DVec3) -> Self {
        let make_arc = ffi::GCMakeArcOfCircle_point_point_point(
            &make_point(p1),
            &make_point(p2),
            &make_point(p3),
        );

        let make_edge = ffi::BRepBuilderAPI_MakeEdge_HandleGeomCurve(
            &ffi::new_HandleGeomCurve_from_HandleGeomTrimmedCurve(&ffi::GCMakeArcOfCircle_Value(
                &make_arc,
            )),
        );

        Self::from_make_edge(make_edge)
    }

    pub fn start_point(&self) -> DVec3 {
        let curve = ffi::BRepAdaptorCurve_ctor(&self.inner);
        let start_param = curve.first_parameter();
        let point = ffi::BRepAdaptorCurve_value(&curve, start_param);

        dvec3(point.x(), point.y(), point.z())
    }

    pub fn end_point(&self) -> DVec3 {
        let curve = ffi::BRepAdaptorCurve_ctor(&self.inner);
        let last_param = curve.last_parameter();
        let point = ffi::BRepAdaptorCurve_value(&curve, last_param);

        dvec3(point.x(), point.y(), point.z())
    }

    pub fn approximation_segments(&self) -> ApproximationSegmentIterator {
        let adaptor_curve = ffi::BRepAdaptorCurve_ctor(&self.inner);
        let approximator = ffi::GCPntsTangentialDeflection_ctor(&adaptor_curve, 0.1, 0.1);

        ApproximationSegmentIterator { count: 1, approximator }
    }

    pub fn tangent_arc(_p1: DVec3, _tangent: DVec3, _p3: DVec3) {}

    pub fn edge_type(&self) -> EdgeType {
        let curve = ffi::BRepAdaptorCurve_ctor(&self.inner);

        EdgeType::from(curve.get_type())
    }
}

pub struct ApproximationSegmentIterator {
    count: usize,
    approximator: UniquePtr<ffi::GCPntsTangentialDeflection>,
}

impl Iterator for ApproximationSegmentIterator {
    type Item = DVec3;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count <= self.approximator.nb_points() as usize {
            let point =
                ffi::GCPntsTangentialDeflection_Value(&self.approximator, self.count as i32);

            self.count += 1;
            Some(dvec3(point.x(), point.y(), point.z()))
        } else {
            None
        }
    }
}
