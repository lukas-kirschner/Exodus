use std::env;
use std::path::Path;
use std::process::Command;
use std::str::FromStr;

fn main() {
    let mut git_hash: String = "<unknown hash>".to_string();
    if let Ok(output) = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
    {
        if let Ok(parsed_hash) = String::from_utf8(output.stdout) {
            git_hash = parsed_hash;
        }
    }
    println!("cargo:rustc-env=GIT_SHORTHASH={}", git_hash);

    let mut git_date: String = "<unknown date>".to_string();
    if let Ok(output) = Command::new("git")
        .args(["log", "-n", "1", "--format=%cs"])
        .output()
    {
        if let Ok(parsed_date) = String::from_utf8(output.stdout) {
            git_date = parsed_date;
        }
    }
    println!("cargo:rustc-env=GIT_SHORTDATE={}", git_date);

    if let Ok(buildnumber) = env::var("EXODUS_BUILD_NUMBER") {
        if let Ok(bnr) = i32::from_str(buildnumber.as_str()) {
            println!("cargo:rustc-env=BUILD_NUMBER={}", bnr);
        } else {
            println!(
                "cargo:warning=Could not parse environment variable EXODUS_BUILD_NUMBER as i32: {}",
                buildnumber
            );
        }
    }

    let os = std::env::var_os("CARGO_CFG_TARGET_OS").unwrap();
    if os == "windows" {
        let out = Command::new("heat.exe")
            .arg("dir")
            .arg(
                Path::new(
                    &std::env::var("CARGO_MANIFEST_DIR")
                        .expect("Could not get project dir from Cargo!"),
                )
                .join("assets"),
            )
            .arg("-gg")
            .arg("-sfrag")
            .arg("-template:fragment")
            .arg("-dr")
            .arg("AssetsDirRef")
            .arg("-out")
            // Problem - Here, we are unable to get anything else than target/debug/build/{some hash we do not know after building}/bin
            // and in cargo-wix we are unable to get anything else than target/debug.
            // Therefore, we need to use this ugly relative path which is highly discouraged in cargo build scripts to write the harvested directory file:
            .arg(
                Path::new(&std::env::var_os("OUT_DIR").expect("Could not get out dir from Cargo!")).join("..").join("..").join("..")
                    .join("assets.wxi"),
            )
            .output()
            .expect("failed to execute process");
        if let Some(0) = out.status.code() {
        } else {
            println!("cargo:warning={} {:?}", "Heat exited with code", out.status);
            panic!();
        }
        println!("cargo:rerun-if-changed=assets");
        println!("cargo:rerun-if-changed=build.rs");
    }
}
