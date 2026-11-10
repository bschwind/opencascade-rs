use crate::primitives::{make_point, SurfaceType};
use cxx::UniquePtr;
use glam::{dvec3, DVec3};
use opencascade_sys as ffi;

pub struct Surface {
    pub(crate) inner: UniquePtr<ffi::geom::Handle_Geom_Surface>,
}

impl Surface {
    pub fn bezier(poles: impl IntoIterator<Item = impl IntoIterator<Item = DVec3>>) -> Self {
        let poles: Vec<Vec<_>> =
            poles.into_iter().map(|poles| poles.into_iter().collect()).collect();

        let mut pole_array = ffi::t_col_gp::TColgp_Array2OfPnt_new(
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

        let bezier = ffi::geom::Geom_BezierSurface_new(&pole_array);
        let inner = ffi::geom::bezier_to_surface(&bezier);

        Self { inner }
    }

    pub fn surface_type(&self) -> SurfaceType {
        let dynamic_type = ffi::geom::DynamicType(&self.inner);
        let name = ffi::standard::type_name(dynamic_type);

        match name.as_str() {
            "Geom_Plane" => SurfaceType::Plane,
            "Geom_CylindricalSurface" => SurfaceType::Cylinder,
            "Geom_ConicalSurface" => SurfaceType::Cone,
            "Geom_SphericalSurface" => SurfaceType::Sphere,
            "Geom_ToroidalSurface" => SurfaceType::Torus,
            "Geom_BezierSurface" => SurfaceType::BezierSurface,
            "Geom_BSplineSurface" => SurfaceType::BSplineSurface,
            "Geom_SurfaceOfRevolution" => SurfaceType::SurfaceOfRevolution,
            "Geom_SurfaceOfExtrusion" => SurfaceType::SurfaceOfExtrusion,
            "Geom_OffsetSurface" => SurfaceType::OffsetSurface,
            _ => SurfaceType::OtherSurface,
        }
    }

    pub fn axis(&self) -> Option<(DVec3, DVec3)> {
        let surface_type = self.surface_type();
        match surface_type {
            SurfaceType::Cylinder => {
                let cyl =
                    ffi::geom::new_HandleGeomCylindricalSurface_from_HandleGeomSurface(&self.inner);
                if cyl.IsNull() {
                    return None;
                }
                let pos = ffi::geom::HandleGeomCylindricalSurface_Position(&cyl);
                Some((
                    dvec3(pos.Location().X(), pos.Location().Y(), pos.Location().Z()),
                    dvec3(pos.Direction().X(), pos.Direction().Y(), pos.Direction().Z()),
                ))
            },
            SurfaceType::Sphere => {
                let sphere =
                    ffi::geom::new_HandleGeomSphericalSurface_from_HandleGeomSurface(&self.inner);
                if sphere.IsNull() {
                    return None;
                }
                let pos = ffi::geom::HandleGeomSphericalSurface_Position(&sphere);
                Some((
                    dvec3(pos.Location().X(), pos.Location().Y(), pos.Location().Z()),
                    dvec3(pos.Direction().X(), pos.Direction().Y(), pos.Direction().Z()),
                ))
            },
            SurfaceType::Cone => {
                let cone =
                    ffi::geom::new_HandleGeomConicalSurface_from_HandleGeomSurface(&self.inner);
                if cone.IsNull() {
                    return None;
                }
                let pos = ffi::geom::HandleGeomConicalSurface_Position(&cone);
                Some((
                    dvec3(pos.Location().X(), pos.Location().Y(), pos.Location().Z()),
                    dvec3(pos.Direction().X(), pos.Direction().Y(), pos.Direction().Z()),
                ))
            },
            SurfaceType::Torus => {
                let torus =
                    ffi::geom::new_HandleGeomToroidalSurface_from_HandleGeomSurface(&self.inner);
                if torus.IsNull() {
                    return None;
                }
                let pos = ffi::geom::HandleGeomToroidalSurface_Position(&torus);
                Some((
                    dvec3(pos.Location().X(), pos.Location().Y(), pos.Location().Z()),
                    dvec3(pos.Direction().X(), pos.Direction().Y(), pos.Direction().Z()),
                ))
            },
            _ => None,
        }
    }

    pub fn radius(&self) -> Option<f64> {
        let surface_type = self.surface_type();
        match surface_type {
            SurfaceType::Cylinder => {
                let cyl =
                    ffi::geom::new_HandleGeomCylindricalSurface_from_HandleGeomSurface(&self.inner);
                if cyl.IsNull() {
                    return None;
                }
                Some(ffi::geom::HandleGeomCylindricalSurface_Radius(&cyl))
            },
            SurfaceType::Sphere => {
                let sphere =
                    ffi::geom::new_HandleGeomSphericalSurface_from_HandleGeomSurface(&self.inner);
                if sphere.IsNull() {
                    return None;
                }
                Some(ffi::geom::HandleGeomSphericalSurface_Radius(&sphere))
            },
            SurfaceType::Cone => {
                let cone =
                    ffi::geom::new_HandleGeomConicalSurface_from_HandleGeomSurface(&self.inner);
                if cone.IsNull() {
                    return None;
                }
                Some(ffi::geom::HandleGeomConicalSurface_RefRadius(&cone))
            },
            SurfaceType::Torus => {
                let torus =
                    ffi::geom::new_HandleGeomToroidalSurface_from_HandleGeomSurface(&self.inner);
                if torus.IsNull() {
                    return None;
                }
                Some(ffi::geom::HandleGeomToroidalSurface_MajorRadius(&torus))
            },
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workplane::Workplane;

    #[test]
    fn test_surface_type_plane() {
        let face = Workplane::xy().rect(7.0, 5.0).to_face();
        let handle = ffi::b_rep::BRep_Tool_Surface(&face.inner);
        let surface = Surface { inner: handle };
        assert_eq!(surface.surface_type(), SurfaceType::Plane);
    }
}
