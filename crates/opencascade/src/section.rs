use crate::primitives::Shape;
use cxx::UniquePtr;
use opencascade_sys::ffi;

/// A wrapper around the `BRepAlgoAPI_Section` class.
pub struct Section {
    pub(crate) inner: UniquePtr<ffi::BRepAlgoAPI_Section>,
}
impl Section {
    /// Create a new `Section` to intersect `target` by `tool`.
    pub fn new(target: &Shape, tool: &Shape) -> Section {
        Section {
            inner: ffi::BRepAlgoAPI_Section_ctor(
                target.inner.as_ref().expect("Target was null"),
                tool.inner.as_ref().expect("Tool was null"),
            ),
        }
    }

    /// Get the edges of the resulting intersection.
    pub fn section_edges(self) -> Vec<Shape> {
        // TODO: Given that the OCCT name is "SectionEdges", can we return a Vec<Edge>?

        let mut ba = ffi::cast_section_to_builderalgo(self.inner);
        let edges = ffi::shape_list_to_vector(ba.pin_mut().SectionEdges());

        let mut vec = vec![];

        for e in edges.iter() {
            vec.push(Shape::from_shape(e));
        }

        vec
    }
}

/// Creates a `Section` from two shapes, performs the intersection, and returns the resulting edges.
pub fn edges(target: &Shape, tool: &Shape) -> Vec<Shape> {
    let section = Section::new(target, tool);
    section.section_edges()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        primitives::{IntoShape, ShapeType},
        workplane::Workplane,
    };
    use glam::dvec3;

    #[test]
    fn section_new() {
        let a = Workplane::xy().rect(1., 1.).to_face();
        let b = Workplane::yz().rect(1., 1.).to_face();

        let s = Section::new(&a.into_shape(), &b.into_shape());

        let edges = s.section_edges();
        assert_eq!(edges.len(), 1);

        let itm = edges.get(0);

        assert!(itm.is_some());

        let s = itm.unwrap();

        assert_eq!(s.shape_type(), ShapeType::Edge);

        for e in s.edges() {
            // There should be only one edge
            assert_eq!(e.start_point(), dvec3(0., -0.5, 0.));
            assert_eq!(e.end_point(), dvec3(0., 0.5, 0.));
        }
    }
}
