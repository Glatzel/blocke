fn main() {
    // Always set linker script
    println!("cargo:rustc-link-arg=-Tlinkall.x");

    // Only enable frame pointers in dev builds
    if cfg!(debug_assertions) {
        println!("cargo:rustc-flag=-Cforce-frame-pointers");
    }
}
