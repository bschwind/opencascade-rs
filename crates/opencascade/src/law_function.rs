use cxx::UniquePtr;
use glam::dvec2;
use opencascade_sys::ffi;

use crate::primitives::make_point2d;

#[must_use]
pub(crate) fn law_function_from_graph(
    pairs: impl IntoIterator<Item = (f64, f64)>,
) -> UniquePtr<ffi::LawFunction> {
    let pairs: Vec<_> = pairs.into_iter().collect();
    let mut array = ffi::TColgpArray1OfPnt2d_ctor(1, pairs.len() as i32);

    for (index, (input, output)) in pairs.into_iter().enumerate() {
        array.pin_mut().set_value(index as i32 + 1, &make_point2d(dvec2(input, output)));
    }

    let mut interpol = ffi::LawInterpol_ctor();
    let is_periodic = false;
    interpol.pin_mut().set(&array, is_periodic);
    ffi::LawInterpol_into_LawFunction(interpol)
}
