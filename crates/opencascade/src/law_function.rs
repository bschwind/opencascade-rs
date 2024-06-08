use cxx::UniquePtr;
use glam::dvec2;
use opencascade_sys::ffi;

use crate::primitives::make_point2d;

pub struct LawFunction {
    pub(crate) inner: UniquePtr<ffi::Law_Function>,
}

impl LawFunction {
    pub fn from_function_graph(pairs: impl IntoIterator<Item = (f64, f64)>) -> Self {
        let pairs: Vec<_> = pairs.into_iter().collect();
        let mut array = ffi::TColgp_Array1OfPnt2d_ctor(1, pairs.len() as i32);

        for (index, (input, output)) in pairs.into_iter().enumerate() {
            array.pin_mut().SetValue(index as i32 + 1, &make_point2d(dvec2(input, output)));
        }

        let mut interpol = ffi::Law_Interpol_ctor();
        let is_periodic = false;
        interpol.pin_mut().Set(&array, is_periodic);
        Self { inner: ffi::Law_Interpol_into_Law_Function(interpol) }
    }
}
