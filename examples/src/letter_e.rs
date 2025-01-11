use opencascade::{
    primitives::Shape,
    text::{Font, FontAspect},
};

pub fn shape() -> Shape {
    let mut font = Font::from_name("Arial", FontAspect::Regular, 100.0);
    let glyph = font.render_glyph('E');
    glyph
}
