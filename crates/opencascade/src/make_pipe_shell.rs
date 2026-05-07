use cxx::UniquePtr;
use opencascade_sys as ffi;

#[must_use]
pub(crate) fn make_pipe_shell_with_law_function(
    profile: &ffi::topo_ds::TopoDS_Wire,
    spine: &ffi::topo_ds::TopoDS_Wire,
    law_function: &ffi::law::Handle_Law_Function,
) -> UniquePtr<ffi::b_rep_offset_api::BRepOffsetAPI_MakePipeShell> {
    let mut make_pipe_shell = ffi::b_rep_offset_api::BRepOffsetAPI_MakePipeShell_new(spine);
    make_pipe_shell.pin_mut().SetMode(false);
    let profile_shape = ffi::topo_ds::cast_wire_to_shape(profile);
    let with_contact = false;
    let with_correction = true;
    make_pipe_shell.pin_mut().SetLaw(profile_shape, law_function, with_contact, with_correction);
    make_pipe_shell
}
