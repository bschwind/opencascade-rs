use glam::dvec3;
use model_api::{
    primitives::{Direction, IntoShape, Shape},
    workplane::Workplane,
    Model,
};

struct CableBracket {}

impl Model for CableBracket {
    fn new() -> Self {
        Self {}
    }

    fn create_model(&mut self) -> Shape {
        let shape = Workplane::xy()
            .rect(16.0, 10.0)
            .fillet(2.0)
            .to_face()
            .extrude(dvec3(0.0, 0.0, 1.0))
            .into_shape();

        let _top_edges = shape.faces().farthest(Direction::PosZ).edges();

        // shape.chamfer_edges(0.7, top_edges)

        shape
    }
}

model_api::register_model!(CableBracket);
