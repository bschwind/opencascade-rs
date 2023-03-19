use cxx::UniquePtr;
use glam::f64::{dvec3, DAffine3, DVec3};
use opencascade_sys::ffi::{
    new_point, write_stl, BRepAlgoAPI_Fuse_ctor, BRepMesh_IncrementalMesh_ctor,
    BRepPrimAPI_MakeBox_ctor, StlAPI_Writer_ctor, TopoDS_Shape, TopoDS_Shape_to_owned,
};
use std::{collections::HashMap, path::Path};

#[derive(Debug)]
pub enum TaggedItem {
    Vertex,
    VertexSet,
    Edge,
    EdgeSet,
    Wire,
    WireSet,
    Face,
    FaceSet,
    Shell,
    ShellSet,
    Solid,
    SolidSet,
    Shape,
    ShapeSet,
}

pub trait Taggable {}

#[derive(Debug, Copy, Clone)]
pub enum Plane {
    XY,
    YZ,
    ZX,
    XZ,
    YX,
    ZY,
    Front,
    Back,
    Left,
    Right,
    Top,
    Bottom,
    Custom { origin: (f64, f64, f64), x_dir: (f64, f64, f64), normal_dir: (f64, f64, f64) },
}

impl Plane {
    pub fn transform_point(&self, point: DVec3) -> DVec3 {
        self.transform().transform_point3(point)
    }

    pub fn transform(&self) -> DAffine3 {
        match self {
            Self::XY => DAffine3::from_cols(
                dvec3(1.0, 0.0, 0.0),
                dvec3(0.0, 1.0, 0.0),
                dvec3(0.0, 0.0, 1.0),
                dvec3(0.0, 0.0, 0.0),
            ),
            Self::YZ => {
                // TODO - fix this
                DAffine3::from_cols(
                    dvec3(1.0, 0.0, 0.0),
                    dvec3(0.0, 1.0, 0.0),
                    dvec3(0.0, 0.0, 1.0),
                    dvec3(0.0, 0.0, 0.0),
                )
            },
            Self::ZX => {
                // TODO - fix this
                DAffine3::from_cols(
                    dvec3(1.0, 0.0, 0.0),
                    dvec3(0.0, 1.0, 0.0),
                    dvec3(0.0, 0.0, 1.0),
                    dvec3(0.0, 0.0, 0.0),
                )
            },
            Self::XZ => {
                // TODO - fix this
                DAffine3::from_cols(
                    dvec3(1.0, 0.0, 0.0),
                    dvec3(0.0, 1.0, 0.0),
                    dvec3(0.0, 0.0, 1.0),
                    dvec3(0.0, 0.0, 0.0),
                )
            },
            Self::YX => {
                // TODO - fix this
                DAffine3::from_cols(
                    dvec3(1.0, 0.0, 0.0),
                    dvec3(0.0, 1.0, 0.0),
                    dvec3(0.0, 0.0, 1.0),
                    dvec3(0.0, 0.0, 0.0),
                )
            },
            Self::ZY => {
                // TODO - fix this
                DAffine3::from_cols(
                    dvec3(1.0, 0.0, 0.0),
                    dvec3(0.0, 1.0, 0.0),
                    dvec3(0.0, 0.0, 1.0),
                    dvec3(0.0, 0.0, 0.0),
                )
            },
            Self::Front => {
                // TODO - fix this
                DAffine3::from_cols(
                    dvec3(1.0, 0.0, 0.0),
                    dvec3(0.0, 1.0, 0.0),
                    dvec3(0.0, 0.0, 1.0),
                    dvec3(0.0, 0.0, 0.0),
                )
            },
            Self::Back => {
                // TODO - fix this
                DAffine3::from_cols(
                    dvec3(1.0, 0.0, 0.0),
                    dvec3(0.0, 1.0, 0.0),
                    dvec3(0.0, 0.0, 1.0),
                    dvec3(0.0, 0.0, 0.0),
                )
            },
            Self::Left => {
                // TODO - fix this
                DAffine3::from_cols(
                    dvec3(1.0, 0.0, 0.0),
                    dvec3(0.0, 1.0, 0.0),
                    dvec3(0.0, 0.0, 1.0),
                    dvec3(0.0, 0.0, 0.0),
                )
            },
            Self::Right => DAffine3::from_cols(
                dvec3(0.0, 0.0, -1.0),
                dvec3(0.0, 1.0, 0.0),
                dvec3(1.0, 0.0, 0.0),
                dvec3(0.0, 0.0, 0.0),
            ),
            Self::Top => DAffine3::from_cols(
                dvec3(1.0, 0.0, 0.0),
                dvec3(0.0, 0.0, -1.0),
                dvec3(0.0, 1.0, 0.0),
                dvec3(0.0, 0.0, 0.0),
            ),
            Self::Bottom => {
                // TODO - fix this
                DAffine3::from_cols(
                    dvec3(1.0, 0.0, 0.0),
                    dvec3(0.0, 1.0, 0.0),
                    dvec3(0.0, 0.0, 1.0),
                    dvec3(0.0, 0.0, 0.0),
                )
            },
            Self::Custom { origin, x_dir, normal_dir } => {
                let x_axis = dvec3(x_dir.0, x_dir.1, x_dir.2);
                let z_axis = dvec3(normal_dir.0, normal_dir.1, normal_dir.2);
                let y_axis = z_axis.cross(x_axis);

                DAffine3::from_cols(x_axis, y_axis, z_axis, dvec3(origin.0, origin.1, origin.2))
            },
        }
    }
}

pub struct Workspace {
    tagged_items: HashMap<String, TaggedItem>, // TODO - replace without a "stringly" typed key
    shapes: Vec<UniquePtr<TopoDS_Shape>>,
}

impl Default for Workspace {
    fn default() -> Self {
        Self::new()
    }
}

impl Workspace {
    pub fn new() -> Self {
        Self { tagged_items: HashMap::new(), shapes: vec![] }
    }

    pub fn add(&mut self, solid: Solid) {
        self.shapes.push(solid.shape);
    }

    pub fn workplane(&mut self, plane: Plane) -> WorkPlane {
        WorkPlane { plane }
    }

    pub fn sketch(&mut self) -> Sketch {
        self.workplane(Plane::XY).sketch()
    }

    pub fn write_stl<P: AsRef<Path>>(self, path: P) -> Result<(), ()> {
        let result_shape = self.shapes.into_iter().reduce(|acc, shape| {
            let mut fuse_operation = BRepAlgoAPI_Fuse_ctor(&acc, &shape);

            let cut_shape = fuse_operation.pin_mut().Shape();
            TopoDS_Shape_to_owned(cut_shape)
        });

        if let Some(output_shape) = result_shape {
            let mut stl_writer = StlAPI_Writer_ctor();
            let triangulation = BRepMesh_IncrementalMesh_ctor(&output_shape, 0.001);
            let success = write_stl(
                stl_writer.pin_mut(),
                triangulation.Shape(),
                path.as_ref().to_string_lossy().to_string(),
            );

            if success {
                Ok(())
            } else {
                Err(()) // TODO(bschwind) - Make an error type
            }
        } else {
            Ok(())
        }
    }
}

pub struct WorkPlane {
    plane: Plane,
}

impl WorkPlane {
    pub fn sketch(&mut self) -> Sketch {
        Sketch { plane: self.plane }
    }
}

pub struct Solid {
    shape: UniquePtr<TopoDS_Shape>,
}

pub struct Sketch {
    plane: Plane,
}

impl Sketch {
    pub fn circle(&mut self) -> Circle {
        Circle {}
    }

    pub fn rect(&mut self, width: f64, height: f64) -> Rect {
        Rect { width, height, plane: self.plane }
    }

    pub fn freeform() -> FreeformSketch {
        FreeformSketch {}
    }
}

pub struct Circle {}

pub struct Rect {
    width: f64,
    height: f64,
    plane: Plane,
}

impl Rect {
    pub fn extrude(self, amount: f64) -> Solid {
        let p1 = dvec3(0.0, 0.0, 0.0);
        let p2 = dvec3(self.width, self.height, amount);

        let p1 = self.plane.transform_point(p1);
        let p2 = self.plane.transform_point(p2);

        let min_corner = p1.min(p2);
        let max_corner = p1.max(p2);

        let point = new_point(min_corner.x, min_corner.y, min_corner.z);
        let diff = max_corner - min_corner;
        let mut my_box = BRepPrimAPI_MakeBox_ctor(&point, diff.x, diff.y, diff.z);
        let shape = TopoDS_Shape_to_owned(my_box.pin_mut().Shape());

        Solid { shape }
    }
}

pub struct FreeformSketch {}

impl FreeformSketch {
    pub fn move_to(mut self) -> Self {
        self
    }

    pub fn line_to(mut self) -> Self {
        self
    }
}
