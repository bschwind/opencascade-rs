use opencascade::{adhoc::AdHocShape, primitives::Shape};

pub fn shape() -> Shape {
    let my_box = AdHocShape::make_box(10.0, 10.0, 1.0);
    let another_box = AdHocShape::make_box(1.0, 1.0, 0.8);

    my_box.subtract(&another_box).chamfer(0.07)
}
