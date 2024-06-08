use glam::dvec3;
use model_api::{
    primitives::{Direction, IntoShape, Shape},
    workplane::Workplane,
    Model,
};

struct RoundedChamfer {}

impl Model for RoundedChamfer {
    fn new() -> Self {
        Self {}
    }

    fn create_model(&mut self) -> Shape {
        let shape = Workplane::xy()
            .rect(16.0, 10.0)
            .fillet(1.0)
            .to_face()
            .extrude(dvec3(0.0, 0.0, 3.0))
            .into_shape();

        let top_edges = shape.faces().farthest(Direction::PosZ).edges();

        shape.chamfer_edges(0.7, top_edges)
    }
}

model_api::register_model!(RoundedChamfer);
