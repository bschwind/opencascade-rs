use glam::DVec3;
use opencascade::{primitives::Shape, text::Font};

pub fn shape() -> Shape {
    let mut font = Font::from_path("/System/Library/Fonts/Supplemental/Courier New.ttf", 100.0);
    let glyph = font.render_glyph('E');
    glyph.extrude(DVec3::new(0.0, 0.0, 10.0))
}
