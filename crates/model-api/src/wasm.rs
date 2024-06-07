use crate::Model;
use glam::DVec3;

// Use a procedural macro to generate bindings for the world we specified in
// `host.wit`
wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "model-world",
    // "init-model" is skipped because it is exported manually below.
    skip: ["init-model"],
});

impl From<DVec3> for Point3 {
    fn from(p: DVec3) -> Self {
        Self { x: p.x, y: p.y, z: p.z }
    }
}

impl From<Point3> for DVec3 {
    fn from(p: Point3) -> Self {
        Self { x: p.x, y: p.y, z: p.z }
    }
}

// Define a custom type and implement the generated `Guest` trait for it which
// represents implementing all the necessary exported interfaces for this
// component.
pub struct CadModelCode;

impl Guest for CadModelCode {
    fn run() -> Shape {
        print("Hello, world!");
        let user_shape = model().create_model();

        user_shape.inner
    }
}

static mut MODEL: Option<Box<dyn Model>> = None;

#[doc(hidden)]
pub fn register_model(build_model: fn() -> Box<dyn Model>) {
    unsafe { MODEL = Some((build_model)()) }
}

fn model() -> &'static mut dyn Model {
    unsafe { MODEL.as_deref_mut().unwrap() }
}

#[macro_export]
macro_rules! register_model {
    ($model_type:ty) => {
        #[export_name = "init-model"]
        pub extern "C" fn __init_model() {
            model_api::wasm::register_model(|| Box::new(<$model_type as model_api::Model>::new()));
        }
    };
}

// export! defines that the `CadModelCode` struct defined below is going to define
// the exports of the `world`, namely the `run` function.
export!(CadModelCode);
