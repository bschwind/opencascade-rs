use crate::{
    primitives::{FaceOrientation, Shape},
    Error,
};
use cxx::UniquePtr;
use glam::{dvec2, dvec3, DVec2, DVec3};
use opencascade_sys::ffi;

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<DVec3>,
    pub uvs: Vec<DVec2>,
    pub normals: Vec<DVec3>,
    pub indices: Vec<usize>,
}

pub struct Mesher {
    pub(crate) inner: UniquePtr<ffi::BRepMeshIncrementalMesh>,
}

impl Mesher {
    pub fn try_new(shape: &Shape, triangulation_tolerance: f64) -> Result<Self, Error> {
        let inner = ffi::BRepMeshIncrementalMesh_ctor(&shape.inner, triangulation_tolerance);

        if inner.is_done() {
            Ok(Self { inner })
        } else {
            Err(Error::TriangulationFailed)
        }
    }

    pub fn mesh(mut self) -> Result<Mesh, Error> {
        let mut vertices = vec![];
        let mut uvs = vec![];
        let mut normals = vec![];
        let mut indices = vec![];

        let triangulated_shape = Shape::from_shape(self.inner.pin_mut().shape());

        for face in triangulated_shape.faces() {
            let mut location = ffi::TopLocLocation_ctor();

            let triangulation_handle =
                ffi::BRep_Tool_Triangulation(&face.inner, location.pin_mut());

            let triangulation = ffi::HandlePolyTriangulation_Get(&triangulation_handle)
                .map_err(|_| Error::UntriangulatedFace)?;

            let index_offset = vertices.len();
            let face_point_count = triangulation.nb_nodes();

            for i in 1..=face_point_count {
                let mut point = ffi::PolyTriangulation_Node(triangulation, i);
                point.pin_mut().transform(&ffi::TopLocLocation_Transformation(&location));
                vertices.push(dvec3(point.x(), point.y(), point.z()));
            }

            let mut u_min = f64::INFINITY;
            let mut v_min = f64::INFINITY;

            let mut u_max = f64::NEG_INFINITY;
            let mut v_max = f64::NEG_INFINITY;

            for i in 1..=(face_point_count) {
                let uv = ffi::PolyTriangulation_UV(triangulation, i);
                let (u, v) = (uv.x(), uv.y());

                u_min = u_min.min(u);
                v_min = v_min.min(v);

                u_max = u_max.max(u);
                v_max = v_max.max(v);

                uvs.push(dvec2(u, v));
            }

            // Normalize the newly added UV coordinates.
            for uv in &mut uvs[index_offset..(index_offset + face_point_count as usize)] {
                uv.x = (uv.x - u_min) / (u_max - u_min);
                uv.y = (uv.y - v_min) / (v_max - v_min);

                if face.orientation() != FaceOrientation::Forward {
                    uv.x = 1.0 - uv.x;
                }
            }

            // Add in the normals.
            // TODO(bschwind) - Use `location` to transform the normals.
            let normal_array = ffi::TColgpArray1OfDir_ctor(0, face_point_count);

            ffi::compute_normals(&face.inner, &triangulation_handle);

            // TODO(bschwind) - Why do we start at 1 here?
            for i in 1..(normal_array.length() as usize) {
                let normal = ffi::PolyTriangulation_Normal(triangulation, i as i32);
                normals.push(dvec3(normal.x(), normal.y(), normal.z()));
            }

            for i in 1..=triangulation.nb_triangles() {
                let triangle = triangulation.triangle(i);

                if face.orientation() == FaceOrientation::Forward {
                    indices.push(index_offset + triangle.value(1) as usize - 1);
                    indices.push(index_offset + triangle.value(2) as usize - 1);
                    indices.push(index_offset + triangle.value(3) as usize - 1);
                } else {
                    indices.push(index_offset + triangle.value(3) as usize - 1);
                    indices.push(index_offset + triangle.value(2) as usize - 1);
                    indices.push(index_offset + triangle.value(1) as usize - 1);
                }
            }
        }

        Ok(Mesh { vertices, uvs, normals, indices })
    }
}
