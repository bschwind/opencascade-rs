use anyhow::{anyhow, Result};
use glam::{dvec3, DVec3};
use opencascade::{
    primitives::{Edge, Face},
    workplane::Workplane,
};
use sexp::{Atom, Sexp};

use crate::board::BoardLayer;

#[derive(Debug, Clone, Default)]
pub struct GraphicLine {
    start: (f64, f64),
    end: (f64, f64),
    layer: String,
}

impl GraphicLine {
    pub fn from_list(list: &[Sexp]) -> Result<Self> {
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

    pub fn start_point(&self) -> DVec3 {
        dvec3(self.start.0, self.start.1, 0.0)
    }

    pub fn end_point(&self) -> DVec3 {
        dvec3(self.end.0, self.end.1, 0.0)
    }

    pub fn layer(&self) -> BoardLayer {
        BoardLayer::from(self.layer.as_str())
    }
}

impl Into<Edge> for &GraphicLine {
    fn into(self) -> Edge {
        Edge::segment(self.start_point(), self.end_point())
    }
}

#[derive(Debug, Clone, Default)]
pub struct GraphicArc {
    start: (f64, f64),
    mid: (f64, f64),
    end: (f64, f64),
    layer: String,
}

impl GraphicArc {
    pub fn from_list(list: &[Sexp]) -> Result<Self> {
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
                "mid" => {
                    let coords = extract_coords(&rest[0], &rest[1])?;
                    line.mid = coords;
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

    pub fn start_point(&self) -> DVec3 {
        dvec3(self.start.0, self.start.1, 0.0)
    }

    pub fn mid_point(&self) -> DVec3 {
        dvec3(self.mid.0, self.mid.1, 0.0)
    }

    pub fn end_point(&self) -> DVec3 {
        dvec3(self.end.0, self.end.1, 0.0)
    }

    pub fn layer(&self) -> BoardLayer {
        BoardLayer::from(self.layer.as_str())
    }
}

impl Into<Edge> for &GraphicArc {
    fn into(self) -> Edge {
        Edge::arc(self.start_point(), self.mid_point(), self.end_point())
    }
}

#[derive(Debug, Clone, Default)]
pub struct GraphicCircle {
    center: (f64, f64),
    end: (f64, f64),
    layer: String,
}

impl GraphicCircle {
    pub fn from_list(list: &[Sexp]) -> Result<Self> {
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
                "center" => {
                    let coords = extract_coords(&rest[0], &rest[1])?;
                    line.center = coords;
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

    pub fn center_point(&self) -> DVec3 {
        dvec3(self.center.0, self.center.1, 0.0)
    }

    pub fn end_point(&self) -> DVec3 {
        dvec3(self.end.0, self.end.1, 0.0)
    }

    pub fn layer(&self) -> BoardLayer {
        BoardLayer::from(self.layer.as_str())
    }
}

impl Into<Face> for &GraphicCircle {
    fn into(self) -> Face {
        let delta_x = (self.center.0 - self.end.0).abs();
        let delta_y = (self.center.1 - self.end.1).abs();
        let radius = (delta_x * delta_x + delta_y * delta_y).sqrt();
        Workplane::xy()
            .translated(self.center_point())
            .circle(self.center.0, self.center.1, radius)
            .to_face()
    }
}

#[derive(Debug, Clone, Default)]
pub struct GraphicRect {
    start: (f64, f64),
    end: (f64, f64),
    layer: String,
}

impl GraphicRect {
    pub fn from_list(list: &[Sexp]) -> Result<Self> {
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

    pub fn start_point(&self) -> DVec3 {
        dvec3(self.start.0, self.start.1, 0.0)
    }

    pub fn end_point(&self) -> DVec3 {
        dvec3(self.end.0, self.end.1, 0.0)
    }

    pub fn layer(&self) -> BoardLayer {
        BoardLayer::from(self.layer.as_str())
    }
}

impl Into<Face> for &GraphicRect {
    fn into(self) -> Face {
        let height = (self.end.1 - self.start.1).abs();
        let width = (self.end.0 - self.start.0).abs();
        Workplane::xy().translated(self.start_point()).rect(height, width).to_face()
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
