//! Add the linker flag -Tdefmt.x

fn main() {
    println!("cargo:rustc-link-arg-examples=-Tdefmt.x");
}