mod box_shape;
mod rounded_chamfer;

model_api::register_model!(rounded_chamfer::RoundedChamfer);
// model_api::register_model!(box_shape::BoxShape);

// model_api::register_model_fn!({
//     let my_box = Shape::box_with_dimensions(10.0, 10.0, 1.0);
//     let another_box = Shape::box_with_dimensions(1.0, 1.0, 0.8);

//     my_box.subtract(&another_box).shape.chamfer(0.07)
// });
