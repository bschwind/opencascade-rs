use clap::ValueEnum;
use opencascade::primitives::Shape;

pub mod airfoil;
pub mod bounding_box;
pub mod box_shape;
pub mod cable_bracket;
pub mod chamfer;
pub mod flat_ethernet_bracket;
pub mod gizmo;
pub mod heater_coil;
pub mod high_level_bottle;
pub mod keyboard_case;
pub mod keycap;
pub mod letter_a;
pub mod offset_2d;
pub mod rounded_chamfer;
pub mod section;
pub mod swept_face;
pub mod swept_face_variable;
pub mod swept_wire;
pub mod swept_wire_variable;
pub mod turners_cube;
pub mod variable_fillet;
pub mod zbox_case;

#[derive(Debug, Copy, Clone, PartialEq, ValueEnum)]
pub enum Example {
    Airfoil,
    BoundingBox,
    CableBracket,
    BoxShape,
    Chamfer,
    FlatEthernetBracket,
    Gizmo,
    HeaterCoil,
    HighLevelBottle,
    KeyboardCase,
    Keycap,
    LetterA,
    Offset2d,
    RoundedChamfer,
    Section,
    SweptFace,
    SweptFaceVariable,
    SweptWire,
    SweptWireVariable,
    TurnersCube,
    VariableFillet,
    ZboxCase,
}

impl Example {
    pub fn shape(self) -> Shape {
        match self {
            Example::Airfoil => airfoil::shape(),
            Example::BoundingBox => bounding_box::shape(),
            Example::CableBracket => cable_bracket::shape(),
            Example::BoxShape => box_shape::shape(),
            Example::Chamfer => chamfer::shape(),
            Example::FlatEthernetBracket => flat_ethernet_bracket::shape(),
            Example::Gizmo => gizmo::shape(),
            Example::HeaterCoil => heater_coil::shape(),
            Example::HighLevelBottle => high_level_bottle::shape(),
            Example::KeyboardCase => keyboard_case::shape(),
            Example::Keycap => keycap::shape(),
            Example::LetterA => letter_a::shape(),
            Example::Offset2d => offset_2d::shape(),
            Example::RoundedChamfer => rounded_chamfer::shape(),
            Example::Section => section::shape(),
            Example::SweptFace => swept_face::shape(),
            Example::SweptFaceVariable => swept_face_variable::shape(),
            Example::SweptWire => swept_wire::shape(),
            Example::SweptWireVariable => swept_wire_variable::shape(),
            Example::TurnersCube => turners_cube::shape(),
            Example::VariableFillet => variable_fillet::shape(),
            Example::ZboxCase => zbox_case::shape(),
        }
    }
}
