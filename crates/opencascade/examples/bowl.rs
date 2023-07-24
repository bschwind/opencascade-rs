use glam::dvec3;
use opencascade::{
    primitives::{self, Direction,  Solid},
    workplane::Workplane,
};

pub fn main() {
    let bot_rad: f64 = 30.0;
    let top_rad: f64 = 40.0;
    let height: f64 = 30.0;
    let thickness: f64 = 2.0;
    // the base bowl
    let mut loft = bowly_shape(bot_rad, top_rad, height, 0.);

    // inner ( shell does not exist yest)
    let inner = bowly_shape(bot_rad - thickness, top_rad - thickness, height, thickness);
    (loft, _) = loft.subtract_shape(&inner);

    // rouind out the top
    let top_side = loft.faces().farthest(Direction::PosZ).edges();
    loft.fillet_edges(thickness / 2.0, top_side);

    loft.write_stl("bowl.stl").unwrap();
}

fn bowly_shape(bot_rad: f64, top_rad: f64, height: f64, offset: f64) -> primitives::Shape {
    let bottom = Workplane::xy().circle(0., 0., bot_rad);
    let mut top = Workplane::xy().circle(0., 0., top_rad);
    top.translate(dvec3(0., 0., height));
    let mut loft = Solid::loft([&bottom, &top]).to_shape();
    loft.set_global_translation(dvec3(0., 0., offset));
    // round out the bottom
    let bottom_side = loft.faces().farthest(Direction::NegZ).edges();
    loft.fillet_edges(4., bottom_side);

    loft
}
