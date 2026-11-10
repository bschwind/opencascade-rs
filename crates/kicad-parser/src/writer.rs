use sexp::{atom_f, atom_s, Sexp};

use crate::{
    board::{BoardLayer, Footprint, KicadBoard},
    graphics::{GraphicArc, GraphicCircle, GraphicLine, GraphicRect},
};

pub fn write_board<W: std::io::Write>(
    writer: &mut W,
    board: &KicadBoard,
) -> Result<(), std::io::Error> {
    let sexp = board_to_sexp(board);
    write!(writer, "{}", sexp)
}

fn board_to_sexp(board: &KicadBoard) -> Sexp {
    let mut items = vec![atom_s("kicad_pcb")];

    for line in &board.graphic_lines {
        items.push(line_to_sexp(line));
    }

    for arc in &board.graphic_arcs {
        items.push(arc_to_sexp(arc));
    }

    for circle in &board.graphic_circles {
        items.push(circle_to_sexp(circle));
    }

    for rect in &board.graphic_rects {
        items.push(rect_to_sexp(rect));
    }

    for footprint in &board.footprints {
        items.push(footprint_to_sexp(footprint));
    }

    Sexp::List(items)
}

fn line_to_sexp(line: &GraphicLine) -> Sexp {
    let items = vec![
        atom_s("gr_line"),
        cons_s("start", point_to_sexp(line.start_point)),
        cons_s("end", point_to_sexp(line.end_point)),
        cons_s("layer", layer_to_sexp(&line.layer)),
    ];
    Sexp::List(items)
}

fn arc_to_sexp(arc: &GraphicArc) -> Sexp {
    let items = vec![
        atom_s("gr_arc"),
        cons_s("start", point_to_sexp(arc.start_point)),
        cons_s("mid", point_to_sexp(arc.mid_point)),
        cons_s("end", point_to_sexp(arc.end_point)),
        cons_s("layer", layer_to_sexp(&arc.layer)),
    ];
    Sexp::List(items)
}

fn circle_to_sexp(circle: &GraphicCircle) -> Sexp {
    let items = vec![
        atom_s("gr_circle"),
        cons_s("start", point_to_sexp(circle.center_point)),
        cons_s("end", point_to_sexp(circle.end_point)),
        cons_s("layer", layer_to_sexp(&circle.layer)),
    ];
    Sexp::List(items)
}

fn rect_to_sexp(rect: &GraphicRect) -> Sexp {
    let items = vec![
        atom_s("gr_rect"),
        cons_s("start", point_to_sexp(rect.start_point)),
        cons_s("end", point_to_sexp(rect.end_point)),
        cons_s("layer", layer_to_sexp(&rect.layer)),
    ];
    Sexp::List(items)
}

fn footprint_to_sexp(footprint: &Footprint) -> Sexp {
    let mut items = vec![
        atom_s("footprint"),
        //cons_s("layer", layer_to_sexp(&footprint.layer)),
        cons_s("at", position_to_sexp(footprint.location, footprint.rotation_degrees)),
    ];

    for line in &footprint.graphic_lines {
        items.push(fp_line_to_sexp(line));
    }

    for arc in &footprint.graphic_arcs {
        items.push(fp_arc_to_sexp(arc));
    }

    Sexp::List(items)
}

// NOTE: The assumption that FootprintLine and GraphicLine are equivalent may not be correct
fn fp_line_to_sexp(line: &GraphicLine) -> Sexp {
    let items = vec![
        atom_s("fp_line"),
        cons_s("start", point_to_sexp(line.start_point)),
        cons_s("end", point_to_sexp(line.end_point)),
        cons_s("layer", layer_to_sexp(&line.layer)),
    ];
    Sexp::List(items)
}

// NOTE: The assumption that FootprintArc and GraphicArc are equivalent may not be correct
fn fp_arc_to_sexp(arc: &GraphicArc) -> Sexp {
    let items = vec![
        atom_s("fp_arc"),
        cons_s("start", point_to_sexp(arc.start_point)),
        cons_s("mid", point_to_sexp(arc.mid_point)),
        cons_s("end", point_to_sexp(arc.end_point)),
        cons_s("layer", layer_to_sexp(&arc.layer)),
    ];
    Sexp::List(items)
}

fn point_to_sexp(point: (f64, f64)) -> Sexp {
    Sexp::List(vec![atom_f(point.0), atom_f(point.1)])
}

fn layer_to_sexp(layer: &BoardLayer) -> Sexp {
    let layer_str: &str = layer.into();
    atom_s(layer_str)
}

fn position_to_sexp(location: (f64, f64), rotation_degrees: f64) -> Sexp {
    Sexp::List(vec![atom_f(location.0), atom_f(location.1), atom_f(rotation_degrees)])
}

fn cons_s(head: &str, tail: Sexp) -> Sexp {
    Sexp::List(match tail {
        Sexp::List(sexps) => {
            let mut items = vec![atom_s(head)];
            items.extend(sexps);
            items
        },
        atom => vec![atom_s(head), atom],
    })
}
