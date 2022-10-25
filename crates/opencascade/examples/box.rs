use opencascade::Shape;

pub fn main() {
    let mut my_box = Shape::make_box();
    my_box.fillet_edges(0.01);
    my_box.write_stl("box.stl");
}
