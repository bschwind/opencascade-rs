use opencascade_sys::ffi::{
    new_point, BRepMeshIncrementalMesh_ctor, BRepPrimAPIMakeBox_ctor, BRep_Tool_Triangulation,
    HandlePolyTriangulation_Get, PolyTriangulation_Node, TopAbsShapeEnum, TopExpExplorer_ctor,
    TopLocLocation_ctor, TopoDS_cast_to_face,
};

#[test]
fn it_can_access_mesh_triangulation() {
    let origin = new_point(0., 0., 0.);
    let mut cube = BRepPrimAPIMakeBox_ctor(&origin, 10., 10., 10.);

    let mut mesh = BRepMeshIncrementalMesh_ctor(cube.pin_mut().shape(), 0.01);

    let mut triangle_corners = 0;

    let mut edge_explorer =
        TopExpExplorer_ctor(mesh.pin_mut().shape(), TopAbsShapeEnum::TopAbs_FACE);
    while edge_explorer.more() {
        let face = TopoDS_cast_to_face(edge_explorer.current());
        let mut location = TopLocLocation_ctor();

        let triangulation_handle = BRep_Tool_Triangulation(face, location.pin_mut());
        if let Ok(triangulation) = HandlePolyTriangulation_Get(&triangulation_handle) {
            for index in 0..triangulation.nb_triangles() {
                let triangle = triangulation.triangle(index + 1);

                for corner_index in 1..=3 {
                    let _point =
                        PolyTriangulation_Node(triangulation, triangle.value(corner_index));
                    triangle_corners += 1;
                }
            }
        }

        edge_explorer.pin_mut().next();
    }

    const SIDES: i32 = 6;
    const TRI_PER_SIDE: i32 = 2;
    const CORNER_PER_TRI: i32 = 3;
    assert_eq!(SIDES * TRI_PER_SIDE * CORNER_PER_TRI, triangle_corners)
}
