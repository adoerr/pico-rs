//! Add the linker flag -Tdefmt.x is added

fn main() {
    println!("cargo:rustc-link-arg-examples=-Tdefmt.x");
}