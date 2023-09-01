use anyhow::{anyhow, Context, Result};
use opencascade::primitives::{Edge, Face, Wire};
use sexp::{Atom, Sexp};
use std::path::Path;

use crate::graphics::{GraphicArc, GraphicCircle, GraphicLine, GraphicRect};

#[derive(Debug, Clone, PartialEq)]
pub enum BoardLayer {
    FCu,
    BCu,
    FAdhes,
    BAdhes,
    FPaste,
    BPaste,
    FSilkS,
    BSilkS,
    FMask,
    BFask,
    DwgsUser,
    CmtsUser,
    Eco1User,
    Eco2User,
    EdgeCuts,
    Margin,
    BCrtYd,
    FCrtYd,
    BFab,
    FFab,
    In1Cu,
    In2Cu,
    In3Cu,
    In4Cu,
    User(String),
}

impl From<&str> for BoardLayer {
    fn from(s: &str) -> Self {
        match s {
            "F.Cu" => BoardLayer::FCu,
            "B.Cu" => BoardLayer::BCu,
            "F.Adhes" => BoardLayer::FAdhes,
            "B.Adhes" => BoardLayer::BAdhes,
            "F.Paste" => BoardLayer::FPaste,
            "B.Paste" => BoardLayer::BPaste,
            "F.SilkS" => BoardLayer::FSilkS,
            "B.SilkS" => BoardLayer::BSilkS,
            "F.Mask" => BoardLayer::FMask,
            "B.Mask" => BoardLayer::BFask,
            "Dwgs.User" => BoardLayer::DwgsUser,
            "Cmts.User" => BoardLayer::CmtsUser,
            "Eco1.User" => BoardLayer::Eco1User,
            "Eco2.User" => BoardLayer::Eco2User,
            "Edge.Cuts" => BoardLayer::EdgeCuts,
            "Margin" => BoardLayer::Margin,
            "B.CrtYd" => BoardLayer::BCrtYd,
            "F.CrtYd" => BoardLayer::FCrtYd,
            "B.Fab" => BoardLayer::BFab,
            "F.Fab" => BoardLayer::FFab,
            "In1.Cu" => BoardLayer::In1Cu,
            "In2.Cu" => BoardLayer::In2Cu,
            "In3.Cu" => BoardLayer::In3Cu,
            "In4.Cu" => BoardLayer::In4Cu,
            _ => BoardLayer::User(s.to_string()),
        }
    }
}

impl std::str::FromStr for BoardLayer {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl<'a> From<&'a BoardLayer> for &'a str {
    fn from(layer: &'a BoardLayer) -> Self {
        match *layer {
            BoardLayer::FCu => "F.Cu",
            BoardLayer::BCu => "B.Cu",
            BoardLayer::FAdhes => "F.Adhes",
            BoardLayer::BAdhes => "B.Adhes",
            BoardLayer::FPaste => "F.Paste",
            BoardLayer::BPaste => "B.Paste",
            BoardLayer::FSilkS => "F.SilkS",
            BoardLayer::BSilkS => "B.SilkS",
            BoardLayer::FMask => "F.Mask",
            BoardLayer::BFask => "B.Mask",
            BoardLayer::DwgsUser => "Dwgs.User",
            BoardLayer::CmtsUser => "Cmts.User",
            BoardLayer::Eco1User => "Eco1.User",
            BoardLayer::Eco2User => "Eco2.User",
            BoardLayer::EdgeCuts => "Edge.Cuts",
            BoardLayer::Margin => "Margin",
            BoardLayer::BCrtYd => "B.CrtYd",
            BoardLayer::FCrtYd => "F.CrtYd",
            BoardLayer::BFab => "B.Fab",
            BoardLayer::FFab => "F.Fab",
            BoardLayer::In1Cu => "In1.Cu",
            BoardLayer::In2Cu => "In2.Cu",
            BoardLayer::In3Cu => "In3.Cu",
            BoardLayer::In4Cu => "In4.Cu",
            BoardLayer::User(ref s) => s,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct KicadBoard {
    graphic_lines: Vec<GraphicLine>,
    graphic_arcs: Vec<GraphicArc>,
    graphic_circles: Vec<GraphicCircle>,
    graphic_rects: Vec<GraphicRect>,
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
                "gr_arc" => {
                    let arc = GraphicArc::from_list(rest)?;
                    board.graphic_arcs.push(arc);
                },
                "gr_line" => {
                    let line = GraphicLine::from_list(rest)?;
                    board.graphic_lines.push(line);
                },
                "gr_circle" => {
                    let line = GraphicCircle::from_list(rest)?;
                    board.graphic_circles.push(line);
                },
                "gr_rect" => {
                    let line = GraphicRect::from_list(rest)?;
                    board.graphic_rects.push(line);
                },
                _ => {},
            }
        }

        Ok(board)
    }

    pub fn layer_edges(&self, layer: BoardLayer) -> Vec<Edge> {
        self.graphic_lines
            .iter()
            .filter(|line| line.layer() == layer)
            .map(Into::<Edge>::into)
            .chain(
                self.graphic_arcs.iter().filter(|arc| arc.layer() == layer).map(Into::<Edge>::into),
            )
            .collect()
    }

    pub fn layer_wire(&self, layer: BoardLayer) -> Wire {
        Wire::from_edges(&self.layer_edges(layer))
    }

    pub fn layer_face(&self, layer: BoardLayer) -> Face {
        Face::from_wire(&self.layer_wire(layer))
    }

    pub fn outline(&self, _offset: f64) -> Face {
        // TODO apply offset around the face
        self.layer_face(BoardLayer::EdgeCuts)
    }
}
