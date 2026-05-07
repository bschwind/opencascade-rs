use cxx::UniquePtr;
use opencascade_sys as ffi;

// All dimensions are in millimeters.
pub fn main() {
    let height = 70.0;
    let width = 50.0;
    let thickness = 30.0;

    // Define the points making up the bottle's profile.
    let point_1 = ffi::gp::new_point(-width / 2.0, 0.0, 0.0);
    let point_2 = ffi::gp::new_point(-width / 2.0, -thickness / 4.0, 0.0);
    let point_3 = ffi::gp::new_point(0.0, -thickness / 2.0, 0.0);
    let point_4 = ffi::gp::new_point(width / 2.0, -thickness / 4.0, 0.0);
    let point_5 = ffi::gp::new_point(width / 2.0, 0.0, 0.0);

    // Define the arcs and segments of the profile.
    let arc = ffi::gc::GC_MakeArcOfCircle_point_point_point(&point_2, &point_3, &point_4);
    let segment_1 = ffi::gc::GC_MakeSegment_point_point(&point_1, &point_2);
    let segment_2 = ffi::gc::GC_MakeSegment_point_point(&point_4, &point_5);

    let mut edge_1 = ffi::b_rep_builder_api::BRepBuilderAPI_MakeEdge_HandleGeomCurve(
        &ffi::geom::new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(
            &ffi::gc::GC_MakeSegment_Value(&segment_1),
        ),
    );

    let mut edge_2 = ffi::b_rep_builder_api::BRepBuilderAPI_MakeEdge_HandleGeomCurve(
        &ffi::geom::new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(
            &ffi::gc::GC_MakeArcOfCircle_Value(&arc),
        ),
    );

    let mut edge_3 = ffi::b_rep_builder_api::BRepBuilderAPI_MakeEdge_HandleGeomCurve(
        &ffi::geom::new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(
            &ffi::gc::GC_MakeSegment_Value(&segment_2),
        ),
    );

    let mut wire = ffi::b_rep_builder_api::BRepBuilderAPI_MakeWire_edge_edge_edge(
        edge_1.pin_mut().Edge(),
        edge_2.pin_mut().Edge(),
        edge_3.pin_mut().Edge(),
    );

    let x_axis = ffi::gp::gp::OX();

    let mut transform = ffi::gp::new_transform();
    transform.pin_mut().set_mirror_axis(x_axis);

    // We're calling Shape() here instead of Wire(), hope that's okay.
    let mut brep_transform = ffi::b_rep_builder_api::BRepBuilderAPI_Transform_new(
        wire.pin_mut().Shape(),
        &transform,
        false,
    );
    let mirrored_shape = brep_transform.pin_mut().Shape();
    let mirrored_wire = ffi::topo_ds::TopoDS::Wire(mirrored_shape);

    let mut make_wire = ffi::b_rep_builder_api::BRepBuilderAPI_MakeWire_new();
    make_wire.pin_mut().add_wire(wire.pin_mut().Wire());
    make_wire.pin_mut().add_wire(mirrored_wire);

    let wire_profile = make_wire.pin_mut().Wire();

    let mut face_profile =
        ffi::b_rep_builder_api::BRepBuilderAPI_MakeFace_wire(wire_profile, false);
    let prism_vec = ffi::gp::new_vec(0.0, 0.0, height);
    // We're calling Shape here instead of Face(), hope that's also okay.
    let mut body = ffi::b_rep_prim_api::BRepPrimAPI_MakePrism_new(
        face_profile.pin_mut().Shape(),
        &prism_vec,
        false,
        true,
    );

    let mut make_fillet =
        ffi::b_rep_fillet_api::BRepFilletAPI_MakeFillet_new(body.pin_mut().Shape());
    let mut edge_explorer = ffi::top_exp::TopExp_Explorer_new(
        body.pin_mut().Shape(),
        ffi::top_abs::TopAbs_ShapeEnum::TopAbs_EDGE,
    );

    while edge_explorer.More() {
        let edge = ffi::topo_ds::TopoDS::Edge(edge_explorer.Current());
        make_fillet.pin_mut().add_edge(thickness / 12.0, edge);
        edge_explorer.pin_mut().Next();
    }

    let body_shape = make_fillet.pin_mut().Shape();

    // Make the bottle neck
    let neck_location = ffi::gp::new_point(0.0, 0.0, height);
    let neck_axis = ffi::gp::gp::DZ();
    let neck_coord_system = ffi::gp::gp_Ax2_new(&neck_location, neck_axis);

    let neck_radius = thickness / 4.0;
    let neck_height = height / 10.0;

    let mut cylinder = ffi::b_rep_prim_api::BRepPrimAPI_MakeCylinder_new(
        &neck_coord_system,
        neck_radius,
        neck_height,
    );
    let cylinder_shape = cylinder.pin_mut().Shape();

    let mut fuse_neck = ffi::b_rep_algo_api::BRepAlgoAPI_Fuse_new(body_shape, cylinder_shape);
    let body_shape = fuse_neck.pin_mut().Shape();

    // Make the bottle hollow
    let mut face_explorer =
        ffi::top_exp::TopExp_Explorer_new(body_shape, ffi::top_abs::TopAbs_ShapeEnum::TopAbs_FACE);
    let mut z_max = -1.0;
    let mut top_face: Option<UniquePtr<ffi::topo_ds::TopoDS_Face>> = None;

    while face_explorer.More() {
        let shape = ffi::top_exp::ExplorerCurrentShape(&face_explorer);
        let face = ffi::topo_ds::TopoDS::Face(&shape);

        let surface = ffi::b_rep::BRep_Tool_Surface(face);
        let dynamic_type = ffi::geom::DynamicType(&surface);
        let name = ffi::standard::type_name(dynamic_type);

        if name == "Geom_Plane" {
            let plane_handle = ffi::geom::new_HandleGeomPlane_from_HandleGeomSurface(&surface);
            let plane_location = ffi::geom::handle_geom_plane_location(&plane_handle);

            let plane_z = plane_location.Z();
            if plane_z > z_max {
                z_max = plane_z;
                top_face = Some(ffi::topo_ds::TopoDS_Face_to_owned(face));
            }
        }

        face_explorer.pin_mut().Next();
    }

    let top_face = top_face.unwrap();

    let mut faces_to_remove = ffi::top_tools::new_list_of_shape();
    faces_to_remove.pin_mut().Append(ffi::topo_ds::cast_face_to_shape(&top_face));

    let mut solid_maker = ffi::b_rep_offset_api::BRepOffsetAPI_MakeThickSolid_new();

    let offset_mode = ffi::b_rep_offset_api::BRepOffset_Mode::BRepOffset_Skin;
    let intersection = false;
    let self_intersection = false;
    let join_type = ffi::geom_abs::GeomAbs_JoinType::GeomAbs_Arc;
    let remove_intersecting_edges = false;

    solid_maker.pin_mut().MakeThickSolidByJoin(
        body_shape,
        &faces_to_remove,
        -thickness / 50.0,
        1.0e-3,
        offset_mode,
        intersection,
        self_intersection,
        join_type,
        remove_intersecting_edges,
        &ffi::message::Message_ProgressRange_new(),
    );

    let body_shape = solid_maker.pin_mut().Shape();

    // Create the threading
    let cylinder_axis = ffi::gp::gp_Ax3_from_gp_Ax2(&neck_coord_system);
    let cylinder_1 = ffi::geom::Geom_CylindricalSurface_new(&cylinder_axis, neck_radius * 0.99);
    let cylinder_1 = ffi::geom::cylinder_to_surface(&cylinder_1);
    let cylinder_2 = ffi::geom::Geom_CylindricalSurface_new(&cylinder_axis, neck_radius * 1.05);
    let cylinder_2 = ffi::geom::cylinder_to_surface(&cylinder_2);

    let a_pnt = ffi::gp::new_point_2d(std::f64::consts::TAU, neck_height / 2.0);
    let a_dir = ffi::gp::gp_Dir2d_new(std::f64::consts::TAU, neck_height / 4.0);
    let thread_axis = ffi::gp::gp_Ax2d_new(&a_pnt, &a_dir);

    let a_major = std::f64::consts::TAU;
    let a_minor = neck_height / 10.0;

    let ellipse_1 = ffi::geom2d::Geom2d_Ellipse_new(&thread_axis, a_major, a_minor);
    let ellipse_1_handle = ffi::geom2d::ellipse_to_HandleGeom2d_Curve(&ellipse_1);
    let ellipse_2 = ffi::geom2d::Geom2d_Ellipse_new(&thread_axis, a_major, a_minor / 4.0);
    let ellipse_2_handle = ffi::geom2d::ellipse_to_HandleGeom2d_Curve(&ellipse_2);
    let arc_1 = ffi::geom2d::Geom2d_TrimmedCurve_new(&ellipse_1_handle, 0.0, std::f64::consts::PI);
    let arc_1 = ffi::geom2d::HandleGeom2d_TrimmedCurve_to_curve(&arc_1);
    let arc_2 = ffi::geom2d::Geom2d_TrimmedCurve_new(&ellipse_2_handle, 0.0, std::f64::consts::PI);
    let arc_2 = ffi::geom2d::HandleGeom2d_TrimmedCurve_to_curve(&arc_2);

    let ellipse_point_1 = ffi::geom2d::ellipse_value(&ellipse_1, 0.0);
    let ellipse_point_2 = ffi::geom2d::ellipse_value(&ellipse_1, std::f64::consts::PI);
    let thread_segment = ffi::gc::GCE2d_MakeSegment_point_point(&ellipse_point_1, &ellipse_point_2);
    let thread_segment = ffi::geom2d::HandleGeom2d_TrimmedCurve_to_curve(&thread_segment);

    let mut edge_1_on_surface_1 =
        ffi::b_rep_builder_api::BRepBuilderAPI_MakeEdge_CurveSurface2d(&arc_1, &cylinder_1);
    let mut edge_2_on_surface_1 = ffi::b_rep_builder_api::BRepBuilderAPI_MakeEdge_CurveSurface2d(
        &thread_segment,
        &cylinder_1,
    );
    let mut edge_1_on_surface_2 =
        ffi::b_rep_builder_api::BRepBuilderAPI_MakeEdge_CurveSurface2d(&arc_2, &cylinder_2);
    let mut edge_2_on_surface_2 = ffi::b_rep_builder_api::BRepBuilderAPI_MakeEdge_CurveSurface2d(
        &thread_segment,
        &cylinder_2,
    );

    let mut threading_wire_1 = ffi::b_rep_builder_api::BRepBuilderAPI_MakeWire_edge_edge(
        edge_1_on_surface_1.pin_mut().Edge(),
        edge_2_on_surface_1.pin_mut().Edge(),
    );
    let mut threading_wire_2 = ffi::b_rep_builder_api::BRepBuilderAPI_MakeWire_edge_edge(
        edge_1_on_surface_2.pin_mut().Edge(),
        edge_2_on_surface_2.pin_mut().Edge(),
    );

    // TODO - does calling Shape() work here instead of Wire()?
    ffi::b_rep_lib::BRepLib::BuildCurves3d(threading_wire_1.pin_mut().Shape());
    ffi::b_rep_lib::BRepLib::BuildCurves3d(threading_wire_2.pin_mut().Shape());

    let is_solid = true;
    let mut threading_loft = ffi::b_rep_offset_api::BRepOffsetAPI_ThruSections_new(is_solid);
    threading_loft.pin_mut().AddWire(threading_wire_1.pin_mut().Wire());
    threading_loft.pin_mut().AddWire(threading_wire_2.pin_mut().Wire());
    threading_loft.pin_mut().CheckCompatibility(false);

    let threading_shape = threading_loft.pin_mut().Shape();

    // Build the resulting compound
    let mut compound = ffi::topo_ds::TopoDS_Compound_new();
    let builder = ffi::b_rep::BRep_Builder_new();
    let builder = ffi::b_rep::BRep_Builder_upcast_to_topods_builder(&builder);
    builder.MakeCompound(compound.pin_mut());

    let mut compound_shape = ffi::topo_ds::TopoDS_Compound_as_shape(compound);
    builder.Add(compound_shape.pin_mut(), body_shape);
    builder.Add(compound_shape.pin_mut(), threading_shape);

    let final_shape = compound_shape;

    // Export to an STL file
    let mut stl_writer = ffi::stl_api::StlAPI_Writer_new();
    let triangulation = ffi::b_rep_mesh::IncrementalMesh_new(&final_shape, 0.01);
    let success = ffi::stl_api::write_stl(
        stl_writer.pin_mut(),
        triangulation.Shape(),
        "bottle.stl".to_owned(),
    );
    // let success = stl_writer.pin_mut().write_stl(triangulation.Shape(), "bottle.stl".to_owned());

    println!("Done! Success = {success}");
}
