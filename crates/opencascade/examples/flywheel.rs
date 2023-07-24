use glam::dvec3;
use opencascade::{
    angle::Angle,
    primitives::Shape,
    workplane::Workplane,
};

pub fn main() {
    let outer: f64 = 30.0;
    let height: f64 = 8.0;
    let thickness: f64 = 8.0;
    let spoke_count: usize = 7;
    let hub_outer: f64 = 8.0;

    let ring = rim(outer,thickness,height);
    let sp = spokes(outer-thickness/2.0,thickness/2.0,spoke_count);
    let (mut fly  ,smooth ) = ring.union_shape(&sp);
    fly.fillet_edges(1.0,smooth);

    let the_hub = hub(hub_outer, thickness);
    let (mut fly , hub_con) = fly.union_shape(&the_hub);
    fly.fillet_edges(1.0,hub_con);

    fly.write_stl("flywheel.stl").unwrap();
}

fn rim(outer:f64 , thickness: f64,height: f64) -> Shape { 
    let outer_wire = Workplane::xy().circle(0., 0., outer);
    let mut ring = outer_wire.to_face().extrude(dvec3(0.0, 0.0, height)).to_shape();
    let inner_wire = Workplane::xy().circle(0.,0.0, outer - thickness/2.0);
    let inner_ring = inner_wire.to_face().extrude(dvec3(0.0,0.0,height)).to_shape();
    (ring, _ ) = ring.subtract_shape(&inner_ring);
    ring.chamfer(0.1);
    ring    
}

fn hub(outer: f64 , height: f64) -> Shape { 
    let outer_wire = Workplane::xy().circle(0., 0., outer);
    let mut hub = outer_wire.to_face().extrude(dvec3(0.0, 0.0, height)).to_shape();
    hub.chamfer(0.1);
    hub
}

fn spoke(length: f64,thickness:f64,rotation:f64) -> Shape { 
    let mut wp = Workplane::xy();
    wp.set_rotation((Angle::Degrees(90.0),Angle::Degrees(rotation),Angle::Degrees(0.0)));
    let inner_wire = wp.circle(0., 0., thickness/2.0);
    let mut spoke = inner_wire.to_face().extrude(wp.normal()*length).to_shape();
    spoke.set_global_translation(dvec3(0.0, 0.0, thickness));
    spoke
}

fn spokes(length: f64 ,  thickness: f64,count: usize) -> Shape { 
    let incr = 360.0 / count as f64;
    let mut first_s = spoke(length,thickness,0.0);
    for i in 1..count { 
        let s = spoke(length,thickness,incr * i as f64);
        (first_s , _) = first_s.union_shape(&s);
    }
    first_s
}
