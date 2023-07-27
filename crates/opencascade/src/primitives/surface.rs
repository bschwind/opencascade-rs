use crate::primitives::make_point;
use cxx::UniquePtr;
use glam::DVec3;
use opencascade_sys::ffi;

pub struct Surface {
    pub(crate) inner: UniquePtr<ffi::HandleGeomSurface>,
}

impl Surface {
    pub fn bezier(poles: impl IntoIterator<Item = impl IntoIterator<Item = DVec3>>) -> Self {
        let poles: Vec<Vec<_>> =
            poles.into_iter().map(|poles| poles.into_iter().collect()).collect();

        let mut pole_array = ffi::TColgp_Array2OfPnt_ctor(
            0,
            poles.len() as i32 - 1,
            0,
            poles.first().map(|first| first.len()).unwrap_or(0) as i32 - 1,
        );

        for (row, poles) in poles.iter().enumerate() {
            for (column, pole) in poles.iter().enumerate() {
                let pole = &make_point(*pole);
                pole_array.pin_mut().SetValue(row as i32, column as i32, pole);
            }
        }

        let bezier = ffi::Geom_BezierSurface_ctor(&pole_array);
        let inner = ffi::bezier_to_surface(&bezier);

        Self { inner }
    }
}
