use crate::{
    angle::ToAngle,
    primitives::{Edge, EdgeConnection, Wire},
    Error,
};
use glam::{dvec3, DVec2};
use kicad_parser::{
    board::{BoardLayer, KicadBoard},
    graphics::{GraphicArc, GraphicLine},
};
use std::path::Path;

impl From<&GraphicLine> for Edge {
    fn from(line: &GraphicLine) -> Edge {
        let start = line.start_point();
        let end = line.end_point();
        Edge::segment(dvec3(start.0, start.1, 0.0), dvec3(end.0, end.1, 0.0))
    }
}

impl From<&GraphicArc> for Edge {
    fn from(arc: &GraphicArc) -> Edge {
        let start = arc.start_point();
        let mid = arc.mid_point();
        let end = arc.end_point();
        Edge::arc(dvec3(start.0, start.1, 0.0), dvec3(mid.0, mid.1, 0.0), dvec3(end.0, end.1, 0.0))
    }
}

// impl From<&GraphicCircle> for Face {
//     fn from(circle: &GraphicCircle) -> Face {
//         let delta_x = (circle.center.0 - circle.end.0).abs();
//         let delta_y = (circle.center.1 - circle.end.1).abs();
//         let radius = (delta_x * delta_x + delta_y * delta_y).sqrt();
//         Workplane::xy()
//             .translated(circle.center_point())
//             .circle(circle.center.0, circle.center.1, radius)
//             .to_face()
//     }
// }

// impl From<&GraphicRect> for Face {
//     fn from(rect: &GraphicRect) -> Face {
//         let height = (rect.end.1 - rect.start.1).abs();
//         let width = (rect.end.0 - rect.start.0).abs();
//         Workplane::xy().translated(rect.start_point()).rect(height, width).to_face()
//     }
// }

pub struct KicadPcb {
    board: KicadBoard,
}

impl KicadPcb {
    pub fn from_file<P: AsRef<Path>>(file: P) -> Result<Self, Error> {
        Ok(Self { board: KicadBoard::from_file(file)? })
    }

    pub fn edge_cuts(&self) -> Wire {
        Wire::from_unordered_edges(
            self.layer_edges(&BoardLayer::EdgeCuts),
            EdgeConnection::default(),
        )
    }

    pub fn layer_edges<'a>(&'a self, layer: &'a BoardLayer) -> impl Iterator<Item = Edge> + '_ {
        let footprint_edges = self.board.footprints().flat_map(|footprint| {
            let angle = footprint.rotation_degrees.degrees();
            let angle_vec = DVec2::from_angle(-angle.radians());
            let translate = DVec2::from(footprint.location);

            footprint
                .lines()
                .filter(|line| line.layer() == *layer)
                .map(move |line| {
                    let start = line.start_point();
                    let end = line.end_point();
                    let start = DVec2::from(start);
                    let end = DVec2::from(end);

                    let start = translate + angle_vec.rotate(start);
                    let end = translate + angle_vec.rotate(end);

                    Edge::segment(start.extend(0.0), end.extend(0.0))
                })
                .chain(footprint.arcs().filter(|arc| arc.layer() == *layer).map(move |arc| {
                    let start = arc.start_point();
                    let mid = arc.mid_point();
                    let end = arc.end_point();
                    let start = DVec2::from(start);
                    let mid = DVec2::from(mid);
                    let end = DVec2::from(end);

                    let start = translate + angle_vec.rotate(start);
                    let mid = translate + angle_vec.rotate(mid);
                    let end = translate + angle_vec.rotate(end);

                    Edge::arc(start.extend(0.0), mid.extend(0.0), end.extend(0.0))
                }))
        });

        self.board
            .lines()
            .filter(|line| line.layer() == *layer)
            .map(Edge::from)
            .chain(self.board.arcs().filter(|arc| arc.layer() == *layer).map(Edge::from))
            .chain(footprint_edges)
    }

    // pub fn layer_wire(&self, layer: BoardLayer) -> Wire {
    //     Wire::from_unordered_edges(&self.layer_edges(layer), EdgeConnection::default())
    // }

    // pub fn layer_face(&self, layer: BoardLayer) -> Face {
    //     Face::from_wire(&self.layer_wire(layer))
    // }

    // pub fn outline(&self, _offset: f64) -> Face {
    //     // TODO apply offset around the face
    //     self.layer_face(BoardLayer::EdgeCuts)
    // }
}
