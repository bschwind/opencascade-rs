use opencascade::primitives::Shape;
use std::panic::{catch_unwind, AssertUnwindSafe};

/// OCCT reports failures with C++ exceptions. These used to cross the FFI boundary and
/// terminate the whole process; now a failed operation panics, which callers can catch.
#[test]
fn oversized_fillet_panics_instead_of_aborting() {
    let plate = Shape::box_with_dimensions(20.0, 20.0, 4.0);
    let result = catch_unwind(AssertUnwindSafe(|| plate.fillet(5.0)));
    assert!(result.is_err(), "a 5 mm fillet can't fit on a 4 mm plate");

    // The process is fine afterwards, and a fillet that fits still works.
    let rounded = Shape::box_with_dimensions(20.0, 20.0, 4.0).fillet(1.0);
    assert!(rounded.faces().count() > 6);
}

#[test]
fn oversized_chamfer_panics_instead_of_aborting() {
    let plate = Shape::box_with_dimensions(20.0, 20.0, 4.0);
    let result = catch_unwind(AssertUnwindSafe(|| plate.chamfer(5.0)));
    assert!(result.is_err(), "a 5 mm chamfer can't fit on a 4 mm plate");
}
