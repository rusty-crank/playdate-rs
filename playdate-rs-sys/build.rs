use std::env;
use std::path::PathBuf;

mod bindgen_helper;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    if cfg!(not(all(target_arch = "arm", target_os = "none"))) {
        return;
    }

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindgen_helper::generate(false, out_path.join("bindings.rs"), None);
}
