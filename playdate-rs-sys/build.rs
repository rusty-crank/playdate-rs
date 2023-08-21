use std::env;
use std::path::{Path, PathBuf};

fn get_playdate_sdk_path() -> String {
    let is_correct_sdk_path = |path: &Path| path.join("bin").join("pdc").is_file();
    if cfg!(target_os = "macos") {
        let playdate_sdk_path = home::home_dir()
            .expect("Could not find home directory")
            .join("Developer")
            .join("PlaydateSDK");
        if is_correct_sdk_path(&playdate_sdk_path) {
            return playdate_sdk_path.to_str().unwrap().to_owned();
        }
    }
    let playdate_sdk_path =
        env::var("PLAYDATE_SDK_PATH").expect("Environment variable PLAYDATE_SDK_PATH is not set");
    if !is_correct_sdk_path(&PathBuf::from(&playdate_sdk_path)) {
        panic!("PLAYDATE_SDK_PATH is not set to the root of the Playdate SDK")
    }
    playdate_sdk_path
}

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    let playdate_sdk_path = get_playdate_sdk_path();

    let inc = |file: &str| format!("{}/C_API/{}", playdate_sdk_path, file);

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Header include path and macros
        .clang_arg(format!("-I{}/C_API", playdate_sdk_path))
        .clang_arg("-DTARGET_EXTENSION=1")
        .clang_arg("-DTARGET_SIMULATOR=1")
        // Include playdate headers only
        .allowlist_file(inc("pd_api/pd_api_.*\\.h$"))
        .allowlist_file(inc("pd_api.h"))
        // Rust enum types
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .derive_default(true)
        .derive_eq(true)
        .bitfield_enum("FileOptions")
        .bitfield_enum("SoundFormat")
        .bitfield_enum("PDButtons")
        .bitfield_enum("PDPeripherals")
        // no_std
        .use_core()
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
