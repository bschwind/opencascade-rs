use cxx::UniquePtr;
use opencascade_sys::ffi::{
    cylinder_to_surface, ellipse_to_HandleGeom2d_Curve, ellipse_value, gp_Ax2_ctor, gp_Ax2d_ctor,
    gp_Ax3_from_gp_Ax2, gp_DZ, gp_Dir2d_ctor, gp_OX, handle_geom_plane_location,
    new_HandleGeomCurve_from_HandleGeom_TrimmedCurve, new_HandleGeomPlane_from_HandleGeomSurface,
    new_list_of_shape, new_point, new_point_2d, new_transform, new_vec, shape_list_append_face,
    type_name, write_stl, BRepAlgoAPI_Fuse_ctor, BRepBuilderAPI_MakeEdge_CurveSurface2d,
    BRepBuilderAPI_MakeEdge_HandleGeomCurve, BRepBuilderAPI_MakeFace_wire,
    BRepBuilderAPI_MakeWire_ctor, BRepBuilderAPI_MakeWire_edge_edge,
    BRepBuilderAPI_MakeWire_edge_edge_edge, BRepBuilderAPI_Transform_ctor,
    BRepFilletAPI_MakeFillet_ctor, BRepLibBuildCurves3d, BRepMesh_IncrementalMesh_ctor,
    BRepOffsetAPI_MakeThickSolid_ctor, BRepOffsetAPI_ThruSections_ctor,
    BRepPrimAPI_MakeCylinder_ctor, BRepPrimAPI_MakePrism_ctor, BRep_Builder_ctor,
    BRep_Builder_upcast_to_topods_builder, BRep_Tool_Surface, DynamicType, ExplorerCurrentShape,
    GCE2d_MakeSegment_point_point, GC_MakeArcOfCircle_Value, GC_MakeArcOfCircle_point_point_point,
    GC_MakeSegment_Value, GC_MakeSegment_point_point, Geom2d_Ellipse_ctor,
    Geom2d_TrimmedCurve_ctor, Geom_CylindricalSurface_ctor, HandleGeom2d_TrimmedCurve_to_curve,
    MakeThickSolidByJoin, StlAPI_Writer_ctor, TopAbs_ShapeEnum, TopExp_Explorer_ctor,
    TopoDS_Compound_as_shape, TopoDS_Compound_ctor, TopoDS_Face, TopoDS_cast_to_edge,
    TopoDS_cast_to_face, TopoDS_cast_to_wire,
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

    let mut solid_maker = BRepOffsetAPI_MakeThickSolid_ctor();
    MakeThickSolidByJoin(
        solid_maker.pin_mut(),
        &body_shape,
        &faces_to_remove,
        -thickness / 50.0,
        1.0e-3,
    );

    let body_shape = solid_maker.pin_mut().Shape();

    // Create the threading
    let cylinder_axis = gp_Ax3_from_gp_Ax2(&neck_coord_system);
    let cylinder_1 = Geom_CylindricalSurface_ctor(&cylinder_axis, neck_radius * 0.99);
    let cylinder_1 = cylinder_to_surface(&cylinder_1);
    let cylinder_2 = Geom_CylindricalSurface_ctor(&cylinder_axis, neck_radius * 1.05);
    let cylinder_2 = cylinder_to_surface(&cylinder_2);

    let a_pnt = new_point_2d(std::f64::consts::TAU, neck_height / 2.0);
    let a_dir = gp_Dir2d_ctor(std::f64::consts::TAU, neck_height / 4.0);
    let thread_axis = gp_Ax2d_ctor(&a_pnt, &a_dir);

    let a_major = std::f64::consts::TAU;
    let a_minor = neck_height / 10.0;

    let ellipse_1 = Geom2d_Ellipse_ctor(&thread_axis, a_major, a_minor);
    let ellipse_1_handle = ellipse_to_HandleGeom2d_Curve(&ellipse_1);
    let ellipse_2 = Geom2d_Ellipse_ctor(&thread_axis, a_major, a_minor / 4.0);
    let ellipse_2_handle = ellipse_to_HandleGeom2d_Curve(&ellipse_2);
    let arc_1 = Geom2d_TrimmedCurve_ctor(&ellipse_1_handle, 0.0, std::f64::consts::PI);
    let arc_1 = HandleGeom2d_TrimmedCurve_to_curve(&arc_1);
    let arc_2 = Geom2d_TrimmedCurve_ctor(&ellipse_2_handle, 0.0, std::f64::consts::PI);
    let arc_2 = HandleGeom2d_TrimmedCurve_to_curve(&arc_2);

    let ellipse_point_1 = ellipse_value(&ellipse_1, 0.0);
    let ellipse_point_2 = ellipse_value(&ellipse_1, std::f64::consts::PI);
    let thread_segment = GCE2d_MakeSegment_point_point(&ellipse_point_1, &ellipse_point_2);
    let thread_segment = HandleGeom2d_TrimmedCurve_to_curve(&thread_segment);

    let mut edge_1_on_surface_1 = BRepBuilderAPI_MakeEdge_CurveSurface2d(&arc_1, &cylinder_1);
    let mut edge_2_on_surface_1 =
        BRepBuilderAPI_MakeEdge_CurveSurface2d(&thread_segment, &cylinder_1);
    let mut edge_1_on_surface_2 = BRepBuilderAPI_MakeEdge_CurveSurface2d(&arc_2, &cylinder_2);
    let mut edge_2_on_surface_2 =
        BRepBuilderAPI_MakeEdge_CurveSurface2d(&thread_segment, &cylinder_2);

    let mut threading_wire_1 = BRepBuilderAPI_MakeWire_edge_edge(
        edge_1_on_surface_1.pin_mut().Edge(),
        edge_2_on_surface_1.pin_mut().Edge(),
    );
    let mut threading_wire_2 = BRepBuilderAPI_MakeWire_edge_edge(
        edge_1_on_surface_2.pin_mut().Edge(),
        edge_2_on_surface_2.pin_mut().Edge(),
    );

    // TODO - does calling Shape() work here instead of Wire()?
    BRepLibBuildCurves3d(threading_wire_1.pin_mut().Shape());
    BRepLibBuildCurves3d(threading_wire_2.pin_mut().Shape());

    let is_solid = true;
    let mut threading_loft = BRepOffsetAPI_ThruSections_ctor(is_solid);
    threading_loft.pin_mut().AddWire(threading_wire_1.pin_mut().Wire());
    threading_loft.pin_mut().AddWire(threading_wire_2.pin_mut().Wire());
    threading_loft.pin_mut().CheckCompatibility(false);

    let threading_shape = threading_loft.pin_mut().Shape();

    // Build the resulting compound
    let mut compound = TopoDS_Compound_ctor();
    let builder = BRep_Builder_ctor();
    let builder = BRep_Builder_upcast_to_topods_builder(&builder);
    builder.MakeCompound(compound.pin_mut());

    let mut compound_shape = TopoDS_Compound_as_shape(compound);
    builder.Add(compound_shape.pin_mut(), &body_shape);
    builder.Add(compound_shape.pin_mut(), &threading_shape);

    let final_shape = compound_shape;

    // Export to an STL file
    let triangulation = BRepMesh_IncrementalMesh_ctor(&final_shape, 0.01);
    let success = write_stl(stl_writer.pin_mut(), triangulation.Shape(), "output.stl".to_owned());

    println!("Done! Success = {}", success);
}
