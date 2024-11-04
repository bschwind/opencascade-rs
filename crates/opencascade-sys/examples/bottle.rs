use cxx::UniquePtr;
use opencascade_sys::ffi::{
    cylinder_to_surface, dynamic_type, ellipse_to_HandleGeom2dCurve, ellipse_value, gp_DZ, gp_OX,
    handle_geom_plane_location, new_HandleGeomCurve_from_HandleGeomTrimmedCurve,
    new_HandleGeomPlane_from_HandleGeomSurface, new_list_of_shape, new_point, new_point_2d,
    new_transform, new_vec, shape_list_append_face, type_name, write_stl, BRepAlgoAPIFuse_ctor,
    BRepBuilderAPIMakeFace_wire, BRepBuilderAPITransform_ctor,
    BRepBuilderAPI_MakeEdge_CurveSurface2d, BRepBuilderAPI_MakeEdge_HandleGeomCurve,
    BRepBuilderAPI_MakeWire_ctor, BRepBuilderAPI_MakeWire_edge_edge,
    BRepBuilderAPI_MakeWire_edge_edge_edge, BRepBuilder_ctor, BRepBuilder_upcast_to_topodsbuilder,
    BRepFilletAPIMakeFillet_ctor, BRepLibBuildCurves3d, BRepMeshIncrementalMesh_ctor,
    BRepOffsetAPIMakeThickSolid_ctor, BRepOffsetAPIThruSections_ctor, BRepPrimAPIMakeCylinder_ctor,
    BRepPrimAPIMakePrism_ctor, BRep_Tool_Surface, ExplorerCurrentShape,
    GCE2dMakeSegment_point_point, GCMakeArcOfCircle_Value, GCMakeArcOfCircle_point_point_point,
    GCMakeSegment_Value, GCMakeSegment_point_point, Geom2dEllipse_ctor, Geom2dTrimmedCurve_ctor,
    GeomCylindricalSurface_ctor, GpAx2_ctor, GpAx2d_ctor, GpAx3_from_GpAx2, GpDir2d_ctor,
    HandleGeom2dTrimmedCurve_to_curve, MakeThickSolidByJoin, StlAPIWriter_ctor, TopAbsShapeEnum,
    TopExpExplorer_ctor, TopoDSCompound_as_shape, TopoDSCompound_ctor, TopoDSFace,
    TopoDSFace_to_owned, TopoDS_cast_to_edge, TopoDS_cast_to_face, TopoDS_cast_to_wire,
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
    let arc = GCMakeArcOfCircle_point_point_point(&point_2, &point_3, &point_4);
    let segment_1 = GCMakeSegment_point_point(&point_1, &point_2);
    let segment_2 = GCMakeSegment_point_point(&point_4, &point_5);

    let mut edge_1 = BRepBuilderAPI_MakeEdge_HandleGeomCurve(
        &new_HandleGeomCurve_from_HandleGeomTrimmedCurve(&GCMakeSegment_Value(&segment_1)),
    );

    let mut edge_2 = BRepBuilderAPI_MakeEdge_HandleGeomCurve(
        &new_HandleGeomCurve_from_HandleGeomTrimmedCurve(&GCMakeArcOfCircle_Value(&arc)),
    );

    let mut edge_3 = BRepBuilderAPI_MakeEdge_HandleGeomCurve(
        &new_HandleGeomCurve_from_HandleGeomTrimmedCurve(&GCMakeSegment_Value(&segment_2)),
    );

    let mut wire = BRepBuilderAPI_MakeWire_edge_edge_edge(
        edge_1.pin_mut().edge(),
        edge_2.pin_mut().edge(),
        edge_3.pin_mut().edge(),
    );

    let x_axis = gp_OX();

    let mut transform = new_transform();
    transform.pin_mut().set_mirror_axis(x_axis);

    // We're calling Shape() here instead of Wire(), hope that's okay.
    let mut brep_transform =
        BRepBuilderAPITransform_ctor(wire.pin_mut().shape(), &transform, false);
    let mirrored_shape = brep_transform.pin_mut().shape();
    let mirrored_wire = TopoDS_cast_to_wire(mirrored_shape);

    let mut make_wire = BRepBuilderAPI_MakeWire_ctor();
    make_wire.pin_mut().add_wire(wire.pin_mut().wire());
    make_wire.pin_mut().add_wire(mirrored_wire);

    let wire_profile = make_wire.pin_mut().wire();

    let mut face_profile = BRepBuilderAPIMakeFace_wire(wire_profile, false);
    let prism_vec = new_vec(0.0, 0.0, height);
    // We're calling Shape here instead of Face(), hope that's also okay.
    let mut body =
        BRepPrimAPIMakePrism_ctor(face_profile.pin_mut().shape(), &prism_vec, false, true);

    let mut make_fillet = BRepFilletAPIMakeFillet_ctor(body.pin_mut().shape());
    let mut edge_explorer =
        TopExpExplorer_ctor(body.pin_mut().shape(), TopAbsShapeEnum::TopAbs_EDGE);

    while edge_explorer.more() {
        let edge = TopoDS_cast_to_edge(edge_explorer.current());
        make_fillet.pin_mut().add_edge(thickness / 12.0, edge);
        edge_explorer.pin_mut().next();
    }

    let body_shape = make_fillet.pin_mut().shape();

    // Make the bottle neck
    let neck_location = new_point(0.0, 0.0, height);
    let neck_axis = gp_DZ();
    let neck_coord_system = GpAx2_ctor(&neck_location, neck_axis);

    let neck_radius = thickness / 4.0;
    let neck_height = height / 10.0;

    let mut cylinder = BRepPrimAPIMakeCylinder_ctor(&neck_coord_system, neck_radius, neck_height);
    let cylinder_shape = cylinder.pin_mut().shape();

    let mut fuse_neck = BRepAlgoAPIFuse_ctor(body_shape, cylinder_shape);
    let body_shape = fuse_neck.pin_mut().shape();

    // Make the bottle hollow
    let mut face_explorer = TopExpExplorer_ctor(body_shape, TopAbsShapeEnum::TopAbs_FACE);
    let mut z_max = -1.0;
    let mut top_face: Option<UniquePtr<TopoDSFace>> = None;

    while face_explorer.more() {
        let shape = ExplorerCurrentShape(&face_explorer);
        let face = TopoDS_cast_to_face(&shape);

        let surface = BRep_Tool_Surface(face);
        let dynamic_type = dynamic_type(&surface);
        let name = type_name(dynamic_type);

        if name == "Geom_Plane" {
            let plane_handle = new_HandleGeomPlane_from_HandleGeomSurface(&surface);
            let plane_location = handle_geom_plane_location(&plane_handle);

            let plane_z = plane_location.z();
            if plane_z > z_max {
                z_max = plane_z;
                top_face = Some(TopoDSFace_to_owned(face));
            }
        }

        face_explorer.pin_mut().next();
    }

    let top_face = top_face.unwrap();

    let mut faces_to_remove = new_list_of_shape();
    shape_list_append_face(faces_to_remove.pin_mut(), &top_face);

    let mut solid_maker = BRepOffsetAPIMakeThickSolid_ctor();
    MakeThickSolidByJoin(
        solid_maker.pin_mut(),
        body_shape,
        &faces_to_remove,
        -thickness / 50.0,
        1.0e-3,
    );

    let body_shape = solid_maker.pin_mut().shape();

    // Create the threading
    let cylinder_axis = GpAx3_from_GpAx2(&neck_coord_system);
    let cylinder_1 = GeomCylindricalSurface_ctor(&cylinder_axis, neck_radius * 0.99);
    let cylinder_1 = cylinder_to_surface(&cylinder_1);
    let cylinder_2 = GeomCylindricalSurface_ctor(&cylinder_axis, neck_radius * 1.05);
    let cylinder_2 = cylinder_to_surface(&cylinder_2);

    let a_pnt = new_point_2d(std::f64::consts::TAU, neck_height / 2.0);
    let a_dir = GpDir2d_ctor(std::f64::consts::TAU, neck_height / 4.0);
    let thread_axis = GpAx2d_ctor(&a_pnt, &a_dir);

    let a_major = std::f64::consts::TAU;
    let a_minor = neck_height / 10.0;

    let ellipse_1 = Geom2dEllipse_ctor(&thread_axis, a_major, a_minor);
    let ellipse_1_handle = ellipse_to_HandleGeom2dCurve(&ellipse_1);
    let ellipse_2 = Geom2dEllipse_ctor(&thread_axis, a_major, a_minor / 4.0);
    let ellipse_2_handle = ellipse_to_HandleGeom2dCurve(&ellipse_2);
    let arc_1 = Geom2dTrimmedCurve_ctor(&ellipse_1_handle, 0.0, std::f64::consts::PI);
    let arc_1 = HandleGeom2dTrimmedCurve_to_curve(&arc_1);
    let arc_2 = Geom2dTrimmedCurve_ctor(&ellipse_2_handle, 0.0, std::f64::consts::PI);
    let arc_2 = HandleGeom2dTrimmedCurve_to_curve(&arc_2);

    let ellipse_point_1 = ellipse_value(&ellipse_1, 0.0);
    let ellipse_point_2 = ellipse_value(&ellipse_1, std::f64::consts::PI);
    let thread_segment = GCE2dMakeSegment_point_point(&ellipse_point_1, &ellipse_point_2);
    let thread_segment = HandleGeom2dTrimmedCurve_to_curve(&thread_segment);

    let mut edge_1_on_surface_1 = BRepBuilderAPI_MakeEdge_CurveSurface2d(&arc_1, &cylinder_1);
    let mut edge_2_on_surface_1 =
        BRepBuilderAPI_MakeEdge_CurveSurface2d(&thread_segment, &cylinder_1);
    let mut edge_1_on_surface_2 = BRepBuilderAPI_MakeEdge_CurveSurface2d(&arc_2, &cylinder_2);
    let mut edge_2_on_surface_2 =
        BRepBuilderAPI_MakeEdge_CurveSurface2d(&thread_segment, &cylinder_2);

    let mut threading_wire_1 = BRepBuilderAPI_MakeWire_edge_edge(
        edge_1_on_surface_1.pin_mut().edge(),
        edge_2_on_surface_1.pin_mut().edge(),
    );
    let mut threading_wire_2 = BRepBuilderAPI_MakeWire_edge_edge(
        edge_1_on_surface_2.pin_mut().edge(),
        edge_2_on_surface_2.pin_mut().edge(),
    );

    // TODO - does calling Shape() work here instead of Wire()?
    BRepLibBuildCurves3d(threading_wire_1.pin_mut().shape());
    BRepLibBuildCurves3d(threading_wire_2.pin_mut().shape());

    let is_solid = true;
    let mut threading_loft = BRepOffsetAPIThruSections_ctor(is_solid);
    threading_loft.pin_mut().add_wire(threading_wire_1.pin_mut().wire());
    threading_loft.pin_mut().add_wire(threading_wire_2.pin_mut().wire());
    threading_loft.pin_mut().check_compatibility(false);

    let threading_shape = threading_loft.pin_mut().shape();

    // Build the resulting compound
    let mut compound = TopoDSCompound_ctor();
    let builder = BRepBuilder_ctor();
    let builder = BRepBuilder_upcast_to_topodsbuilder(&builder);
    builder.make_compound(compound.pin_mut());

    let mut compound_shape = TopoDSCompound_as_shape(compound);
    builder.add(compound_shape.pin_mut(), body_shape);
    builder.add(compound_shape.pin_mut(), threading_shape);

    let final_shape = compound_shape;

    // Export to an STL file
    let mut stl_writer = StlAPIWriter_ctor();
    let triangulation = BRepMeshIncrementalMesh_ctor(&final_shape, 0.01);
    let success = write_stl(stl_writer.pin_mut(), triangulation.shape(), "bottle.stl".to_owned());

    println!("Done! Success = {success}");
}
