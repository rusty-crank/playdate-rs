use std::env;
use std::path::PathBuf;
use std::process::Command;

mod bindgen_helper;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=bindgen_helper.rs");

    if env::var("TARGET").unwrap() == "thumbv7em-none-eabihf" {
        return;
    }

    if std::env::var("DOCS_RS").is_ok() {
        // Manually extract the sdk header files to the OUT_DIR
        let old_cwd = env::current_dir().unwrap();
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        env::set_current_dir(&out_path).unwrap();
        std::fs::create_dir_all("PlaydateSDK").unwrap();
        let status = Command::new("tar")
            .args([
                "xzf",
                &manifest_dir
                    .join("playdate-sdk-2.0.3.tar.gz")
                    .to_string_lossy(),
                "--strip",
                "1",
                "-C",
                "PlaydateSDK",
            ])
            .status()
            .unwrap();
        assert!(status.success());
        env::set_var("PLAYDATE_SDK_PATH", out_path.join("PlaydateSDK"));
        env::set_current_dir(old_cwd).unwrap();
    }

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindgen_helper::generate(false, out_path.join("bindings.rs"), None);
}
