use opencascade::Shape;

pub fn main() {
    let mut my_box = Shape::make_box();
    my_box.chamfer_edges(0.07);
    my_box.write_stl("box.stl");
}
