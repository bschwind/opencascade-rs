use cxx::UniquePtr;
use opencascade_sys::ffi::{
    gp_Ax2_ctor, gp_DZ, gp_OX, handle_geom_plane_location,
    new_HandleGeomCurve_from_HandleGeom_TrimmedCurve, new_HandleGeomPlane_from_HandleGeomSurface,
    new_list_of_shape, new_point, new_transform, new_vec, shape_list_append_face, type_name,
    write_stl, BRepAlgoAPI_Fuse_ctor, BRepBuilderAPI_MakeEdge_HandleGeomCurve,
    BRepBuilderAPI_MakeFace_wire, BRepBuilderAPI_MakeWire_ctor,
    BRepBuilderAPI_MakeWire_edge_edge_edge, BRepBuilderAPI_Transform_ctor,
    BRepFilletAPI_MakeFillet_ctor, BRepMesh_IncrementalMesh_ctor, BRepPrimAPI_MakeCylinder_ctor,
    BRepPrimAPI_MakePrism_ctor, BRep_Tool_Surface, DynamicType, ExplorerCurrentShape,
    GC_MakeArcOfCircle_Value, GC_MakeArcOfCircle_point_point_point, GC_MakeSegment_Value,
    GC_MakeSegment_point_point, StlAPI_Writer_ctor, TopAbs_ShapeEnum, TopExp_Explorer,
    TopExp_Explorer_ctor, TopoDS_Face, TopoDS_cast_to_edge, TopoDS_cast_to_face,
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
    let point_3 = new_point(0.0, -thickness / 2.0, 0.0);
    let point_4 = new_point(width / 2.0, -thickness / 4.0, 0.0);
    let point_5 = new_point(width / 2.0, 0.0, 0.0);

    // Define the arcs and segments of the profile.
    let arc = GC_MakeArcOfCircle_point_point_point(&point_2, &point_3, &point_4);
    let segment_1 = GC_MakeSegment_point_point(&point_1, &point_2);
    let segment_2 = GC_MakeSegment_point_point(&point_4, &point_5);

    let mut edge_1 = BRepBuilderAPI_MakeEdge_HandleGeomCurve(
        &new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(&GC_MakeSegment_Value(&segment_1)),
    );

    let mut edge_2 = BRepBuilderAPI_MakeEdge_HandleGeomCurve(
        &new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(&GC_MakeArcOfCircle_Value(&arc)),
    );

    let mut edge_3 = BRepBuilderAPI_MakeEdge_HandleGeomCurve(
        &new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(&GC_MakeSegment_Value(&segment_2)),
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
    let mut body =
        BRepPrimAPI_MakePrism_ctor(face_profile.pin_mut().Shape(), &prism_vec, false, true);

    let mut make_fillet = BRepFilletAPI_MakeFillet_ctor(&body.pin_mut().Shape());
    let mut edge_explorer =
        TopExp_Explorer_ctor(&body.pin_mut().Shape(), TopAbs_ShapeEnum::TopAbs_EDGE);

    while edge_explorer.More() {
        let edge = TopoDS_cast_to_edge(edge_explorer.Current());
        make_fillet.pin_mut().add_edge(thickness / 12.0, edge);
        edge_explorer.pin_mut().Next();
    }

    let mut stl_writer = StlAPI_Writer_ctor();

    let body_shape = make_fillet.pin_mut().Shape();

    // Make the bottle neck
    let neck_location = new_point(0.0, 0.0, height);
    let neck_axis = gp_DZ();
    let neck_coord_system = gp_Ax2_ctor(&neck_location, &neck_axis);

    let neck_radius = thickness / 4.0;
    let neck_height = height / 10.0;

    let mut cylinder = BRepPrimAPI_MakeCylinder_ctor(&neck_coord_system, neck_radius, neck_height);
    let cylinder_shape = cylinder.pin_mut().Shape();

    let mut fuse_neck = BRepAlgoAPI_Fuse_ctor(&body_shape, &cylinder_shape);
    let body_shape = fuse_neck.pin_mut().Shape();

    // Make the bottle hollow
    let mut face_explorer = TopExp_Explorer_ctor(&body_shape, TopAbs_ShapeEnum::TopAbs_FACE);
    let mut z_max = -1.0;
    let mut top_face: Option<UniquePtr<TopoDS_Face>> = None;

    while face_explorer.More() {
        let shape = ExplorerCurrentShape(&face_explorer);
        let face = TopoDS_cast_to_face(&shape);

        let surface = BRep_Tool_Surface(&face);
        let dynamic_type = DynamicType(&surface);
        let name = type_name(&dynamic_type);

        if name == "Geom_Plane" {
            let plane_handle = new_HandleGeomPlane_from_HandleGeomSurface(&surface);
            let plane_location = handle_geom_plane_location(&plane_handle);

            let plane_z = plane_location.Z();
            if plane_z > z_max {
                z_max = plane_z;
                top_face = Some(face);
            }
        }

        face_explorer.pin_mut().Next();
    }

    let top_face = top_face.unwrap();

    let mut faces_to_remove = new_list_of_shape();
    shape_list_append_face(faces_to_remove.pin_mut(), &top_face);

    // Export to an STL file
    let triangulation = BRepMesh_IncrementalMesh_ctor(body_shape, 0.1);
    let success = write_stl(stl_writer.pin_mut(), triangulation.Shape(), "output.stl".to_owned());

    println!("Done! Success = {}", success);
}
