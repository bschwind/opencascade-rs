// Binary Operation Algorithms
pub use inner::*;

#[cxx::bridge]
mod inner {
    #[derive(Debug)]
    #[repr(u32)]
    pub enum BOPAlgo_GlueEnum {
        BOPAlgo_GlueOff,
        BOPAlgo_GlueShift,
        BOPAlgo_GlueFull,
    }

    unsafe extern "C++" {
        include!("opencascade-sys/include/bop_algo.hxx");

        type BOPAlgo_GlueEnum;
    }
}
