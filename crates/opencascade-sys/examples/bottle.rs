use opencascade_sys::ffi::{
    gp_OX, new_HandleGeomCurve_from_HandleGeom_TrimmedCurve, new_point, new_transform, new_vec,
    BRepBuilderAPI_MakeEdge_HandleGeomCurve, BRepBuilderAPI_MakeFace_wire,
    BRepBuilderAPI_MakeWire_ctor, BRepBuilderAPI_MakeWire_edge_edge_edge,
    BRepBuilderAPI_Transform_ctor, BRepPrimAPI_MakePrism_ctor, GC_MakeArcOfCircle_Value,
    GC_MakeArcOfCircle_point_point_point, GC_MakeSegment_Value, GC_MakeSegment_point_point,
    TopoDS_cast_to_wire,
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

    let mut edge_1 = BRepBuilderAPI_MakeEdge_HandleGeomCurve(
        &new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(&GC_MakeSegment_Value(&segment_1)),
    );

    let mut edge_2 = BRepBuilderAPI_MakeEdge_HandleGeomCurve(
        &new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(&GC_MakeSegment_Value(&segment_2)),
    );

    let mut edge_3 = BRepBuilderAPI_MakeEdge_HandleGeomCurve(
        &new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(&GC_MakeArcOfCircle_Value(&arc)),
    );

    let mut wire = BRepBuilderAPI_MakeWire_edge_edge_edge(
        edge_1.pin_mut().Edge(),
        edge_2.pin_mut().Edge(),
        edge_3.pin_mut().Edge(),
    );

    let x_axis = gp_OX();

    let mut transform = new_transform();
    transform.pin_mut().set_mirror_axis(&x_axis);

    // We're calling Shape() here instead of Wire(), hope that's okay.
    let mut brep_transform =
        BRepBuilderAPI_Transform_ctor(wire.pin_mut().Shape(), &transform, false);
    let mirrored_shape = brep_transform.pin_mut().Shape();
    let mirrored_wire = TopoDS_cast_to_wire(&mirrored_shape);

    let mut make_wire = BRepBuilderAPI_MakeWire_ctor();
    make_wire.pin_mut().add_wire(wire.pin_mut().Wire());
    make_wire.pin_mut().add_wire(mirrored_wire);

    let wire_profile = make_wire.pin_mut().Wire();

    let mut face_profile = BRepBuilderAPI_MakeFace_wire(&wire_profile, false);
    let prism_vec = new_vec(0.0, 0.0, height);
    // We're calling Shape here instead of Face(), hope that's also okay.
    let body = BRepPrimAPI_MakePrism_ctor(face_profile.pin_mut().Shape(), &prism_vec, false, true);
}
