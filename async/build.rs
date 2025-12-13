use std::{env, fs::File, io::Write, path::PathBuf};

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR env var not set"));

    File::create(out.join("../memory.x"))
        .expect("failed to create memory.x")
        .write_all(include_bytes!("memory.x"))
        .expect("failed to write memory.x");
    println!("cargo:rustc-link-search={}", out.display());

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    println!("cargo:rerun-if-changed=memory.x");

    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tlink-rp.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
