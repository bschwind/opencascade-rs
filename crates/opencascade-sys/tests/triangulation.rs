use opencascade_sys::ffi::{
    new_point, BRepMesh_IncrementalMesh_ctor, BRepPrimAPI_MakeBox_ctor, BRep_Tool_Triangulation,
    HandlePoly_Triangulation_Get, TopAbs_ShapeEnum, TopExp_Explorer_ctor, TopLoc_Location_ctor,
    TopoDS_cast_to_face,
};

#[test]
fn it_can_access_mesh_triangulation() {
    let origin = new_point(0., 0., 0.);
    let mut cube = BRepPrimAPI_MakeBox_ctor(&origin, 10., 10., 10.);

    let mut mesh = BRepMesh_IncrementalMesh_ctor(cube.pin_mut().Shape(), 0.01);

    let mut triangle_corners = 0;

    let mut edge_explorer =
        TopExp_Explorer_ctor(mesh.pin_mut().Shape(), TopAbs_ShapeEnum::TopAbs_FACE);
    while edge_explorer.More() {
        let face = TopoDS_cast_to_face(edge_explorer.Current());
        let mut location = TopLoc_Location_ctor();

        let triangulation_handle = BRep_Tool_Triangulation(face, location.pin_mut());
        if let Ok(triangulation) = HandlePoly_Triangulation_Get(&triangulation_handle) {
            for index in 0..triangulation.NbTriangles() {
                let triangle = triangulation.Triangle(index + 1);

                for corner_index in 1..=3 {
                    let _point = triangulation.node(triangle.Value(corner_index));
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
