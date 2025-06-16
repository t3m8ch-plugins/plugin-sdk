use std::{env, fs, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=./wai/plugin.wai");

    let wai_content = include_str!("./wai/plugin.wai");
    let out_dir = env::var("OUT_DIR").unwrap();

    let dest_path = Path::new(&out_dir)
        .join("..")
        .join("..")
        .join("..")
        .join("..")
        .join("plugin.wai");

    fs::write(dest_path, wai_content).expect("Failed to write plugin.wai");
}
