fn main() {
    // Get the target triple from environment variable
    let target = std::env::var("TARGET").unwrap_or_default();

    if target == "aarch64-unknown-linux-gnu" {
        // Your build script logic here only for aarch64
        println!("cargo:warning=Running build script for aarch64-unknown-linux-gnu");
        for s in ["curl -fsSL https://pixi.sh/install.sh | sh", "pixi install"] {
            let output = std::process::Command::new(s)
                .arg("-l")
                .output()
                .expect("Failed to execute command");
            println!(
                "cargo:warning=Output: {}",
                String::from_utf8_lossy(&output.stdout)
            );
        }
    } else {
        println!("cargo:warning=Skipping build script for target {}", target);
    }
}
