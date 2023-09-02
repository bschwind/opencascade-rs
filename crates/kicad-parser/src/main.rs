use anyhow::{anyhow, Result};
use glam::DVec3;
use opencascade::primitives::Shape;

mod board;
mod graphics;

fn main() -> Result<()> {
    let Some(input_file) = std::env::args().nth(1) else {
        return Err(anyhow!("Usage: kicad-parser <input_file.kicad_pcb>"));
    };

    let board = board::KicadBoard::from_file(input_file)?;
    let outline = board.outline(0.5);
    // let outline: Face = Into::<Face>::into(&board.graphic_rects[0]);

    let solid = outline.extrude(DVec3 { x: 0.0, y: 0.0, z: 10.0 });
    let mut shape: Shape = solid.into();
    shape.clean();
    shape.write_stl("outline.stl")?;

    Ok(())
}
