pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/message.hxx");

        type Message_ProgressRange;
        #[cxx_name = "construct_unique"]
        pub fn Message_ProgressRange_new() -> UniquePtr<Message_ProgressRange>;
    }
}
