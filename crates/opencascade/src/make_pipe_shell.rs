use cxx::UniquePtr;
use opencascade_sys::ffi;

#[must_use]
pub(crate) fn make_pipe_shell_with_law_function(
    profile: &ffi::TopoDS_Wire,
    spine: &ffi::TopoDS_Wire,
    law_function: &ffi::HandleLawFunction,
) -> UniquePtr<ffi::BRepOffsetAPI_MakePipeShell> {
    let mut make_pipe_shell = ffi::BRepOffsetAPI_MakePipeShell_ctor(spine);
    make_pipe_shell.pin_mut().SetMode(false);
    let profile_shape = ffi::cast_wire_to_shape(profile);
    let with_contact = false;
    let with_correction = true;
    make_pipe_shell.pin_mut().SetLaw(profile_shape, law_function, with_contact, with_correction);
    make_pipe_shell
}
