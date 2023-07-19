use anyhow::{anyhow, Context, Result};
use sexp::{Atom, Sexp};
use std::path::Path;

fn main() -> Result<()> {
    let Some(input_file) = std::env::args().nth(1) else {
        return Err(anyhow!("Usage: kicad-parser <input_file.kicad_pcb>"));
    };

    let board = KicadBoard::from_file(input_file)?;

    dbg!(board);

    Ok(())
}

#[derive(Debug, Clone, Default)]
pub struct KicadBoard {
    graphic_lines: Vec<GraphicLine>,
}

impl KicadBoard {
    pub fn from_file<P: AsRef<Path>>(file: P) -> Result<Self> {
        let kicad_board_str = std::fs::read_to_string(&file)
            .context(format!("Reading {:?}", file.as_ref().to_string_lossy()))?;
        let sexp = sexp::parse(&kicad_board_str)?;

        let Sexp::List(list) = sexp else {
            return Err(anyhow!("Top level file wasn't a list"));
        };

        let Sexp::Atom(Atom::S(head)) = &list[0] else {
            return Err(anyhow!("First element in the top level list should be a string"));
        };

        match head.as_str() {
            "kicad_pcb" => {
                let board_fields = &list[1..];
                Ok(Self::handle_board_fields(board_fields)?)
            },
            _ => Err(anyhow!("Invalid top-level file type - expected 'kicad_pcb'")),
        }
    }

    fn handle_board_fields(fields: &[Sexp]) -> Result<Self> {
        let mut board = Self::default();

        for field in fields {
            let Sexp::List(list) = field else {
                continue;
            };

            let Sexp::Atom(Atom::S(head)) = &list[0] else {
                continue;
            };

            let rest = &list[1..];

            match head.as_str() {
                "version" => {},
                "generator" => {},
                "general" => {},
                "paper" => {},
                "layers" => {},
                "footprint" => {},
                "gr_arc" => {},
                "gr_line" => {
                    let line = GraphicLine::from_list(rest)?;
                    board.graphic_lines.push(line);
                },
                _ => {},
            }
        }

        Ok(board)
    }
}

#[derive(Debug, Clone, Default)]
pub struct GraphicLine {
    start: (f64, f64),
    end: (f64, f64),
    layer: String,
}

impl GraphicLine {
    fn from_list(list: &[Sexp]) -> Result<Self> {
        let mut line = Self::default();

        for field in list {
            let Sexp::List(list) = field else {
                continue;
            };

            let Sexp::Atom(Atom::S(head)) = &list[0] else {
                continue;
            };

            let rest = &list[1..];

            match head.as_str() {
                "start" => {
                    let coords = extract_coords(&rest[0], &rest[1])?;
                    line.start = coords;
                },
                "end" => {
                    let coords = extract_coords(&rest[0], &rest[1])?;
                    line.end = coords;
                },
                "layer" => {
                    if let Sexp::Atom(Atom::S(layer)) = &rest[0] {
                        line.layer = layer.to_string();
                    }
                },
                _ => {},
            }
        }

        Ok(line)
    }
}

fn extract_coords(x: &Sexp, y: &Sexp) -> Result<(f64, f64)> {
    Ok((extract_number(x)?, extract_number(y)?))
}

fn extract_number(num: &Sexp) -> Result<f64> {
    match num {
        Sexp::Atom(Atom::F(float)) => Ok(*float),
        Sexp::Atom(Atom::I(int)) => Ok(*int as f64),
        _ => Err(anyhow!("Expected a number to be a float or integer")),
    }
}
