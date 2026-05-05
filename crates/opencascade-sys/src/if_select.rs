// Interface Select
pub use inner::*;

#[cxx::bridge]
mod inner {
    #[derive(Debug)]
    #[repr(u32)]
    pub enum IFSelect_ReturnStatus {
        IFSelect_RetVoid,
        IFSelect_RetDone,
        IFSelect_RetError,
        IFSelect_RetFail,
        IFSelect_RetStop,
    }

    unsafe extern "C++" {
        include!("opencascade-sys/include/if_select.hxx");

        type IFSelect_ReturnStatus;
    }
}
