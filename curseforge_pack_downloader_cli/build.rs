use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Print a line to force rebuild when the version changes.
    println!("cargo:rerun-if-env-changed=CARGO_PKG_VERSION");

    // Print a line here to debug if it runs
    println!("Running build.rs...");

    // Existing logic for determining OS and renaming the executable
    let os = if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else {
        "unknown"
    };

    let version = env!("CARGO_PKG_VERSION");

    let output_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let target_dir = output_dir
        .join("../../../..")
        .canonicalize()
        .expect("Failed to canonicalize path");
    let exe_name = format!("curseforge_pack_downloader_cli-{}-{}", os, version);

    let original_exe = if os == "windows" {
        target_dir.join("release/curseforge_pack_downloader_cli.exe")
    } else {
        target_dir.join("release/curseforge_pack_downloader_cli")
    };
    let new_exe = if os == "windows" {
        target_dir.join(format!("release/{}.exe", exe_name))
    } else {
        target_dir.join(format!("release/{}", exe_name))
    };

    if let Err(e) = fs::rename(&original_exe, &new_exe) {
        eprintln!("Failed to rename executable: {}", e);
    }
}
