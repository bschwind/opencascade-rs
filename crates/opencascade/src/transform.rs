use cxx::UniquePtr;
use glam::DMat4;
use opencascade_sys::ffi;

pub fn gp_trsf(mat: &DMat4) -> UniquePtr<ffi::gp_Trsf> {
    let mut t = ffi::new_transform();
    t.pin_mut().SetValues(
        mat.x_axis.x,
        mat.y_axis.x,
        mat.z_axis.x,
        mat.w_axis.x,
        mat.x_axis.y,
        mat.y_axis.y,
        mat.z_axis.y,
        mat.w_axis.y,
        mat.x_axis.z,
        mat.y_axis.z,
        mat.z_axis.z,
        mat.w_axis.z,
    );
    t
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn set_gp_trsf_values() {
        let m = glam::dmat4(
            glam::dvec4(0.0, 0.0, 1.0, 0.0),
            glam::dvec4(1.0, 0.0, 0.0, 0.0),
            glam::dvec4(0.0, 1.0, 0.0, 0.0),
            glam::dvec4(0.0, 0.0, 0.0, 1.0),
        );

        let m = glam::DMat4::IDENTITY;

        let t = gp_trsf(&m);

        assert_eq!(t.Value(0, 0), 0.0);
    }
}
