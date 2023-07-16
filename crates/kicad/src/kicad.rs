use std::fs;
use std::path::PathBuf;
use lexpr::{self, parse::Options};

#[derive(Debug, Default, Clone)]
struct Generator {
    name: String,
    version: Option<String>,
}

#[derive(Debug, Default, Clone)]
struct General {
    thickness: f32,
    drawings: u32,
    tracks: u32,
    zones: u32,
    modules: u32,
    nets: u32,
}

type Point = (f32, f32);

#[derive(Debug, Default, Clone)]
struct Layer {
    num: u32,
    name: String,
    typ: String,
    param4: String, // this changed in KiCad 6, but there's no documentation yet
}

#[derive(Debug, Default, Clone)]
struct PCBPlotParams {}

#[derive(Debug, Default, Clone)]
struct Setup {}

#[derive(Debug, Default, Clone)]
// Net declarations
struct Net {
    num: u32,
    name: String,
}

#[derive(Debug, Default, Clone)]
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

#[derive(Debug, Default, Clone)]
struct Font {
    size: (f32, f32),
    thickness: f32,
}

#[derive(Debug, Default, Clone)]
struct Effects {
    font: Font,
    justify: String,
}

#[derive(Debug, Default, Clone)]
struct FootprintText {
    typ: String,
    value: String,
    at: Vec<f32>,
    layer: String,
    hide: bool,
    effects: Effects,
}

#[derive(Debug, Default, Clone)]
struct FootprintLine {
    reference: String,
    start: (f32, f32),
    end: (f32, f32),
    layer: String,
    hide: bool,
    width: f32,
}

#[derive(Debug, Default, Clone)]
struct FootprintArc {
    start: (f32, f32),
    end: (f32, f32),
    angle: f32,
    layer: String,
    width: f32,
}

#[derive(Debug, Default, Clone)]
struct FootprintCircle {
    center: (f32, f32),
    end: (f32, f32),
    layer: String,
    width: f32,
}

#[derive(Debug, Default, Clone)]
struct PadOptions {
    clearance: String,
    anchor: String,
}

#[derive(Debug, Default, Clone)]
struct PadPrimitives {
    gr_poly: GrPoly,
}

#[derive(Debug, Default, Clone)]
struct Pad {
    num: String,      // can be int or string (e.g. for BGAs)
    pad_type: String, // smd, thr, tht
    typ: String,      // roundrect, rect, circle
    at: Vec<f32>,
    size: Vec<f32>,
    layers: Vec<String>,
    roundrect_rratio: f32,
    net: Net,
    drill: f32,
    drill_oval: (f32, f32),
    zone_connect: u8,
    options: PadOptions,
    primitives: PadPrimitives,
}

#[derive(Debug, Default, Clone)]
struct Model {
    path: String,
    at: Vec<f32>, // TODO: other coordinate systems than xyz???
    scale: Vec<f32>,
    rotate: Vec<f32>,
}

#[derive(Debug, Default, Clone)]
struct Module {
    name: String,
    layer: String,
    tedit: String,
    tstamp: String,
    at: Vec<f32>,
    descr: String,
    tags: String,
    path: String,
    attr: String,
    fp_texts: Vec<FootprintText>,
    fp_lines: Vec<FootprintLine>,
    fp_arcs: Vec<FootprintArc>,
    fp_circles: Vec<FootprintCircle>,
    fp_polys: Vec<FootprintPoly>,
    pads: Vec<Pad>,
    models: Vec<Model>,
}

#[derive(Debug, Default, Clone)]
struct GrText {
    label: String,
    at: Vec<f32>,
    layer: String,
    effects: Effects,
    tstamp: String,
}

#[derive(Debug, Default, Clone)]
struct GrCircle {
    center: (f32, f32),
    end: (f32, f32),
    layer: String,
    width: f32,
    tstamp: String,
}

#[derive(Debug, Default, Clone)]
struct GrArc {
    start: (f32, f32),
    end: (f32, f32),
    angle: f32,
    layer: String,
    width: f32,
    tstamp: String,
}

#[derive(Debug, Default, Clone)]
struct GrLine {
    start: (f32, f32),
    end: (f32, f32),
    layer: String,
    width: f32,
    tstamp: String,
}

#[derive(Debug, Default, Clone)]
struct GrPoly {
    points: Vec<Point>,
    width: f32,
}

#[derive(Debug, Default, Clone)]
struct Segment {
    start: (f32, f32),
    end: (f32, f32),
    layer: String,
    width: f32,
    net: u32,
    tstamp: String,
}

#[derive(Debug, Default, Clone)]
struct Via {
    at: (f32, f32),
    size: f32,
    drill: f32,
    layers: Vec<String>,
    net: u32,
    tstamp: String,
}

#[derive(Debug, Default, Clone)]
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

#[derive(Debug, Default, Clone)]
struct Polygon {
    points: Vec<Point>,
    filled: bool,
}

#[derive(Debug, Default, Clone)]
struct FootprintPoly {
    points: Vec<Point>,
    layer: String,
    width: f32,
}

#[derive(Debug, Default, Clone)]
struct ZoneFill {
    do_fill: bool,
    arc_segments: u8,
    thermal_gap: f32,
    thermal_bridge_width: f32,
    smoothing: String,
    radius: f32,
}

#[derive(Debug, Default, Clone)]
struct Keepout {
    tracks: String,
    vias: String,
    copperpour: String,
}

#[derive(Debug, Default, Clone)]
struct Zone {
    net: u32,
    net_name: String,
    layers: Vec<String>,
    tstamp: String,
    priority: u8,
    hatch: (String, f32),
    connect_pads: (String, f32), // TODO: fix this
    min_thickness: f32,
    fill: ZoneFill,
    polygons: Vec<Polygon>,
    keepout: Keepout,
}

#[derive(Debug, Default, Clone)]
pub struct PCB {
    version: u64,
    generator: Generator,
    general: General,
    page: String,
    layers: Vec<Layer>,
    setup: Setup,
    nets: Vec<Net>,
    net_classes: Vec<NetClass>,
    modules: Vec<Module>,
    dimensions: Vec<Dimension>,
    gr_circles: Vec<GrCircle>,
    gr_texts: Vec<GrText>,
    gr_arcs: Vec<GrArc>,
    gr_lines: Vec<GrLine>,
    segments: Vec<Segment>,
    vias: Vec<Via>,
    zones: Vec<Zone>,
}

pub fn read_kicad_pcb(filepath: &PathBuf) -> PCB {
    println!("reading test pcb...");

    let contents =
        fs::read_to_string(filepath).expect("Something went wrong reading the file");

    let results = lexpr::from_str_custom(&contents, Options::kicad()).unwrap();

    // the pcb structure
    let pcb = results.as_pair().unwrap();
    let iter = pcb.1.list_iter().unwrap();

    let mut pcb = PCB::default();

    for value in iter {
        let v = value.to_vec().unwrap();
        let sym = v.first().unwrap();

        if !sym.is_cons() {
            let name = sym.to_string();
            match name.as_str() {
                "version" => pcb.version = v[1].as_u64().unwrap(),
                "general" => pcb.general = parse_general(v),
                "page" => pcb.page = v[1].as_symbol().unwrap().to_string(),
                "layers" => pcb.layers = parse_layers(v),
                "setup" => println!("setup {:#?}", v[1]),
                "net" => pcb.nets.push(parse_net(v)),
                "net_class" => pcb.net_classes.push(parse_netclass(v)),
                "module" => pcb.modules.push(parse_module(v)),
                "segment" => pcb.segments.push(parse_segment(v)),
                "via" => pcb.vias.push(parse_via(v)),
                "dimension" => pcb.dimensions.push(parse_dimension(v)),
                "gr_circle" => pcb.gr_circles.push(parse_gr_circle(v)),
                "gr_text" => pcb.gr_texts.push(parse_gr_text(v)),
                "gr_line" => pcb.gr_lines.push(parse_gr_line(v)),
                "gr_arc" => pcb.gr_arcs.push(parse_gr_arc(v)),
                "zone" => pcb.zones.push(parse_zone(v)),
                _ => println!("uwu, what is this? {}", name),
            }
        } else {
            println!("{:#?}", v.to_vec());
        }
    }

    pcb
}

fn parse_general(v: Vec<lexpr::Value>) -> General {
    let mut g = General::default();

    for value in v {
        // first value is a symbol
        if value.is_symbol() {
            continue;
        }

        let param = value.to_vec().unwrap();
        let name = param[0].to_string();

        match name.as_str() {
            "thickness" => g.thickness = param[1].as_f64().unwrap() as f32,
            "drawings" => g.drawings = param[1].as_u64().unwrap() as u32,
            "zones" => g.zones = param[1].as_u64().unwrap() as u32,
            "modules" => g.modules = param[1].as_u64().unwrap() as u32,
            "nets" => g.nets = param[1].as_u64().unwrap() as u32,
            _ => println!("uwu, what is this? {}", name),
        }
    }
    g
}

fn parse_layers(v: Vec<lexpr::Value>) -> Vec<Layer> {
    let mut layers = Vec::new();

    for value in v {
        // first value is a symbol
        if value.is_symbol() {
            continue;
        }

        let p = value.to_vec().unwrap();

        let mut l = Layer::default();

        l.num = p[0].as_u64().unwrap() as u32;
        l.name = sym_or_str(value.get(1));
        l.typ = sym_or_str(value.get(2));
        if p.len() == 4 {
            l.param4 = sym_or_str(value.get(3));
        }

        layers.push(l);
    }

    layers
}

fn parse_net(v: Vec<lexpr::Value>) -> Net {
    let mut net = Net::default();

    net.num = v[1].as_u64().unwrap() as u32;
    if v[2].is_symbol() {
        net.name = v[2].as_symbol().unwrap().to_string();
    } else if v[2].is_string() {
        net.name = v[2].as_str().unwrap().to_string();
    }

    net
}

fn sym_or_str(v: Option<&lexpr::Value>) -> String {
    match v {
        None => String::new(),
        Some(e) => {
            if e.is_symbol() {
                e.as_symbol().unwrap().to_string()
            } else if e.is_string() {
                e.as_str().unwrap().to_string()
            } else {
                String::new() // TODO: bugcheck
            }
        }
    }
}

fn parse_vecf(ev: Vec<lexpr::Value>) -> Vec<f32> {
    if ev.len() == 2 {
        return vec![ev[1].as_f64().unwrap() as f32];
    } else if ev.len() == 3 {
        return vec![
            ev[1].as_f64().unwrap() as f32,
            ev[2].as_f64().unwrap() as f32,
        ];
    }
    return vec![
        ev[1].as_f64().unwrap() as f32,
        ev[2].as_f64().unwrap() as f32,
        ev[3].as_f64().unwrap() as f32,
    ];
}

fn parse_netclass(v: Vec<lexpr::Value>) -> NetClass {
    let mut nc = NetClass::default();

    let mut it = v.iter();

    it.next(); // throw away the first element which is just net_class

    nc.name = it.next().unwrap().as_symbol().unwrap().to_string();
    nc.label = it.next().unwrap().as_str().unwrap().to_string();

    for value in it {
        let inner = value.to_vec().unwrap();
        let label = inner[0].to_string();

        match label.as_str() {
            "add_net" => nc.nets.push(sym_or_str(value.get(1))),
            "clearance" => nc.clearance = inner[1].as_f64().unwrap() as f32,
            "trace_width" => nc.trace_width = inner[1].as_f64().unwrap() as f32,
            "via_dia" => nc.via_dia = inner[1].as_f64().unwrap() as f32,
            "via_drill" => nc.via_drill = inner[1].as_f64().unwrap() as f32,
            "uvia_dia" => nc.uvia_dia = inner[1].as_f64().unwrap() as f32,
            "uvia_drill" => nc.uvia_drill = inner[1].as_f64().unwrap() as f32,
            "diff_pair_width" => nc.diff_pair_width = inner[1].as_f64().unwrap() as f32,
            "diff_pair_gap" => nc.diff_pair_gap = inner[1].as_f64().unwrap() as f32,
            _ => println!("unknown cons in net_class: {:#?}", value),
        }
    }

    nc
}

fn parse_segment(v: Vec<lexpr::Value>) -> Segment {
    let mut seg = Segment::default();

    for value in v.iter() {
        if value.is_symbol() {
            continue;
        }

        let mut ev = value.to_vec().unwrap();

        // TODO: fix tstamp handling in lexpr
        if ev[0].is_cons() {
            ev = ev[0].to_vec().unwrap()
        }

        let label = ev[0].to_string();

        match label.as_str() {
            "start" => {
                seg.start = (
                    ev[1].as_f64().unwrap() as f32,
                    ev[2].as_f64().unwrap() as f32,
                )
            }
            "end" => {
                seg.end = (
                    ev[1].as_f64().unwrap() as f32,
                    ev[2].as_f64().unwrap() as f32,
                )
            }
            "width" => seg.width = ev[1].as_f64().unwrap() as f32,
            "layer" => seg.layer = sym_or_str(ev.get(1)),
            "net" => seg.net = ev[1].as_u64().unwrap() as u32,
            "tstamp" => seg.tstamp = sym_or_str(value.get(1)),
            _ => println!("unknown cons in segment: {:#?}", value),
        }
    }

    seg
}

fn parse_via(v: Vec<lexpr::Value>) -> Via {
    let mut via = Via::default();

    for value in v.iter() {
        if value.is_symbol() {
            continue;
        }
        let mut ev = value.to_vec().unwrap();

        // TODO: fix tstamp handling in lexpr
        if ev[0].is_cons() {
            ev = ev[0].to_vec().expect("cons in via")
        }

        let label = ev[0].to_string();

        match label.as_str() {
            "at" => {
                via.at = (
                    ev[1].as_f64().expect("via x") as f32,
                    ev[2].as_f64().expect("via y") as f32,
                )
            }
            "size" => via.size = ev[1].as_f64().expect("via size") as f32,
            "drill" => via.drill = ev[1].as_f64().expect("via drill") as f32,
            "layers" => {
                for l in ev[1..].iter() {
                    via.layers.push(l.as_symbol().unwrap_or("").to_string());
                }
            }
            "net" => via.net = ev[1].as_u64().expect("via net") as u32,
            "tstamp" => via.tstamp = sym_or_str(value.get(1)),
            _ => println!("unknown cons in via: {:#?}", value),
        }
    }

    via
}

fn parse_pts(v: Vec<lexpr::Value>) -> Vec<Point> {
    let mut points = Vec::new();

    for value in v {
        // first value is a symbol
        if value.is_symbol() {
            continue;
        }

        let p = value.to_vec().unwrap();

        // TODO: check for other coordinate system types than xy

        let l = (p[1].as_f64().expect("x point") as f32, p[2].as_f64().expect("y point") as f32);

        points.push(l);
    }

    points
}

fn parse_gr_text(v: Vec<lexpr::Value>) -> GrText {
    let mut grt = GrText::default();

    for elem in v.iter() {
        if !elem.is_cons() {
            grt.label = sym_or_str(elem.get(0));
            continue;
        }

        let mut ev = elem.to_vec().unwrap();

        // TODO: fix tstamp handling in lexpr
        if ev[0].is_cons() {
            ev = ev[0].to_vec().unwrap()
        }

        let label = ev[0].to_string();

        match label.as_str() {
            "at" => grt.at = parse_vecf(ev),
            "layer" => grt.layer = sym_or_str(elem.get(1)),
            "effects" => grt.effects = parse_effects(ev),
            "tstamp" => grt.tstamp = sym_or_str(elem.get(1)),
            _ => println!("unknown cons in gr_text: {:#?}", elem),
        }
    }

    grt
}

fn parse_gr_circle(v: Vec<lexpr::Value>) -> GrCircle {
    let mut grc = GrCircle::default();

    for elem in v.iter() {
        if !elem.is_cons() {
            continue;
        }

        let mut ev = elem.to_vec().unwrap();

        // TODO: fix tstamp handling in lexpr
        if ev[0].is_cons() {
            ev = ev[0].to_vec().unwrap()
        }

        let label = ev[0].to_string();

        match label.as_str() {
            "center" => {
                grc.center = (
                    ev[1].as_f64().unwrap() as f32,
                    ev[2].as_f64().unwrap() as f32,
                )
            }
            "end" => {
                grc.end = (
                    ev[1].as_f64().unwrap() as f32,
                    ev[2].as_f64().unwrap() as f32,
                )
            }
            "layer" => grc.layer = sym_or_str(elem.get(1)),
            "width" => grc.width = ev[1].as_f64().unwrap() as f32,
            "tstamp" => grc.tstamp = sym_or_str(elem.get(1)),
            _ => println!("unknown cons in gr_circle: {} {:#?}", label, elem),
        }
    }

    grc
}

fn parse_gr_poly(v: Vec<lexpr::Value>) -> GrPoly {
    let mut grp = GrPoly::default();

    for elem in v.iter() {
        if !elem.is_cons() {
            continue;
        }

        let mut ev = elem.to_vec().unwrap();

        // TODO: fix tstamp handling in lexpr
        if ev[0].is_cons() {
            ev = ev[0].to_vec().unwrap()
        }

        let label = ev[0].to_string();

        match label.as_str() {
            "pts" => grp.points = parse_pts(ev),
            "width" => grp.width = ev[1].as_f64().unwrap() as f32,
            _ => println!("unknown cons in gr_poly: {} {:#?}", label, elem),
        }
    }

    grp
}

fn parse_gr_line(v: Vec<lexpr::Value>) -> GrLine {
    let mut grl = GrLine::default();

    for elem in v.iter() {
        if !elem.is_cons() {
            continue;
        }

        let mut ev = elem.to_vec().unwrap();

        // TODO: fix tstamp handling in lexpr
        if ev[0].is_cons() {
            ev = ev[0].to_vec().unwrap()
        }

        let label = ev[0].to_string();

        match label.as_str() {
            "start" => {
                grl.start = (
                    ev[1].as_f64().unwrap() as f32,
                    ev[2].as_f64().unwrap() as f32,
                )
            }
            "end" => {
                grl.end = (
                    ev[1].as_f64().unwrap() as f32,
                    ev[2].as_f64().unwrap() as f32,
                )
            }
            "layer" => grl.layer = sym_or_str(elem.get(1)),
            "width" => grl.width = ev[1].as_f64().unwrap() as f32,
            "tstamp" => grl.tstamp = sym_or_str(elem.get(1)),
            _ => println!("unknown cons in gr_line: {:#?}", elem),
        }
    }

    grl
}

fn parse_gr_arc(v: Vec<lexpr::Value>) -> GrArc {
    let mut gra = GrArc::default();

    for elem in v.iter() {
        if !elem.is_cons() {
            continue;
        }

        let mut ev = elem.to_vec().unwrap();

        // TODO: fix tstamp handling in lexpr
        if ev[0].is_cons() {
            ev = ev[0].to_vec().unwrap()
        }

        let label = ev[0].to_string();

        match label.as_str() {
            "start" => {
                gra.start = (
                    ev[1].as_f64().unwrap() as f32,
                    ev[2].as_f64().unwrap() as f32,
                )
            }
            "end" => {
                gra.end = (
                    ev[1].as_f64().unwrap() as f32,
                    ev[2].as_f64().unwrap() as f32,
                )
            }
            "angle" => gra.angle = ev[1].as_f64().unwrap() as f32,
            "layer" => gra.layer = sym_or_str(elem.get(1)),
            "width" => gra.width = ev[1].as_f64().unwrap() as f32,
            "tstamp" => gra.tstamp = sym_or_str(elem.get(1)),
            _ => println!("unknown cons in gr_arc: {:#?}", elem),
        }
    }

    gra
}

fn parse_effects(v: Vec<lexpr::Value>) -> Effects {
    let mut eff = Effects::default();

    for elem in v.iter() {
        if elem.is_symbol() {
            continue;
        }
        let ev = elem.to_vec().unwrap();
        let label = ev[0].to_string();

        match label.as_str() {
            "font" => eff.font = parse_font(ev),
            "justify" => eff.justify = sym_or_str(elem.get(1)),
            _ => println!("unknown cons in effects: {:#?}", elem),
        }
    }

    eff
}

fn parse_font(v: Vec<lexpr::Value>) -> Font {
    let mut fnt = Font::default();

    for elem in v.iter() {
        if elem.is_symbol() {
            continue;
        }
        let ev = elem.to_vec().unwrap();
        let label = ev[0].to_string();

        match label.as_str() {
            "size" => {
                fnt.size = (
                    ev[1].as_f64().unwrap() as f32,
                    ev[2].as_f64().unwrap() as f32,
                )
            }
            "thickness" => fnt.thickness = ev[1].as_f64().unwrap() as f32,
            _ => println!("unknown cons in font: {:#?}", elem),
        }
    }

    fnt
}

fn parse_dimension(v: Vec<lexpr::Value>) -> Dimension {
    let mut dim = Dimension::default();

    for elem in v.iter() {
        if elem.is_symbol() {
            continue;
        } else if elem.is_number() {
            dim.num = elem.as_u64().unwrap() as u32
        } else {
            let ev = elem.to_vec().unwrap();
            let label = ev[0].to_string();

            match label.as_str() {
                "width" => dim.width = ev[1].as_f64().unwrap() as f32,
                "layer" => dim.layer = sym_or_str(elem.get(1)),
                "gr_text" => dim.text = parse_gr_text(ev),
                "feature1" => dim.feature1 = parse_pts(ev[1].to_vec().unwrap()),
                "feature2" => dim.feature2 = parse_pts(ev[1].to_vec().unwrap()),
                "crossbar" => dim.crossbar = parse_pts(ev[1].to_vec().unwrap()),
                "arrow1a" => dim.arrow1a = parse_pts(ev[1].to_vec().unwrap()),
                "arrow1b" => dim.arrow1b = parse_pts(ev[1].to_vec().unwrap()),
                "arrow2a" => dim.arrow1a = parse_pts(ev[1].to_vec().unwrap()),
                "arrow2b" => dim.arrow1b = parse_pts(ev[1].to_vec().unwrap()),
                _ => println!("unknown cons in dimension: {:#?}", elem),
            }
        }
    }

    dim
}

fn parse_keepout(v: Vec<lexpr::Value>) -> Keepout {
    let mut k = Keepout::default();

    for elem in v.iter() {
        if elem.is_symbol() {
            continue;
        }
        let ev = elem.to_vec().unwrap();
        let label = ev[0].to_string();

        match label.as_str() {
            "tracks" => k.tracks = sym_or_str(elem.get(1)),
            "vias" => k.vias = sym_or_str(elem.get(1)),
            "copperpour" => k.copperpour = sym_or_str(elem.get(1)),
            _ => println!("unknown cons in dimension: {:#?}", elem),
        }
    }

    k
}

fn parse_zone(v: Vec<lexpr::Value>) -> Zone {
    let mut zone = Zone::default();

    for elem in v.iter() {
        if elem.is_symbol() {
            continue;
        }
        let mut ev = elem.to_vec().unwrap();

        // TODO: fix tstamp handling in lexpr
        if ev[0].is_cons() {
            ev = ev[0].to_vec().unwrap()
        }

        let label = ev[0].to_string();

        match label.as_str() {
            "net" => zone.net = ev[1].as_u64().unwrap() as u32,
            "net_name" => zone.net_name = sym_or_str(elem.get(1)),
            "layer" => zone.layers.push(ev[1].to_string()),
            "layers" => {
                for l in ev[1..].iter() {
                    zone.layers.push(l.to_string())
                }
            }
            "tstamp" => zone.tstamp = sym_or_str(elem.get(1)),
            "hatch" => {
                zone.hatch = (
                    ev[1].as_symbol().unwrap().to_string(),
                    ev[2].as_f64().unwrap() as f32,
                );
            }
            "priority" => zone.priority = ev[1].as_u64().unwrap() as u8,
            "connect_pads" => zone.connect_pads = parse_connect_pads(ev),
            "min_thickness" => zone.min_thickness = ev[1].as_f64().unwrap() as f32,
            "fill" => zone.fill = parse_fill(ev),
            "polygon" => zone.polygons.push(Polygon {
                filled: false,
                points: parse_pts(ev[1].to_vec().unwrap()),
            }),
            "filled_polygon" => zone.polygons.push(Polygon {
                filled: false,
                points: parse_pts(ev[1].to_vec().unwrap()),
            }),
            "keepout" => zone.keepout = parse_keepout(ev),
            _ => println!("unknown cons in zone: {} {:#?}", label, elem),
        }
    }

    zone
}

fn parse_connect_pads(v: Vec<lexpr::Value>) -> (String, f32) {
    let s = sym_or_str(v.get(1));
    if s.as_str() == "yes" {
        let clearance = v[2].to_vec().unwrap()[1].as_f64().unwrap() as f32;
        return (s, clearance);
    }

    (String::from("no"), 0.0)
}

fn parse_fill(v: Vec<lexpr::Value>) -> ZoneFill {
    let mut fill = ZoneFill::default();

    for elem in v.iter() {
        if elem.is_symbol() {
            fill.do_fill = if elem.as_symbol().unwrap() == "yes" {
                true
            } else {
                false
            };
            continue;
        };

        let ev = elem.to_vec().unwrap();
        let label = ev[0].to_string();

        match label.as_str() {
            "arc_segments" => fill.arc_segments = ev[1].as_u64().unwrap() as u8,
            "thermal_gap" => fill.thermal_gap = ev[1].as_f64().unwrap() as f32,
            "thermal_bridge_width" => fill.thermal_bridge_width = ev[1].as_f64().unwrap() as f32,
            "smoothing" => fill.smoothing = ev[1].as_symbol().unwrap().to_string(),
            "radius" => fill.radius = ev[1].as_f64().unwrap() as f32,
            _ => println!("unknown cons in fill: {:#?}", elem),
        }
    }

    fill
}

fn parse_fp_text(v: Vec<lexpr::Value>) -> FootprintText {
    let mut fpt = FootprintText::default();

    fpt.typ = sym_or_str(v.get(1));
    fpt.value = sym_or_str(v.get(2));

    for elem in v[3..].iter() {
        if elem.is_symbol() {
            fpt.hide = if elem.as_symbol().unwrap() == "hide" {
                true
            } else {
                false
            };
            continue;
        }

        let ev = elem.to_vec().unwrap();
        let label = ev[0].to_string();

        match label.as_str() {
            "at" => fpt.at = parse_vecf(ev),
            "layer" => fpt.layer = sym_or_str(elem.get(1)),
            "effects" => fpt.effects = parse_effects(ev),
            _ => println!("unknown cons in fp_text: {:#?}", elem),
        }
    }

    fpt
}

fn parse_fp_line(v: Vec<lexpr::Value>) -> FootprintLine {
    let mut fpl = FootprintLine::default();

    for elem in v[1..].iter() {
        let ev = elem.to_vec().unwrap();
        let label = ev[0].to_string();

        match label.as_str() {
            "start" => {
                fpl.start = (
                    ev[1].as_f64().unwrap() as f32,
                    ev[2].as_f64().unwrap() as f32,
                )
            }
            "end" => {
                fpl.end = (
                    ev[1].as_f64().unwrap() as f32,
                    ev[2].as_f64().unwrap() as f32,
                )
            }
            "layer" => fpl.layer = sym_or_str(elem.get(1)),
            "width" => fpl.width = ev[1].as_f64().unwrap() as f32,
            _ => println!("unknown cons in fp_line: {:#?}", elem),
        }
    }

    fpl
}

fn parse_fp_arc(v: Vec<lexpr::Value>) -> FootprintArc {
    let mut fpa = FootprintArc::default();

    for elem in v.iter() {
        if !elem.is_cons() {
            continue;
        }

        let ev = elem.to_vec().unwrap();
        let label = ev[0].to_string();

        match label.as_str() {
            "start" => {
                fpa.start = (
                    ev[1].as_f64().unwrap() as f32,
                    ev[2].as_f64().unwrap() as f32,
                )
            }
            "end" => {
                fpa.end = (
                    ev[1].as_f64().unwrap() as f32,
                    ev[2].as_f64().unwrap() as f32,
                )
            }
            "angle" => fpa.angle = ev[1].as_f64().unwrap() as f32,
            "layer" => fpa.layer = sym_or_str(elem.get(1)),
            "width" => fpa.width = ev[1].as_f64().unwrap() as f32,
            _ => println!("unknown cons in fp_arc: {:#?}", elem),
        }
    }

    fpa
}

fn parse_fp_circle(v: Vec<lexpr::Value>) -> FootprintCircle {
    let mut fpc = FootprintCircle::default();

    for elem in v.iter() {
        if !elem.is_cons() {
            continue;
        }

        let ev = elem.to_vec().unwrap();
        let label = ev[0].to_string();

        match label.as_str() {
            "center" => {
                fpc.center = (
                    ev[1].as_f64().unwrap() as f32,
                    ev[2].as_f64().unwrap() as f32,
                )
            }
            "end" => {
                fpc.end = (
                    ev[1].as_f64().unwrap() as f32,
                    ev[2].as_f64().unwrap() as f32,
                )
            }
            "layer" => fpc.layer = sym_or_str(elem.get(1)),
            "width" => fpc.width = ev[1].as_f64().unwrap() as f32,
            _ => println!("unknown cons in fp_circle: {:#?}", elem),
        }
    }

    fpc
}

fn parse_fp_poly(v: Vec<lexpr::Value>) -> FootprintPoly {
    let mut fpp = FootprintPoly::default();

    for elem in v.iter() {
        if !elem.is_cons() {
            continue;
        }

        let ev = elem.to_vec().unwrap();
        let label = ev[0].to_string();

        match label.as_str() {
            "pts" => fpp.points = parse_pts(ev),
            "layer" => fpp.layer = sym_or_str(elem.get(1)),
            "width" => fpp.width = ev[1].as_f64().unwrap() as f32,
            _ => println!("unknown cons in fp_arc: {:#?}", elem),
        }
    }

    fpp
}

fn parse_model(v: Vec<lexpr::Value>) -> Model {
    let mut model = Model::default();

    model.path = sym_or_str(v.get(1));

    for elem in v[2..].iter() {
        let ev = elem.to_vec().unwrap();
        let label = ev[0].to_string();

        match label.as_str() {
            "at" => model.at = parse_vecf(ev[1].to_vec().unwrap()),
            "scale" => model.scale = parse_vecf(ev[1].to_vec().unwrap()),
            "rotate" => model.rotate = parse_vecf(ev[1].to_vec().unwrap()),
            _ => println!("unknown cons in model: {:#?}", elem),
        }
    }

    model
}

fn parse_pad_options(v: Vec<lexpr::Value>) -> PadOptions {
    let mut po = PadOptions::default();

    for elem in v.iter() {
        if elem.is_symbol() {
            continue;
        }
        let ev = elem.to_vec().unwrap();
        let label = ev[0].to_string();

        match label.as_str() {
            "clearance" => po.clearance = sym_or_str(elem.get(1)),
            "anchor" => po.anchor = sym_or_str(elem.get(1)),
            _ => println!("unknown cons in pad_options: {:#?}", elem),
        }
    }

    po
}

fn parse_pad_primitives(v: Vec<lexpr::Value>) -> PadPrimitives {
    let mut pp = PadPrimitives::default();

    for elem in v.iter() {
        if elem.is_symbol() {
            continue;
        }
        let ev = elem.to_vec().unwrap();
        let label = ev[0].to_string();

        match label.as_str() {
            "gr_poly" => pp.gr_poly = parse_gr_poly(ev),
            _ => println!("unknown cons in pad_options: {:#?}", elem),
        }
    }

    pp
}

fn parse_pad(v: Vec<lexpr::Value>) -> Pad {
    let mut pad = Pad::default();

    pad.num = sym_or_str(v.get(1)).to_string();
    pad.pad_type = sym_or_str(v.get(2)).to_string();
    pad.typ = sym_or_str(v.get(3)).to_string();

    for elem in v[4..].iter() {
        let ev = elem.to_vec().unwrap();
        let label = ev[0].to_string();

        match label.as_str() {
            "at" => pad.at = parse_vecf(ev),
            "size" => pad.size = parse_vecf(ev),
            "layers" => {
                for l in ev[1..].iter() {
                    pad.layers.push(l.as_symbol().unwrap().to_string())
                }
            }
            "drill" => {
                if ev[1].is_symbol() {
                    if ev[1].as_symbol().unwrap() == "oval" {
                        pad.drill_oval = (
                            ev[2].as_f64().unwrap() as f32,
                            ev[3].as_f64().unwrap() as f32,
                        )
                    }
                } else {
                    pad.drill = ev[1].as_f64().unwrap() as f32
                }
            }
            "net" => pad.net = parse_net(ev),
            "roundrect_rratio" => pad.roundrect_rratio = ev[1].as_f64().unwrap() as f32,
            "zone_connect" => pad.zone_connect = ev[1].as_u64().unwrap() as u8,
            "options" => pad.options = parse_pad_options(ev),
            "primitives" => pad.primitives = parse_pad_primitives(ev),
            _ => println!("unknown cons in pad: {:#?}", elem),
        }
    }

    pad
}

fn parse_module(v: Vec<lexpr::Value>) -> Module {
    let mut module = Module::default();
    let mut it = v.iter();

    it.next(); // advance to the symbol after "module"

    module.name = sym_or_str(it.next());

    for elem in it {
        let mut ev = elem.to_vec().unwrap();

        // TODO: fix tstamp handling in lexpr
        if ev[0].is_cons() {
            ev = ev[0].to_vec().unwrap()
        }

        let label = ev[0].to_string();

        match label.as_str() {
            "layer" => module.layer = sym_or_str(elem.get(1)),
            "tstamp" => module.tstamp = sym_or_str(elem.get(1)),
            "tedit" => module.tedit = sym_or_str(elem.get(1)),
            "at" => module.at = parse_vecf(ev),
            "descr" => module.layer = sym_or_str(elem.get(1)),
            "tags" => module.tags = sym_or_str(elem.get(1)),
            "path" => module.path = sym_or_str(elem.get(1)),
            "attr" => module.attr = sym_or_str(elem.get(1)),
            "fp_text" => module.fp_texts.push(parse_fp_text(ev)),
            "fp_line" => module.fp_lines.push(parse_fp_line(ev)),
            "fp_arc" => module.fp_arcs.push(parse_fp_arc(ev)),
            "fp_circle" => module.fp_circles.push(parse_fp_circle(ev)),
            "fp_poly" => module.fp_polys.push(parse_fp_poly(ev)),
            "pad" => module.pads.push(parse_pad(ev)),
            "model" => module.models.push(parse_model(ev)),
            _ => println!("unknown cons in module: {} {:#?}", label, ev),
        }
    }

    module
}

