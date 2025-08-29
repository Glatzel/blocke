fn main() {
    // Only enable frame pointers in dev builds
    if cfg!(debug_assertions) {
        println!("cargo:rustc-flag=-Cforce-frame-pointers");
    }
}
