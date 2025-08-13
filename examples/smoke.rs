extern crate libnghttp3_sys as ffi;

fn main() {
    unsafe {
        ffi::nghttp3_version(0);

    }
}
