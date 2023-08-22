use std::env;
use std::path::PathBuf;

#[path = "../../bindgen_helper.rs"]
mod bindgen_helper;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let arm_gcc_path = if args.len() > 1 {
        args[1].clone()
    } else {
        panic!("ERROR: arm-none-eabi-gcc installation path not specified!")
    };

    if !PathBuf::from(&arm_gcc_path).join("include").is_dir() {
        panic!("ERROR: arm-none-eabi-gcc installation path is not valid!");
    }

    let out_path = PathBuf::from("src").join("thumbv7em_bindings.rs");
    dbg!(out_path.clone());
    bindgen_helper::generate(true, out_path, Some(&arm_gcc_path));
}
