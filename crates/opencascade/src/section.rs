use crate::primitives::{Face, Shape};
use cxx::UniquePtr;
use opencascade_sys::ffi;

pub struct Section {
    pub(crate) inner: UniquePtr<ffi::BRepAlgoAPI_Section>,
}
impl Section {
    pub fn new(s1: &Face, s2: &Face) -> Section {
        Section {
            inner: ffi::BRepAlgoAPI_Section_ctor(
                ffi::cast_face_to_shape(s1.inner.as_ref().expect("s1 was null")),
                ffi::cast_face_to_shape(s2.inner.as_ref().expect("s2 was null")),
            ),
        }
    }

    pub fn build(&mut self, msg_prog: Option<&ffi::Message_ProgressRange>) {
        let mpr =
            if msg_prog.is_some() { msg_prog.unwrap() } else { &ffi::Message_ProgressRange_ctor() };

        self.inner.pin_mut().Build(mpr);
    }

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

#[cfg(test)]
mod test {
    use super::*;
    use crate::{primitives::ShapeType, workplane::Workplane};
    use glam::dvec3;

    #[test]
    fn section_new() {
        let a = Workplane::xy().rect(1., 1.).to_face();
        let b = Workplane::yz().rect(1., 1.).to_face();

        let mut s = Section::new(&a, &b);
        s.build(None);

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
