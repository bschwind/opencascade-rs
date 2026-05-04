use cxx::UniquePtr;
use opencascade_sys::ffi;

#[must_use]
pub(crate) fn make_pipe_shell_with_law_function(
    profile: &opencascade_sys::topo_ds::TopoDS_Wire,
    spine: &opencascade_sys::topo_ds::TopoDS_Wire,
    law_function: &ffi::Handle_Law_Function,
) -> UniquePtr<opencascade_sys::b_rep_offset_api::BRepOffsetAPI_MakePipeShell> {
    let mut make_pipe_shell =
        opencascade_sys::b_rep_offset_api::BRepOffsetAPI_MakePipeShell_ctor(spine);
    make_pipe_shell.pin_mut().SetMode(false);
    let profile_shape = opencascade_sys::topo_ds::cast_wire_to_shape(profile);
    let with_contact = false;
    let with_correction = true;
    make_pipe_shell.pin_mut().SetLaw(profile_shape, law_function, with_contact, with_correction);
    make_pipe_shell
}
