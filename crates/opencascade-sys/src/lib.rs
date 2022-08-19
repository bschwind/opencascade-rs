#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        // include!("cxx-demo/include/blobstore.h");

        type BlobstoreClient;

        pub fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
    }
}
