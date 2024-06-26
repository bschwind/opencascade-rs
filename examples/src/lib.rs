use clap::ValueEnum;
use opencascade::primitives::Shape;

pub mod airfoil;
pub mod box_shape;
pub mod cable_bracket;
pub mod chamfer;
pub mod gizmo;
pub mod heater_coil;
pub mod high_level_bottle;
pub mod keyboard_case;
pub mod keycap;
pub mod offset_2d;
pub mod rounded_chamfer;
pub mod swept_face;
pub mod swept_face_variable;
pub mod swept_wire;
pub mod swept_wire_variable;
pub mod turners_cube;
pub mod variable_fillet;

#[derive(Debug, Copy, Clone, PartialEq, ValueEnum)]
pub enum Example {
    Airfoil,
    CableBracket,
    BoxShape,
    Chamfer,
    Gizmo,
    HeaterCoil,
    HighLevelBottle,
    KeyboardCase,
    Keycap,
    Offset2d,
    RoundedChamfer,
    SweptFace,
    SweptFaceVariable,
    SweptWire,
    SweptWireVariable,
    TurnersCube,
    VariableFillet,
}

impl Example {
    pub fn shape(self) -> Shape {
        match self {
            Example::Airfoil => airfoil::shape(),
            Example::CableBracket => cable_bracket::shape(),
            Example::BoxShape => box_shape::shape(),
            Example::Chamfer => chamfer::shape(),
            Example::Gizmo => gizmo::shape(),
            Example::HeaterCoil => heater_coil::shape(),
            Example::HighLevelBottle => high_level_bottle::shape(),
            Example::KeyboardCase => keyboard_case::shape(),
            Example::Keycap => keycap::shape(),
            Example::Offset2d => offset_2d::shape(),
            Example::RoundedChamfer => rounded_chamfer::shape(),
            Example::SweptFace => swept_face::shape(),
            Example::SweptFaceVariable => swept_face_variable::shape(),
            Example::SweptWire => swept_wire::shape(),
            Example::SweptWireVariable => swept_wire_variable::shape(),
            Example::TurnersCube => turners_cube::shape(),
            Example::VariableFillet => variable_fillet::shape(),
        }
    }
}
