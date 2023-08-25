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
        // Manually download the SDK and extract it to the OUT_DIR
        let old_cwd = env::current_dir().unwrap();
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        env::set_current_dir(&out_path).unwrap();
        let status = Command::new("curl")
            .args([
                "-L",
                "https://download-keycdn.panic.com/playdate_sdk/Linux/PlaydateSDK-2.0.3.tar.gz",
                "-o",
                "PlaydateSDK.tar.gz",
            ])
            .status()
            .unwrap();
        assert!(status.success());
        std::fs::create_dir_all("PlaydateSDK").unwrap();
        let status = Command::new("tar")
            .args([
                "xzf",
                "PlaydateSDK.tar.gz",
                "--strip",
                "1",
                "-C",
                "PlaydateSDK",
            ])
            .status()
            .unwrap();
        assert!(status.success());
        env::set_var("PLAYDATE_SDK_PATH", out_path.join("PlaydateSDK"));
        env::set_current_dir(&old_cwd).unwrap();
    }

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindgen_helper::generate(false, out_path.join("bindings.rs"), None);
}
