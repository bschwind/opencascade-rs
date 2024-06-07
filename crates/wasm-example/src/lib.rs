use model_api::{primitives::Shape, workplane::Workplane, Model};

struct CableBracket {}

impl Model for CableBracket {
    fn new() -> Self {
        Self {}
    }

    fn create_model(&mut self) -> Shape {
        let wire = Workplane::xy().rect(16.0, 10.0).fillet(1.0);
        Shape::from_wire(&wire)
    }
}

model_api::register_model!(CableBracket);
