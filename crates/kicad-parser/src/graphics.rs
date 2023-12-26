use crate::{extract_coords, Error};
use sexp::{Atom, Sexp};

use crate::board::BoardLayer;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GraphicLine {
    start: (f64, f64),
    end: (f64, f64),
    layer: String,
}

impl GraphicLine {
    pub fn from_list(list: &[Sexp]) -> Result<Self, Error> {
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

    pub fn start_point(&self) -> (f64, f64) {
        self.start
    }

    pub fn end_point(&self) -> (f64, f64) {
        self.end
    }

    pub fn layer(&self) -> BoardLayer {
        BoardLayer::from(self.layer.as_str())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GraphicArc {
    start: (f64, f64),
    mid: (f64, f64),
    end: (f64, f64),
    layer: String,
}

impl GraphicArc {
    pub fn from_list(list: &[Sexp]) -> Result<Self, Error> {
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

    pub fn start_point(&self) -> (f64, f64) {
        self.start
    }

    pub fn mid_point(&self) -> (f64, f64) {
        self.mid
    }

    pub fn end_point(&self) -> (f64, f64) {
        self.end
    }

    pub fn layer(&self) -> BoardLayer {
        BoardLayer::from(self.layer.as_str())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GraphicCircle {
    center: (f64, f64),
    end: (f64, f64),
    layer: String,
}

impl GraphicCircle {
    pub fn from_list(list: &[Sexp]) -> Result<Self, Error> {
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

    pub fn center_point(&self) -> (f64, f64) {
        self.center
    }

    pub fn end_point(&self) -> (f64, f64) {
        self.end
    }

    pub fn layer(&self) -> BoardLayer {
        BoardLayer::from(self.layer.as_str())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GraphicRect {
    start: (f64, f64),
    end: (f64, f64),
    layer: String,
}

impl GraphicRect {
    pub fn from_list(list: &[Sexp]) -> Result<Self, Error> {
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

    pub fn layer(&self) -> BoardLayer {
        BoardLayer::from(self.layer.as_str())
    }
}
