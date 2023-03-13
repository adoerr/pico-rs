use std::{env, fs::File, io::Write, path::PathBuf};

use anyhow::Result;

fn main() -> Result<()> {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))?.write_all(include_bytes!("memory.x"))?;

    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");

    Ok(())
}
