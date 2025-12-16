use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Get the output directory for the build
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Copy memory.ld to OUT_DIR as memory.x so link.x can find it
    fs::copy("memory.ld", out_dir.join("memory.x")).unwrap();

    // Tell Rust to include this linker script
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=memory.ld");
}