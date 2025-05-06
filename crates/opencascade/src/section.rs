use crate::primitives::Face;
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
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::primitives::Edge;
    use crate::workplane::Workplane;
    use glam::dvec3;

    #[test]
    fn section_new() {
        let a = Workplane::xy().rect(1., 1.).to_face();
        let b = Workplane::yz().rect(1., 1.).to_face();

        let mut s = Section::new(&a, &b);
        s.build(None);
        let mut ba = ffi::cast(s.inner);
        assert_eq!(ba.pin_mut().SectionEdges().Size(), 1);

        let v = ffi::shape_list_to_vector(ba.pin_mut().SectionEdges());
        let itm = v.get(0).unwrap();
        assert_eq!(itm.ShapeType(), ffi::TopAbs_ShapeEnum::TopAbs_EDGE);
        let edge = Edge::from_edge(ffi::TopoDS_cast_to_edge(itm));
        assert_eq!(edge.start_point(), dvec3(0., -0.5, 0.));
        assert_eq!(edge.end_point(), dvec3(0., 0.5, 0.));
    }
}
