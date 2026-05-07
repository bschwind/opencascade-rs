pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/law.hxx");

        type TColgp_Array1OfPnt2d = crate::t_col_gp::TColgp_Array1OfPnt2d;

        // Handles
        type Handle_Law_Function;
        pub fn IsNull(self: &Handle_Law_Function) -> bool;
        // End Handles

        type Law_Function;
        pub fn Law_Function_to_handle(
            law: UniquePtr<Law_Function>,
        ) -> UniquePtr<Handle_Law_Function>;

        type Law_Interpol;
        #[cxx_name = "construct_unique"]
        pub fn Law_Interpol_new() -> UniquePtr<Law_Interpol>;
        pub fn Law_Interpol_into_Law_Function(
            interpol: UniquePtr<Law_Interpol>,
        ) -> UniquePtr<Law_Function>;
        pub fn Set(self: Pin<&mut Law_Interpol>, array: &TColgp_Array1OfPnt2d, periodic: bool);
    }

    impl UniquePtr<Handle_Law_Function> {}
}
