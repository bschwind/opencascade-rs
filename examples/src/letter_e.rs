use opencascade::{primitives::Shape, text::Font};

pub fn shape() -> Shape {
    let mut font = Font::from_path("/System/Library/Fonts/Supplemental/Courier New.ttf", 100.0);
    let glyph = font.render_glyph('E');
    glyph
}
