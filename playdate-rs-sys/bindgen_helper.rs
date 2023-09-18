use bindgen::callbacks::EnumVariantValue;
use std::env;
use std::path::{Path, PathBuf};

pub fn get_playdate_sdk_path() -> String {
    let is_correct_sdk_path = |path: &Path| path.join("C_API").join("pd_api.h").is_file();
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
        panic!(
            "PLAYDATE_SDK_PATH ({}) is not set to the root of the Playdate SDK",
            playdate_sdk_path
        )
    }
    playdate_sdk_path
}

#[derive(Debug)]
struct EnumRenameParseCallbacks;

impl bindgen::callbacks::ParseCallbacks for EnumRenameParseCallbacks {
    fn enum_variant_name(
        &self,
        _enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: EnumVariantValue,
    ) -> Option<String> {
        let renames = [
            ("kASCIIEncoding", "ASCII"),
            ("kUTF8Encoding", "UTF8"),
            ("k16BitLEEncoding", "LE16Bit"),
            ("kInt", "Int"),
            ("kFloat", "Float"),
            ("kStr", "Str"),
        ];
        for (from, to) in &renames {
            if original_variant_name == *from {
                return Some((*to).to_owned());
            }
        }
        let prefixes = [
            "kDrawMode",
            "kBitmap",
            "kColor",
            "kLineCapStyle",
            "kLCDFontLanguage",
            "kPolygonFill",
            "kPDLanguage",
            "kType",
            "kJSON",
            "kCollisionType",
            "kLFOType",
            "kWaveform",
            "kFilterType",
            "kEvent",
        ];
        for prefix in &prefixes {
            if let Some(x) = original_variant_name.strip_prefix(prefix) {
                return Some(x.to_owned());
            }
        }
        return Some(original_variant_name.to_owned());
    }
}

pub fn generate(device: bool, out_dir: impl AsRef<Path>, arm_gcc_path: Option<&str>) {
    let playdate_sdk_path = get_playdate_sdk_path();

    let inc = |file: &str| format!("{}/C_API/{}", playdate_sdk_path, file);

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let mut builder = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Header include path and macros
        .clang_arg(format!("-I{}/C_API", playdate_sdk_path))
        .clang_arg("-DTARGET_EXTENSION=1");
    if device {
        builder = builder
            .clang_arg("-DTARGET_SIMULATOR=1")
            .clang_arg(format!("-I{}/include", arm_gcc_path.unwrap()));
    } else {
        builder = builder.clang_arg("-DTARGET_PLAYDATE=1");
    }
    let bindings = builder
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
        // Skip LCDColor
        .blocklist_type("LCDColor")
        .parse_callbacks(Box::new(EnumRenameParseCallbacks))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to file.
    bindings
        .write_to_file(out_dir)
        .expect("Couldn't write bindings!");
}
