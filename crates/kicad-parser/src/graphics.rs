use crate::{extract_coords, Error};
use sexp::{Atom, Sexp};

use crate::board::BoardLayer;

#[derive(Debug, Clone, PartialEq)]
pub struct GraphicLine {
    pub start_point: (f64, f64),
    pub end_point: (f64, f64),
    pub layer: BoardLayer,
}

impl GraphicLine {
    pub fn from_list(list: &[Sexp]) -> Result<Self, Error> {
        let mut start_point: Option<(f64, f64)> = None;
        let mut end_point: Option<(f64, f64)> = None;
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
                    start_point = Some(coords);
                },
                "end" => {
                    let coords = extract_coords(&rest[0], &rest[1])?;
                    end_point = Some(coords);
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

        if let (Some(start_point), Some(end_point), Some(layer)) = (start_point, end_point, layer) {
            Ok(Self { start_point, end_point, layer })
        } else {
            Err(Error::IncompleteGraphicLine(list.to_vec()))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GraphicArc {
    pub start_point: (f64, f64),
    pub mid_point: (f64, f64),
    pub end_point: (f64, f64),
    pub layer: BoardLayer,
}

impl GraphicArc {
    pub fn from_list(list: &[Sexp]) -> Result<Self, Error> {
        let mut start_point: Option<(f64, f64)> = None;
        let mut mid_point: Option<(f64, f64)> = None;
        let mut end_point: Option<(f64, f64)> = None;
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
                    start_point = Some(coords);
                },
                "mid" => {
                    let coords = extract_coords(&rest[0], &rest[1])?;
                    mid_point = Some(coords);
                },
                "end" => {
                    let coords = extract_coords(&rest[0], &rest[1])?;
                    end_point = Some(coords);
                },
                "layer" => {
                    if let Sexp::Atom(Atom::S(layer_str)) = &rest[0] {
                        layer = Some(layer_str.as_str().into());
                    }
                },
                _ => {},
            }
        }

        if let (Some(start_point), Some(mid_point), Some(end_point), Some(layer)) =
            (start_point, mid_point, end_point, layer)
        {
            Ok(Self { start_point, mid_point, end_point, layer })
        } else {
            Err(Error::IncompleteGraphicArc(list.to_vec()))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GraphicCircle {
    pub center_point: (f64, f64),
    pub end_point: (f64, f64),
    pub layer: BoardLayer,
}

impl GraphicCircle {
    pub fn from_list(list: &[Sexp]) -> Result<Self, Error> {
        let mut center_point: Option<(f64, f64)> = None;
        let mut end_point: Option<(f64, f64)> = None;
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
                    center_point = Some(coords);
                },
                "end" => {
                    let coords = extract_coords(&rest[0], &rest[1])?;
                    end_point = Some(coords);
                },
                "layer" => {
                    if let Sexp::Atom(Atom::S(layer_str)) = &rest[0] {
                        layer = Some(layer_str.as_str().into());
                    }
                },
                _ => {},
            }
        }

        if let (Some(center_point), Some(end_point), Some(layer)) = (center_point, end_point, layer)
        {
            Ok(Self { center_point, end_point, layer })
        } else {
            Err(Error::IncompleteGraphicCircle(list.to_vec()))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GraphicRect {
    pub start_point: (f64, f64),
    pub end_point: (f64, f64),
    pub layer: BoardLayer,
}

impl GraphicRect {
    pub fn from_list(list: &[Sexp]) -> Result<Self, Error> {
        let mut start_point: Option<(f64, f64)> = None;
        let mut end_point: Option<(f64, f64)> = None;
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
                    start_point = Some(coords);
                },
                "end" => {
                    let coords = extract_coords(&rest[0], &rest[1])?;
                    end_point = Some(coords);
                },
                "layer" => {
                    if let Sexp::Atom(Atom::S(layer_str)) = &rest[0] {
                        layer = Some(layer_str.as_str().into());
                    }
                },
                _ => {},
            }
        }

        if let (Some(start_point), Some(end_point), Some(layer)) = (start_point, end_point, layer) {
            Ok(Self { start_point, end_point, layer })
        } else {
            Err(Error::IncompleteGraphicRect(list.to_vec()))
        }
    }
}
