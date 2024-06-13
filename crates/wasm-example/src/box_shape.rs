use model_api::{primitives::Shape, Model};

pub struct BoxShape {}

impl Model for BoxShape {
    fn new() -> Self {
        Self {}
    }

    fn create_model(&mut self) -> Shape {
        let my_box = Shape::box_with_dimensions(10.0, 10.0, 1.0);
        let another_box = Shape::box_with_dimensions(1.0, 1.0, 0.8);

        my_box.subtract(&another_box).chamfer(0.07)
    }
}
