use std::process::Command;

fn main() {
    let mut git_hash: String = "<unknown hash>".to_string();
    if let Ok(output) = Command::new("git").args(&["rev-parse", "--short", "HEAD"]).output() {
        if let Ok(parsed_hash) = String::from_utf8(output.stdout) {
            git_hash = parsed_hash;
        }
    }
    println!("cargo:rustc-env=GIT_SHORTHASH={}", git_hash);

    let mut git_date: String = "<unknown date>".to_string();
    if let Ok(output) = Command::new("git").args(&["log", "-n", "1", "--format=%cs"]).output() {
        if let Ok(parsed_date) = String::from_utf8(output.stdout) {
            git_date = parsed_date;
        }
    }
    println!("cargo:rustc-env=GIT_SHORTDATE={}", git_date)
}