fn main() {
    // tauri_build is not available — icons must be provided for production builds
    // For dev: use `npx tauri dev` or provide real icons
    println!("cargo:rerun-if-changed=build.rs");
}
