use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use kicad_files::board::{Footprint, Layer};
use kicad_files::board::graphic::{Arc as GrArc, Circle as GrCircle, Line as GrLine, Text as GrText};
use kicad_files::common::{Paper, TitleBlock, Point};
use kicad_files::{mm};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename = "generator")]
struct Generator {
    name: String,
    #[serde(with = "serde_kicad_sexpr::Option")]
    version: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename = "general")]
struct General {
    thickness: mm,
    #[serde(with = "serde_kicad_sexpr::Option")]
    drawings: Option<u32>,
    #[serde(with = "serde_kicad_sexpr::Option")]
    tracks: Option<u32>,
    #[serde(with = "serde_kicad_sexpr::Option")]
    zones: Option<u32>,
    #[serde(with = "serde_kicad_sexpr::Option")]
    modules: Option<u32>,
    #[serde(with = "serde_kicad_sexpr::Option")]
    nets: Option<u32>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename = "net")]
struct Net {
    num: u32,
    name: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename = "net_class")]
struct NetClass {
    name: String,
    label: String,
    clearance: f32,
    trace_width: f32,
    diff_pair_width: f32,
    diff_pair_gap: f32,
    via_dia: f32,
    via_drill: f32,
    uvia_dia: f32,
    uvia_drill: f32,
    nets: Vec<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct Segment {
    start: (f32, f32),
    end: (f32, f32),
    layer: String,
    width: f32,
    net: u32,
    tstamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Dimension {
    num: u32,
    width: f32,
    layer: String,
    text: GrText,
    feature1: Vec<Point>,
    feature2: Vec<Point>,
    crossbar: Vec<Point>,
    arrow1a: Vec<Point>,
    arrow1b: Vec<Point>,
    arrow2a: Vec<Point>,
    arrow2b: Vec<Point>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct Via {
    at: (f32, f32),
    size: f32,
    drill: f32,
    layers: Vec<String>,
    net: u32,
    tstamp: String,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// struct Zone {
//     net: u32,
//     net_name: String,
//     layers: Vec<String>,
//     tstamp: String,
//     priority: u8,
//     hatch: (String, f32),
//     connect_pads: (String, f32), // TODO: fix this
//     min_thickness: f32,
//     fill: ZoneFill,
//     polygons: Vec<Polygon>,
//     keepout: Keepout,
// }

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename = "setup")]
struct Setup {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "kicad_pcb")]
pub struct KicadPCB {
    version: u64,
    generator: Generator,
    general: General,
    paper: Paper,
    title_block: TitleBlock,
    page: String,
    layers: Vec<Layer>,
    setup: Setup,
    nets: Vec<Net>,
    net_classes: Vec<NetClass>,
    modules: Vec<Footprint>, // deserializes footprint or module
    dimensions: Vec<Dimension>,
    gr_circles: Vec<GrCircle>,
    gr_texts: Vec<GrText>,
    gr_arcs: Vec<GrArc>,
    gr_lines: Vec<GrLine>,
    segments: Vec<Segment>,
    vias: Vec<Via>,
    // zones: Vec<Zone>,
}

pub fn read_kicad_pcb(filepath: &PathBuf) -> KicadPCB {
    let contents =
        fs::read_to_string(filepath).expect("Something went wrong reading the file");

   serde_kicad_sexpr::from_str::<KicadPCB>(&contents).unwrap()
}
