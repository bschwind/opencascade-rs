use crate::{extract_coords, Error};
use sexp::{Atom, Sexp};

use crate::board::BoardLayer;

#[derive(Debug, Clone, PartialEq)]
pub struct GraphicLine {
    pub start: (f64, f64),
    pub end: (f64, f64),
    pub layer: BoardLayer,
}

impl GraphicLine {
    pub fn from_list(list: &[Sexp]) -> Result<Self, Error> {
        let mut start: Option<(f64, f64)> = None;
        let mut end: Option<(f64, f64)> = None;
        let mut layer: Option<BoardLayer> = None;

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
                    start = Some(coords);
                },
                "end" => {
                    let coords = extract_coords(&rest[0], &rest[1])?;
                    end = Some(coords);
                },
                "layer" => {
                    if let Sexp::Atom(Atom::S(layer_str)) = &rest[0] {
                        let layer_valid = layer_str.as_str().into();
                        layer = Some(layer_valid);
                    }
                },
                _ => {},
            }
        }

        if let (Some(start), Some(end), Some(layer)) = (start, end, layer) {
            Ok(Self { start, end, layer })
        } else {
            Err(Error::IncompleteGraphicLine(list.to_vec()))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GraphicArc {
    pub start: (f64, f64),
    pub mid: (f64, f64),
    pub end: (f64, f64),
    pub layer: BoardLayer,
}

impl GraphicArc {
    pub fn from_list(list: &[Sexp]) -> Result<Self, Error> {
        let mut start: Option<(f64, f64)> = None;
        let mut mid: Option<(f64, f64)> = None;
        let mut end: Option<(f64, f64)> = None;
        let mut layer: Option<BoardLayer> = None;

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
                    start = Some(coords);
                },
                "mid" => {
                    let coords = extract_coords(&rest[0], &rest[1])?;
                    mid = Some(coords);
                },
                "end" => {
                    let coords = extract_coords(&rest[0], &rest[1])?;
                    end = Some(coords);
                },
                "layer" => {
                    if let Sexp::Atom(Atom::S(layer_str)) = &rest[0] {
                        layer = Some(layer_str.as_str().into());
                    }
                },
                _ => {},
            }
        }

        if let (Some(start), Some(mid), Some(end), Some(layer)) = (start, mid, end, layer) {
            Ok(Self { start, mid, end, layer })
        } else {
            Err(Error::IncompleteGraphicArc(list.to_vec()))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GraphicCircle {
    pub center: (f64, f64),
    pub end: (f64, f64),
    pub layer: BoardLayer,
}

impl GraphicCircle {
    pub fn from_list(list: &[Sexp]) -> Result<Self, Error> {
        let mut center: Option<(f64, f64)> = None;
        let mut end: Option<(f64, f64)> = None;
        let mut layer: Option<BoardLayer> = None;

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
                    center = Some(coords);
                },
                "end" => {
                    let coords = extract_coords(&rest[0], &rest[1])?;
                    end = Some(coords);
                },
                "layer" => {
                    if let Sexp::Atom(Atom::S(layer_str)) = &rest[0] {
                        layer = Some(layer_str.as_str().into());
                    }
                },
                _ => {},
            }
        }

        if let (Some(center), Some(end), Some(layer)) = (center, end, layer) {
            Ok(Self { center, end, layer })
        } else {
            Err(Error::IncompleteGraphicCircle(list.to_vec()))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GraphicRect {
    pub start: (f64, f64),
    pub end: (f64, f64),
    pub layer: BoardLayer,
}

impl GraphicRect {
    pub fn from_list(list: &[Sexp]) -> Result<Self, Error> {
        let mut start: Option<(f64, f64)> = None;
        let mut end: Option<(f64, f64)> = None;
        let mut layer: Option<BoardLayer> = None;

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
                    start = Some(coords);
                },
                "end" => {
                    let coords = extract_coords(&rest[0], &rest[1])?;
                    end = Some(coords);
                },
                "layer" => {
                    if let Sexp::Atom(Atom::S(layer_str)) = &rest[0] {
                        layer = Some(layer_str.as_str().into());
                    }
                },
                _ => {},
            }
        }

        if let (Some(start), Some(end), Some(layer)) = (start, end, layer) {
            Ok(Self { start, end, layer })
        } else {
            Err(Error::IncompleteGraphicRect(list.to_vec()))
        }
    }
}
