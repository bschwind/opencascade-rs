use opencascade::Shape;

pub fn main() {
    let mut my_box = Shape::make_box(10.0, 10.0, 1.0);
    let another_box = Shape::make_box(1.0, 1.0, 0.8);

    my_box.subtract(&another_box);
    my_box.chamfer_edges(0.07);
    my_box.write_stl("box.stl");
}
