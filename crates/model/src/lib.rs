use glam::dvec3;
use wasm_api::Vertex;

#[no_mangle]
pub extern "C" fn model() {
    let _first_vertex = Vertex::new(dvec3(10.0, 15.0, -23.0));
    let _second_vertex = Vertex::new(dvec3(1.0, 2.0, 3.0));
}
