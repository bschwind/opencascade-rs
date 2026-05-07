use opencascade_sys as ffi;

#[test]
fn it_can_access_mesh_triangulation() {
    let origin = opencascade_sys::gp::new_point(0., 0., 0.);
    let mut cube = opencascade_sys::b_rep_prim_api::BRepPrimAPI_MakeBox_new(&origin, 10., 10., 10.);

    let mut mesh = opencascade_sys::b_rep_mesh::IncrementalMesh_new(cube.pin_mut().Shape(), 0.01);

    let mut triangle_corners = 0;

    let mut edge_explorer = opencascade_sys::top_exp::TopExp_Explorer_new(
        mesh.pin_mut().Shape(),
        opencascade_sys::top_abs::TopAbs_ShapeEnum::TopAbs_FACE,
    );
    while edge_explorer.More() {
        let face = opencascade_sys::topo_ds::TopoDS::Face(edge_explorer.Current());
        let mut location = opencascade_sys::top_loc::Location_new();

        let triangulation_handle =
            opencascade_sys::b_rep::BRep_Tool_Triangulation(face, location.pin_mut());
        if let Ok(triangulation) =
            opencascade_sys::poly::Handle_Poly_Triangulation_Get(&triangulation_handle)
        {
            for index in 0..triangulation.NbTriangles() {
                let triangle = triangulation.Triangle(index + 1);

                for corner_index in 1..=3 {
                    let _point = ffi::poly::Poly_Triangulation_Node(
                        triangulation,
                        triangle.Value(corner_index),
                    );
                    triangle_corners += 1;
                }
            }
        }

        edge_explorer.pin_mut().Next();
    }

    const SIDES: i32 = 6;
    const TRI_PER_SIDE: i32 = 2;
    const CORNER_PER_TRI: i32 = 3;
    assert_eq!(SIDES * TRI_PER_SIDE * CORNER_PER_TRI, triangle_corners)
}
