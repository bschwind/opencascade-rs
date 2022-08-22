use opencascade_sys::ffi::{
    new_HandleGeomCurve_from_HandleGeom_TrimmedCurve, new_point,
    BRepBuilderAPI_MakeEdge_HandleGeomCurve, GC_MakeArcOfCircle_Value,
    GC_MakeArcOfCircle_point_point_point, GC_MakeSegment_Value, GC_MakeSegment_point_point,
};

// All dimensions are in millimeters.
pub fn main() {
    let height = 70.0;
    let width = 50.0;
    let thickness = 30.0;

    // Define the points making up the bottle's profile.
    let point_1 = new_point(-width / 2.0, 0.0, 0.0);
    let point_2 = new_point(-width / 2.0, -thickness / 4.0, 0.0);
    let point_3 = new_point(0.0, thickness / 2.0, 0.0);
    let point_4 = new_point(width / 2.0, thickness / 4.0, 0.0);
    let point_5 = new_point(width / 2.0, 0.0, 0.0);

    // Define the arcs and segments of the profile.
    let segment_1 = GC_MakeSegment_point_point(&point_1, &point_2);
    let segment_2 = GC_MakeSegment_point_point(&point_4, &point_5);
    let arc = GC_MakeArcOfCircle_point_point_point(&point_2, &point_3, &point_4);

    let edge_1 = BRepBuilderAPI_MakeEdge_HandleGeomCurve(
        &new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(&GC_MakeSegment_Value(&segment_1)),
    );

    let edge_2 = BRepBuilderAPI_MakeEdge_HandleGeomCurve(
        &new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(&GC_MakeSegment_Value(&segment_2)),
    );

    let edge_3 = BRepBuilderAPI_MakeEdge_HandleGeomCurve(
        &new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(&GC_MakeArcOfCircle_Value(&arc)),
    );

    let wire = ();
}
