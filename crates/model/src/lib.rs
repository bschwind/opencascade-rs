use glam::{dvec3, DVec3};
use wasm_api::{Edge, Vertex};

#[no_mangle]
pub extern "C" fn model() {
    let _first_vertex = Vertex::new(dvec3(10.0, 15.0, -23.0));
    let _second_vertex = Vertex::new(dvec3(1.0, 2.0, 3.0));

    for i in 0..20000 {
        let _edge = Edge::segment(
            DVec3::ZERO,
            dvec3(i as f64 * 0.1, (i as f64 * 10.0).sin() * 20.0, (i as f64 * 10.0).cos() * 15.0),
        );
    }
}
