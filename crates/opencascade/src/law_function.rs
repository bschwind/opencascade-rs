use crate::primitives::make_point2d;
use cxx::UniquePtr;
use glam::dvec2;

#[must_use]
pub(crate) fn law_function_from_graph(
    pairs: impl IntoIterator<Item = (f64, f64)>,
) -> UniquePtr<opencascade_sys::law::Law_Function> {
    let pairs: Vec<_> = pairs.into_iter().collect();
    let mut array = opencascade_sys::t_col_gp::TColgp_Array1OfPnt2d_ctor(1, pairs.len() as i32);

    for (index, (input, output)) in pairs.into_iter().enumerate() {
        array.pin_mut().SetValue(index as i32 + 1, &make_point2d(dvec2(input, output)));
    }

    let mut interpol = opencascade_sys::law::Law_Interpol_ctor();
    let is_periodic = false;
    interpol.pin_mut().Set(&array, is_periodic);
    opencascade_sys::law::Law_Interpol_into_Law_Function(interpol)
}
