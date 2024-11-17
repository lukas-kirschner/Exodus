use copy_to_output::copy_to_output;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

fn write_harvest_xslt(relative_source_folder: &str) -> PathBuf {
    let xslt_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("wix")
        .join(format!("{}.xslt", relative_source_folder));
    let dir_var = format!("{}Dir", relative_source_folder);
    let xslt = format!(
        r#"
<xsl:stylesheet version="1.0" 
xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
<xsl:output method="xml" version="1.0" encoding="UTF-8" indent="yes"/>
<xsl:strip-space elements="*"/>

<!-- identity transform -->
<xsl:template match="@*|node()">
    <xsl:copy>
        <xsl:apply-templates select="@*|node()"/>
    </xsl:copy>
</xsl:template>

<!-- match the root element of unknown name -->
<xsl:template match="/*">
    <xsl:copy>
        <xsl:processing-instruction name="define">
            <xsl:text>{0}="$(var.CargoTargetBinDir)\\{1}"</xsl:text>
        </xsl:processing-instruction>

        <xsl:apply-templates select="@*|node()"/>

        <xsl:processing-instruction name="undef">
            <xsl:text>{0}</xsl:text>
        </xsl:processing-instruction>
    </xsl:copy>
</xsl:template>

</xsl:stylesheet>
"#,
        dir_var, relative_source_folder
    );
    fs::write(&xslt_path, xslt).expect("Unable to write file");
    xslt_path
}

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
        // Copy assets folder to target folder
        copy_to_output("assets", &env::var("PROFILE").unwrap()).expect("Could not copy");
        let assets_xslt = write_harvest_xslt("assets");
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
            .arg("APPLICATIONFOLDER")
            .arg("-cg")
            .arg("AssetsComponent")
            .arg("-var")
            .arg("var.assetsDir")
            .arg("-t")
            .arg(assets_xslt)
            .arg("-out")
            // We write the harvested fragment into the source folder which is highly discouraged in Cargo build scripts.
            .arg(
               Path::new(
                    &std::env::var("CARGO_MANIFEST_DIR")
                        .expect("Could not get project dir from Cargo!"),
                ).join("wix")
                    .join("assets.wxs"),
            )
            .output()
            .expect("failed to execute process");
        if let Some(0) = out.status.code() {
        } else {
            println!("cargo:warning=Heat exited with code {:?}", out.status);
            panic!();
        }
        println!("cargo:rerun-if-changed=assets");
        println!("cargo:rerun-if-changed=build.rs");
    }
}
