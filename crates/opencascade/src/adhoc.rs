use crate::{
    primitives::{Face, Solid, Wire},
    Error,
};
use glam::{dvec3, DVec3};

/// Collections of helper functions for the [`Shape`] struct that provides an "ad-hoc"
/// API. New code is encouraged to use more fine-grained API in the [`primitives`] module.
pub struct AdHocShape;

impl AdHocShape {
    /// Purposefully underpowered for now, this simply takes a list of points,
    /// creates a face out of them, and then extrudes it by h in the positive Z
    /// direction.
    pub fn extrude_polygon(
        points: impl IntoIterator<Item = DVec3>,
        h: f64,
    ) -> Result<Solid, Error> {
        let wire = Wire::from_ordered_points(points)?;
        Ok(Face::from_wire(&wire).extrude(dvec3(0.0, 0.0, h)))
    }
}
