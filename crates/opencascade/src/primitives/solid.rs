use crate::{
    primitives::{BooleanShape, Compound, Edge, Face, Shape, Wire},
    Error,
};
use cxx::UniquePtr;
use glam::{dvec3, DVec3};
use opencascade_sys as ffi;

pub struct Solid {
    pub(crate) inner: UniquePtr<ffi::topo_ds::TopoDS_Solid>,
}

impl AsRef<Solid> for Solid {
    fn as_ref(&self) -> &Solid {
        self
    }
}

impl Solid {
    pub(crate) fn from_solid(solid: &ffi::topo_ds::TopoDS_Solid) -> Self {
        let inner = ffi::topo_ds::TopoDS_Solid_to_owned(solid);

        Self { inner }
    }

    // TODO(bschwind) - Do some cool stuff from this link:
    // https://neweopencascade.wordpress.com/2018/10/17/lets-talk-about-fillets/
    // Key takeaway: Use the `SectionEdges` function to retrieve edges that were
    // the result of combining two shapes.
    #[must_use]
    pub fn fillet_edge(&self, radius: f64, edge: &Edge) -> Compound {
        let inner_shape = ffi::topo_ds::cast_solid_to_shape(&self.inner);

        let mut make_fillet = ffi::b_rep_fillet_api::BRepFilletAPI_MakeFillet_new(inner_shape);
        make_fillet.pin_mut().add_edge(radius, &edge.inner);

        let filleted_shape = make_fillet.pin_mut().Shape();

        let compound = ffi::topo_ds::TopoDS::Compound(filleted_shape);

        Compound::from_compound(compound)
    }

    pub fn loft<T: AsRef<Wire>>(wires: impl IntoIterator<Item = T>) -> Self {
        let is_solid = true;
        let mut make_loft = ffi::b_rep_offset_api::BRepOffsetAPI_ThruSections_new(is_solid);

        for wire in wires.into_iter() {
            make_loft.pin_mut().AddWire(&wire.as_ref().inner);
        }

        // Set to CheckCompatibility to `true` to avoid twisted results.
        make_loft.pin_mut().CheckCompatibility(true);

        let shape = make_loft.pin_mut().Shape();
        let solid = ffi::topo_ds::TopoDS::Solid(shape);

        Self::from_solid(solid)
    }

    #[must_use]
    pub fn subtract(&self, other: &Solid) -> BooleanShape {
        let inner_shape = ffi::topo_ds::cast_solid_to_shape(&self.inner);
        let other_inner_shape = ffi::topo_ds::cast_solid_to_shape(&other.inner);

        let mut cut_operation =
            ffi::b_rep_algo_api::BRepAlgoAPI_Cut_new(inner_shape, other_inner_shape);

        let edge_list = cut_operation.pin_mut().SectionEdges();
        let vec = ffi::topo_ds::shape_list_to_vector(edge_list);

        let mut new_edges = vec![];
        for shape in vec.iter() {
            let edge = ffi::topo_ds::TopoDS::Edge(shape);
            new_edges.push(Edge::from_edge(edge));
        }

        let shape = Shape::from_shape(cut_operation.pin_mut().Shape());

        BooleanShape { shape, new_edges }
    }

    #[must_use]
    pub fn union(&self, other: &Solid) -> BooleanShape {
        let inner_shape = ffi::topo_ds::cast_solid_to_shape(&self.inner);
        let other_inner_shape = ffi::topo_ds::cast_solid_to_shape(&other.inner);

        let mut fuse_operation =
            ffi::b_rep_algo_api::BRepAlgoAPI_Fuse_new(inner_shape, other_inner_shape);
        let edge_list = fuse_operation.pin_mut().SectionEdges();
        let vec = ffi::topo_ds::shape_list_to_vector(edge_list);

        let mut new_edges = vec![];
        for shape in vec.iter() {
            let edge = ffi::topo_ds::TopoDS::Edge(shape);
            new_edges.push(Edge::from_edge(edge));
        }

        let shape = Shape::from_shape(fuse_operation.pin_mut().Shape());

        BooleanShape { shape, new_edges }
    }

    #[must_use]
    pub fn intersect(&self, other: &Solid) -> BooleanShape {
        let inner_shape = ffi::topo_ds::cast_solid_to_shape(&self.inner);
        let other_inner_shape = ffi::topo_ds::cast_solid_to_shape(&other.inner);

        let mut fuse_operation =
            ffi::b_rep_algo_api::BRepAlgoAPI_Common_new(inner_shape, other_inner_shape);
        let edge_list = fuse_operation.pin_mut().SectionEdges();
        let vec = ffi::topo_ds::shape_list_to_vector(edge_list);

        let mut new_edges = vec![];
        for shape in vec.iter() {
            let edge = ffi::topo_ds::TopoDS::Edge(shape);
            new_edges.push(Edge::from_edge(edge));
        }

        let shape = Shape::from_shape(fuse_operation.pin_mut().Shape());

        BooleanShape { shape, new_edges }
    }

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

    #[must_use]
    pub fn volume(&self) -> f64 {
        let mut props = ffi::g_prop::GProps_new();
        let inner_shape = ffi::topo_ds::cast_solid_to_shape(&self.inner);
        ffi::b_rep_g_prop::BRepGProp::VolumeProperties(
            inner_shape,
            props.pin_mut(),
            true,
            false,
            false,
        );
        props.Mass()
    }

    #[must_use]
    pub fn center_of_mass(&self) -> DVec3 {
        let mut props = ffi::g_prop::GProps_new();
        let inner_shape = ffi::topo_ds::cast_solid_to_shape(&self.inner);
        ffi::b_rep_g_prop::BRepGProp::VolumeProperties(
            inner_shape,
            props.pin_mut(),
            true,
            false,
            false,
        );
        let center = ffi::g_prop::GProp_GProps_CentreOfMass(&props);
        dvec3(center.X(), center.Y(), center.Z())
    }

    #[must_use]
    pub fn surface_area(&self) -> f64 {
        let mut props = ffi::g_prop::GProps_new();
        let inner_shape = ffi::topo_ds::cast_solid_to_shape(&self.inner);
        ffi::b_rep_g_prop::BRepGProp::SurfaceProperties(inner_shape, props.pin_mut(), false, false);
        props.Mass()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volume_of_box() {
        let shape = Shape::box_centered(10.0, 10.0, 10.0);
        let solid = shape.expect_solid();
        let volume = solid.volume();
        assert!((volume - 1000.0).abs() <= 0.00001, "Expected volume ~1000.0, got {volume}",);
    }

    #[test]
    fn test_volume_of_cylinder() {
        let shape = Shape::cylinder_radius_height(3.0, 15.0);
        let solid = shape.expect_solid();
        let volume = solid.volume();
        let expected = std::f64::consts::PI * 9.0 * 15.0;
        assert!((volume - expected).abs() <= 0.001, "Expected volume ~{expected}, got {volume}",);
    }

    #[test]
    fn test_center_of_mass_of_centered_box() {
        let shape = Shape::box_centered(10.0, 10.0, 10.0);
        let solid = shape.expect_solid();
        let com = solid.center_of_mass();
        assert!(
            com.distance_squared(dvec3(0.0, 0.0, 0.0)) <= 0.00001,
            "Expected center of mass at (0, 0, 0), got ({}, {}, {})",
            com.x,
            com.y,
            com.z,
        );
    }

    #[test]
    fn test_center_of_mass_of_translated_box() {
        let shape = Shape::box_with_dimensions(10.0, 10.0, 10.0).translated(dvec3(5.0, 5.0, 5.0));
        let solid = shape.expect_solid();
        let com = solid.center_of_mass();
        assert!(
            com.distance_squared(dvec3(10.0, 10.0, 10.0)) <= 0.00001,
            "Expected center of mass at (10, 10, 10), got ({}, {}, {})",
            com.x,
            com.y,
            com.z,
        );
    }

    #[test]
    fn test_surface_area_of_box() {
        let shape = Shape::box_centered(10.0, 10.0, 10.0);
        let solid = shape.expect_solid();
        let area = solid.surface_area();
        assert!((area - 600.0).abs() <= 0.00001, "Expected surface area ~600.0, got {area}",);
    }
}
