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
    use crate::workplane::Workplane;

    #[test]
    fn section_new() {
        let a = Workplane::xy().rect(1., 1.).to_face();
        let b = Workplane::yz().rect(1., 1.).to_face();

        let mut s = Section::new(&a, &b);
        s.build(None);
        let mut ba = ffi::cast(s.inner);
        assert_eq!(ba.pin_mut().SectionEdges().Size(), 1);
    }
}
