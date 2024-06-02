use model_api::{workplane::Workplane, Model};

struct CableBracket {}

impl Model for CableBracket {
    fn new() -> Self {
        Self {}
    }

    fn create_model(&mut self) {
        let _shape = Workplane::xy().rect(16.0, 10.0);
    }
}

model_api::register_model!(CableBracket);
