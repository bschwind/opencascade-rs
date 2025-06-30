use crate::{
    angle::ToAngle,
    primitives::{Edge, EdgeConnection, Face, Wire},
    workplane::Workplane,
    Error,
};
use glam::DVec2;
use kicad_parser::{
    board::{BoardLayer, KicadBoard},
    graphics::{GraphicArc, GraphicCircle, GraphicLine, GraphicRect},
};
use std::path::Path;

impl From<&GraphicLine> for Edge {
    fn from(line: &GraphicLine) -> Edge {
        let start = DVec2::from(line.start_point);
        let end = DVec2::from(line.end_point);
        Edge::segment(start.extend(0.0), end.extend(0.0))
    }
}

impl From<&GraphicArc> for Edge {
    fn from(arc: &GraphicArc) -> Edge {
        let start = DVec2::from(arc.start_point);
        let mid = DVec2::from(arc.mid_point);
        let end = DVec2::from(arc.end_point);
        Edge::arc(start.extend(0.0), mid.extend(0.0), end.extend(0.0))
    }
}

impl From<&GraphicCircle> for Face {
    fn from(circle: &GraphicCircle) -> Face {
        let center = DVec2::from(circle.center_point);
        let end = DVec2::from(circle.end_point);

        let delta = (center - end).abs();

        let radius = (delta.x * delta.x + delta.y * delta.y).sqrt();
        Workplane::xy().translated(center.extend(0.0)).circle(center.x, center.y, radius).to_face()
    }
}

impl From<&GraphicRect> for Face {
    fn from(rect: &GraphicRect) -> Face {
        let start = DVec2::from(rect.start_point);
        let end = DVec2::from(rect.end_point);

        let dimensions = (end - start).abs();
        Workplane::xy().translated(start.extend(0.0)).rect(dimensions.x, dimensions.y).to_face()
    }
}

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

    pub fn layer_edges<'a>(&'a self, layer: &'a BoardLayer) -> impl Iterator<Item = Edge> + 'a {
        let footprint_edges = self.board.footprints().flat_map(|footprint| {
            let angle = footprint.rotation_degrees.degrees();
            // TODO(bschwind) - Document why a negative angle is needed here.
            let angle_vec = DVec2::from_angle(-angle.radians());
            let translate = DVec2::from(footprint.location);

            footprint
                .graphic_lines
                .iter()
                .filter(|line| line.layer == *layer)
                .map(move |line| {
                    let start = line.start_point;
                    let end = line.end_point;
                    let start = DVec2::from(start);
                    let end = DVec2::from(end);

                    let start = translate + angle_vec.rotate(start);
                    let end = translate + angle_vec.rotate(end);

                    Edge::segment(start.extend(0.0), end.extend(0.0))
                })
                .chain(footprint.graphic_arcs.iter().filter(|arc| arc.layer == *layer).map(
                    move |arc| {
                        let start = arc.start_point;
                        let mid = arc.mid_point;
                        let end = arc.end_point;
                        let start = DVec2::from(start);
                        let mid = DVec2::from(mid);
                        let end = DVec2::from(end);

                        let start = translate + angle_vec.rotate(start);
                        let mid = translate + angle_vec.rotate(mid);
                        let end = translate + angle_vec.rotate(end);

                        Edge::arc(start.extend(0.0), mid.extend(0.0), end.extend(0.0))
                    },
                ))
        });

        self.board
            .lines()
            .filter(|line| line.layer == *layer)
            .map(Edge::from)
            .chain(self.board.arcs().filter(|arc| arc.layer == *layer).map(Edge::from))
            .chain(footprint_edges)
    }
}
