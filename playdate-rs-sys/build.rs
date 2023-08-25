use std::env;
use std::path::PathBuf;

mod bindgen_helper;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=bindgen_helper.rs");

    if env::var("TARGET").unwrap() == "thumbv7em-none-eabihf" || cfg!(doc) {
        return;
    }

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindgen_helper::generate(false, out_path.join("bindings.rs"), None);
}
